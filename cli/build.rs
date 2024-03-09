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

use std::fs::{copy, create_dir};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tmp_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp"));

    // BASE

    if !tmp_path.exists() {
        create_dir(tmp_path)?;
    }

    copy(
        manifest_path.join("Cargo.toml"),
        tmp_path.join("Freight.toml"),
    )?;

    // CORE

    let tmp_core_path = tmp_path.join("core");

    if !tmp_core_path.exists() {
        create_dir(tmp_core_path)?;
    }

    copy(
        manifest_path.join("core/build.rs"),
        tmp_path.join("core/build.rs"),
    )?;

    copy(
        manifest_path.join("core/Cargo.toml"),
        tmp_path.join("core/Freight.toml"),
    )?;

    copy(
        manifest_path.join("core/bicycle.proto"),
        tmp_path.join("core/bicycle.proto"),
    )?;

    let tmp_core_src_path = tmp_path.join("core/src");

    if !tmp_core_src_path.exists() {
        create_dir(tmp_core_src_path)?;
    }

    copy(
        manifest_path.join("core/src/lib.rs"),
        tmp_path.join("core/src/lib.rs"),
    )?;

    let tmp_core_src_models_path = tmp_path.join("core/src/models");

    if !tmp_core_src_models_path.exists() {
        create_dir(tmp_core_src_models_path)?;
    }

    copy(
        manifest_path.join("core/src/models/example.rs"),
        tmp_path.join("core/src/models/example.rs"),
    )?;

    // ENGINES

    let tmp_engines_path = tmp_path.join("engines");

    if !tmp_engines_path.exists() {
        create_dir(tmp_engines_path)?;
    }

    let tmp_engines_rocksdb_path = tmp_path.join("engines/rocksdb");

    if !tmp_engines_rocksdb_path.exists() {
        create_dir(tmp_engines_rocksdb_path)?;
    }

    copy(
        manifest_path.join("engines/rocksdb/Cargo.toml"),
        tmp_path.join("engines/rocksdb/Freight.toml"),
    )?;

    let tmp_engines_rocksdb_src_path = tmp_path.join("engines/rocksdb/src");

    if !tmp_engines_rocksdb_src_path.exists() {
        create_dir(tmp_engines_rocksdb_src_path)?;
    }

    copy(
        manifest_path.join("engines/rocksdb/src/lib.rs"),
        tmp_path.join("engines/rocksdb/src/lib.rs"),
    )?;

    // SERVER

    let tmp_server_path = tmp_path.join("server");

    if !tmp_server_path.exists() {
        create_dir(tmp_server_path)?;
    }

    copy(
        manifest_path.join("server/Cargo.toml"),
        tmp_path.join("server/Freight.toml"),
    )?;

    let tmp_server_src_path = tmp_path.join("server/src");

    if !tmp_server_src_path.exists() {
        create_dir(tmp_server_src_path)?;
    }

    copy(
        manifest_path.join("server/src/main.rs"),
        tmp_path.join("server/src/main.rs"),
    )?;

    // SPROC

    let tmp_sproc_path = tmp_path.join("sproc");

    if !tmp_sproc_path.exists() {
        create_dir(tmp_sproc_path)?;
    }

    copy(
        manifest_path.join("sproc/build.rs"),
        tmp_path.join("sproc/build.rs"),
    )?;

    copy(
        manifest_path.join("sproc/Cargo.toml"),
        tmp_path.join("sproc/Freight.toml"),
    )?;

    copy(
        manifest_path.join("sproc/sproc.proto"),
        tmp_path.join("sproc/sproc.proto"),
    )?;

    let tmp_sproc_wasm_src_path = tmp_path.join("sproc/src");

    if !tmp_sproc_wasm_src_path.exists() {
        create_dir(tmp_sproc_wasm_src_path)?;
    }

    copy(
        manifest_path.join("sproc/src/lib.rs"),
        tmp_path.join("sproc/src/lib.rs"),
    )?;

    Ok(())
}
