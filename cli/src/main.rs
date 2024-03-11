/*
Bicycle is a protobuf defined database framework.

Copyright (C) 2024 Ordinary Labs

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use clap::{arg, command, value_parser, Command};
use std::env;

use bicycle_sproc::proto::{sproc_client::SprocClient, OneOff, Proc, Stored};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new("bicycle")
        .bin_name("bicycle")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("build")
            .arg_required_else_help(true)
                .about("builds server, client, shims and proto definitions.")
                .arg(
                    arg!(<SCHEMA_PATH> "path to the schema.proto file")
                        .value_parser(value_parser!(String)).required(true),
                )
                .arg(
                    arg!(--"engine" <ENGINE> "specifies database engine.")
                        .value_parser(["rocksdb"])
                        .default_value("rocksdb"),
                ),
        )
        .subcommand(
            command!("sproc")
                .arg_required_else_help(true)
                .about("commands for interacting with the stored procedure API.\n'--lang rust' depends on `cargo-wasi`")
                .subcommand_required(true)
                .subcommand(
                    command!("deploy")
                        .arg_required_else_help(true)
                        .about("deploys a stored procedure.")
                        .arg(
                            arg!(<LIB_PATH> "relative path to the lib directory.")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"name" <NAME> "name for the stored procedure.")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"lang" <LANGUAGE> "language to be compiled to WebAssembly.")
                                .value_parser(["rust"]).required(true),
                        )
                )
                .subcommand(
                    command!("exec")
                        .arg_required_else_help(true)
                        .about("runs a stored procedure.")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"name" <NAME> "name of stored procedure.")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"args" <ARGUMENTS> "arguments to be parsed into the protobuf Value WKT\nformatted as JSON")
                                .value_parser(value_parser!(String)),
                        )
                )
                .subcommand(
                    command!("oneoff")
                        .arg_required_else_help(true)
                        .about("sends up a one-off procedure.")
                        .arg(
                            arg!(<LIB_PATH> "relative path to the lib directory.")
                                .value_parser(value_parser!(String)).required(true),
                        )
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)),
                        )
                        .arg(
                            arg!(--"lang" <LANGUAGE> "language to be compiled to WebAssembly")
                                .value_parser(["rust"]),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"args" <ARGUMENTS> "arguments to be parsed into the protobuf Value WKT")
                                .value_parser(value_parser!(String)),
                        )
                )
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("build", matches)) => {
            let schema_path = matches.get_one::<String>("SCHEMA_PATH").expect("required");

            let engine = matches
                .get_one::<String>("engine")
                .expect("default value provided");

            bicycle::build(schema_path, engine)?;
        }
        Some(("sproc", matches)) => match matches.subcommand() {
            Some(("deploy", matches)) => {
                let lib_path = matches.get_one::<String>("LIB_PATH").expect("required");

                let name = matches
                    .get_one::<String>("name")
                    .expect("required")
                    .to_string();
                let lang = matches
                    .get_one::<String>("lang")
                    .expect("required")
                    .to_string();
                let addr = matches
                    .get_one::<String>("addr")
                    .expect("required")
                    .to_string();

                match lang.as_str() {
                    "rust" => {
                        std::env::set_current_dir(lib_path)?;

                        println!("🦀 targeting WebAssembly...");
                        std::process::Command::new("cargo")
                            .args(["wasi", "build", "--release"])
                            .output()?;

                        let proc_bytes = std::fs::read("target/wasm32-wasi/release/proc.wasm")?;

                        println!("🕸️  compiled to WebAssembly.");

                        println!("📦 deploying procedure...");
                        let mut client = SprocClient::connect(addr).await?;

                        let request = tonic::Request::new(Proc {
                            name: name.to_string(),
                            proc: proc_bytes,
                        });

                        client.deploy(request).await?;

                        println!("📦 procedure deployed.");
                    }
                    _ => unreachable!(),
                }
            }
            Some(("exec", matches)) => {
                let name = matches
                    .get_one::<String>("name")
                    .expect("required")
                    .to_string();
                let addr = matches
                    .get_one::<String>("addr")
                    .expect("required")
                    .to_string();

                let args = match matches.get_one::<String>("args") {
                    Some(args) => {
                        println!("📦 encoding proc args...");
                        let json_value = serde_json::from_str(args)?;
                        println!("📦 proc args encoded.");

                        json_to_proto(json_value)
                    }
                    None => prost_types::Value { kind: None },
                };

                println!("🚀 executing procedure...");
                let mut client = SprocClient::connect(addr).await?;

                let request = tonic::Request::new(Stored {
                    name: name.to_string(),
                    args: Some(args),
                });

                let now = std::time::Instant::now();
                let response = client.exec_stored(request).await?;
                println!("✅ done!\n⏱️  round trip in {}ms", now.elapsed().as_millis());

                let response_as_json = proto_to_json(response.into_inner());
                println!(
                    "\nresponse:\n\n{}",
                    serde_json::to_string_pretty(&response_as_json).unwrap()
                );
            }
            Some(("oneoff", matches)) => {
                let lib_path = matches.get_one::<String>("LIB_PATH").expect("required");

                let lang = matches
                    .get_one::<String>("lang")
                    .expect("required")
                    .to_string();
                let addr = matches
                    .get_one::<String>("addr")
                    .expect("required")
                    .to_string();

                let args = match matches.get_one::<String>("args") {
                    Some(args) => {
                        println!("📦 encoding proc args...");
                        let json_value = serde_json::from_str(args)?;
                        println!("📦 proc args encoded.");

                        json_to_proto(json_value)
                    }
                    None => prost_types::Value { kind: None },
                };

                match lang.as_str() {
                    "rust" => {
                        std::env::set_current_dir(lib_path)?;

                        println!("🦀 targeting WebAssembly...");
                        std::process::Command::new("cargo")
                            .args(["wasi", "build", "--release"])
                            .output()?;

                        let proc_bytes = std::fs::read("target/wasm32-wasi/release/proc.wasm")?;

                        println!("🕸️  compiled to WebAssembly.");

                        println!("🚀 executing procedure...");
                        let mut client = SprocClient::connect(addr).await?;

                        let request = tonic::Request::new(OneOff {
                            proc: proc_bytes,
                            args: Some(args),
                        });

                        let now = std::time::Instant::now();
                        let response = client.exec_one_off(request).await?;
                        println!("✅ done!\n⏱️  round trip in {}ms", now.elapsed().as_millis());

                        let response_as_json = proto_to_json(response.into_inner());
                        println!(
                            "\nresponse:\n\n{}",
                            serde_json::to_string_pretty(&response_as_json).unwrap()
                        );
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}

fn json_map_to_proto_struct(
    json: serde_json::Map<String, serde_json::Value>,
) -> prost_types::Struct {
    prost_types::Struct {
        fields: json
            .into_iter()
            .map(|(k, v)| (k, json_to_proto(v)))
            .collect(),
    }
}

pub fn json_to_proto(json: serde_json::Value) -> prost_types::Value {
    let kind = match json {
        serde_json::Value::Null => prost_types::value::Kind::NullValue(0),
        serde_json::Value::Bool(v) => prost_types::value::Kind::BoolValue(v),
        serde_json::Value::Number(n) => match n.as_f64() {
            Some(n) => prost_types::value::Kind::NumberValue(n),
            None => prost_types::value::Kind::NullValue(0),
        },
        serde_json::Value::String(s) => prost_types::value::Kind::StringValue(s),
        serde_json::Value::Array(v) => {
            prost_types::value::Kind::ListValue(prost_types::ListValue {
                values: v.into_iter().map(json_to_proto).collect(),
            })
        }
        serde_json::Value::Object(v) => {
            prost_types::value::Kind::StructValue(json_map_to_proto_struct(v))
        }
    };

    prost_types::Value { kind: Some(kind) }
}

pub fn proto_to_json(proto: prost_types::Value) -> serde_json::Value {
    if let Some(kind) = proto.kind {
        match kind {
            prost_types::value::Kind::NullValue(_) => serde_json::Value::Null,
            prost_types::value::Kind::BoolValue(v) => serde_json::Value::Bool(v),
            prost_types::value::Kind::NumberValue(n) => match serde_json::Number::from_f64(n) {
                Some(n) => serde_json::Value::Number(n),
                None => serde_json::Value::Null,
            },
            prost_types::value::Kind::StringValue(s) => serde_json::Value::String(s),
            prost_types::value::Kind::ListValue(lst) => {
                serde_json::Value::Array(lst.values.into_iter().map(proto_to_json).collect())
            }
            prost_types::value::Kind::StructValue(v) => serde_json::Value::Object(
                v.fields
                    .into_iter()
                    .map(|(k, v)| (k, proto_to_json(v)))
                    .collect(),
            ),
        }
    } else {
        serde_json::Value::Null
    }
}
