/*
Bicycle is a framework for managing data.

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
use prost_types::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new("bicycle")
        .bin_name("bicycle")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("create")
                .about("creates a new server binary and client proto definition.")
                .arg(
                    arg!(<SCHEMA_PATH> "path to the schema.proto file")
                        .value_parser(value_parser!(String)),
                )
                .arg_required_else_help(true)
                .arg(
                    arg!(--"engine" <ENGINE> "specifies database engine.")
                        .value_parser(["rocksdb"])
                        .default_value("rocksdb"),
                ),
        )
        .subcommand(
            command!("sproc")
                .about("commands for interacting with the stored procedure API.")
                .subcommand_required(true)
                .subcommand(
                    command!("deploy")
                        .about("deploys a stored procedure.")
                        .arg(
                            arg!(<LIB_PATH> "relative path to the lib directory.")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"name" <NAME> "name for the stored procedure.")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"lang" <LANGUAGE> "language to be compiled to WebAssembly")
                                .value_parser(["rust"]),
                        )
                        .arg_required_else_help(true)
                )
                .subcommand(
                    command!("exec")
                        .about("runs a stored procedure.")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"name" <NAME> "name of stored procedure.")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                )
                .subcommand(
                    command!("oneoff")
                        .about("runs a one off stored procedure.")
                        .arg(
                            arg!(<LIB_PATH> "relative path to the lib directory.")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"addr" <ADDRESS> "address of the database (i.e http://0.0.0.0::50051)")
                                .value_parser(value_parser!(String)),
                        )
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--"lang" <LANGUAGE> "language to be compiled to WebAssembly")
                                .value_parser(["rust"]),
                        )
                        .arg_required_else_help(true)
                )
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("create", matches)) => {
            let schema_path = matches.get_one::<String>("SCHEMA_PATH").expect("required");

            let engine = matches
                .get_one::<String>("engine")
                .expect("default value provided");

            bicycle::create(schema_path, engine)?;
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

                        println!("🦀 -> 🕸️  building WebAssembly...");
                        std::process::Command::new("cargo")
                            .args(["wasi", "build", "--release"])
                            .env("RUSTFLAGS", "-Ctarget-feature=+multivalue")
                            .output()?;

                        let proc_bytes = std::fs::read("target/wasm32-wasi/release/proc.wasm")?;

                        println!("🕸️  WebAssembly build done.");

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

                println!("🚀 running procedure...");
                let mut client = SprocClient::connect(addr).await?;

                let request = tonic::Request::new(Stored {
                    name: name.to_string(),
                    // TODO: accept args
                    args: Some(Value { kind: None }),
                });

                let response = client.exec_stored(request).await?;

                println!("{:#?}", response);
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

                match lang.as_str() {
                    "rust" => {
                        std::env::set_current_dir(lib_path)?;

                        println!("🦀 -> 🕸️  building WebAssembly...");
                        std::process::Command::new("cargo")
                            .args(["wasi", "build", "--release"])
                            .env("RUSTFLAGS", "-Ctarget-feature=+multivalue")
                            .output()?;

                        let proc_bytes = std::fs::read("target/wasm32-wasi/release/proc.wasm")?;

                        println!("🕸️  WebAssembly build done.");

                        println!("🚀 running procedure...");
                        let mut client = SprocClient::connect(addr).await?;

                        let request = tonic::Request::new(OneOff {
                            proc: proc_bytes,
                            // TODO: accept args
                            args: Some(Value {
                                kind: Some(prost_types::value::Kind::BoolValue(true)),
                            }),
                        });

                        let response = client.exec_one_off(request).await?;

                        println!("{:#?}", response);
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
