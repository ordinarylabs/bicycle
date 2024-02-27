/*
Bicycle is a database database framework.

Copyright (C) 2024  Ordinary Labs, LLC

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

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Not enough arguments");
    }

    let command = &args[1];
    let schema_path = &args[2];

    let mut plugins: Vec<String> = vec![];

    if args.len() == 5 && args[3].to_string() == "--plugins".to_string() {
        let plugins_str = &args[4];
        let plugins_string = plugins_str.to_string();

        for plugin in plugins_string.split(',').into_iter() {
            plugins.push(plugin.to_string());
        }
    }

    match command.as_str() {
        "create" => bicycle::build_with_plugins(schema_path, plugins),
        _ => panic!("invalid command"),
    }
}
