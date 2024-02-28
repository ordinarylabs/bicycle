/*
Bicycle is a database database framework.

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

#![doc = include_str!("../../README.md")]

pub(crate) const PRECOMPILE_DIR: &'static str = "./__precompile__";

mod create;
use create::create_with_plugins;

pub(crate) mod gen;
pub(crate) mod utils;

/// builds a Bicycle database server binary.
/// outputs to `out/` directory.
///
/// NOTE: takes awhile for RocksDB to build.
///
/// * `schema_path` - path to the schema.proto file
pub fn build(schema_path: &str) {
    create_with_plugins(schema_path, vec![])
}

/// builds a Bicycle database server binary with plugins.
/// outputs to `out/` directory.
///
/// NOTE: takes awhile for RocksDB to build.
///
/// * `schema_path` - path to the schema.proto file
/// * `plugin_names`
pub fn build_with_plugins(schema_path: &str, plugin_names: Vec<String>) {
    create_with_plugins(schema_path, plugin_names)
}
