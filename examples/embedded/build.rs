/*
BicycleDB is a protobuf-defined database management system.

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

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let schema_path = concat!(env!("CARGO_MANIFEST_DIR"), "/schema.proto");
    bicycle::build(schema_path, "sqlite")
}
