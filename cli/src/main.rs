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

fn main() {
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
                )
                .arg(
                    arg!(--"runtime" <RUNTIME> "specifies set of runtimes to include.")
                        .value_parser(["javascript"]),
                ),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("create", matches)) => {
            let schema_path = matches.get_one::<String>("SCHEMA_PATH").expect("required");

            let engine = matches
                .get_one::<String>("engine")
                .expect("default value provided");

            let mut runtimes = vec![];

            if let Some(rts) = matches.get_many::<String>("runtime") {
                for rt in rts {
                    runtimes.push(rt.clone());
                }
            }

            bicycle::create(schema_path, engine, runtimes)
        }
        _ => unreachable!(),
    }
}
