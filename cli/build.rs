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
        manifest_path.join("core/Cargo.toml"),
        tmp_path.join("core/Freight.toml"),
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

    // RocksDB
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

    // SQLite
    let tmp_engines_sqlite_path = tmp_path.join("engines/sqlite");

    if !tmp_engines_sqlite_path.exists() {
        create_dir(tmp_engines_sqlite_path)?;
    }

    copy(
        manifest_path.join("engines/sqlite/Cargo.toml"),
        tmp_path.join("engines/sqlite/Freight.toml"),
    )?;

    let tmp_engines_sqlite_src_path = tmp_path.join("engines/sqlite/src");

    if !tmp_engines_sqlite_src_path.exists() {
        create_dir(tmp_engines_sqlite_src_path)?;
    }

    copy(
        manifest_path.join("engines/sqlite/src/lib.rs"),
        tmp_path.join("engines/sqlite/src/lib.rs"),
    )?;

    // PROTO

    let tmp_proto_path = tmp_path.join("proto");

    if !tmp_proto_path.exists() {
        create_dir(tmp_proto_path)?;
    }

    copy(
        manifest_path.join("proto/build.rs"),
        tmp_path.join("proto/build.rs"),
    )?;

    copy(
        manifest_path.join("proto/Cargo.toml"),
        tmp_path.join("proto/Freight.toml"),
    )?;

    copy(
        manifest_path.join("proto/bicycle.proto"),
        tmp_path.join("proto/bicycle.proto"),
    )?;

    let tmp_proto_src_path = tmp_path.join("proto/src");

    if !tmp_proto_src_path.exists() {
        create_dir(tmp_proto_src_path)?;
    }

    copy(
        manifest_path.join("proto/src/lib.rs"),
        tmp_path.join("proto/src/lib.rs"),
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

    // SHIMS

    let tmp_shims_path = tmp_path.join("shims");

    if !tmp_shims_path.exists() {
        create_dir(tmp_shims_path)?;
    }

    copy(
        manifest_path.join("shims/build.rs"),
        tmp_path.join("shims/build.rs"),
    )?;

    copy(
        manifest_path.join("shims/Cargo.toml"),
        tmp_path.join("shims/Freight.toml"),
    )?;

    let tmp_shims_src_path = tmp_path.join("shims/src");

    if !tmp_shims_src_path.exists() {
        create_dir(tmp_shims_src_path)?;
    }

    copy(
        manifest_path.join("shims/src/lib.rs"),
        tmp_path.join("shims/src/lib.rs"),
    )?;

    let tmp_shims_src_models_path = tmp_path.join("shims/src/models");

    if !tmp_shims_src_models_path.exists() {
        create_dir(tmp_shims_src_models_path)?;
    }

    copy(
        manifest_path.join("shims/src/models/example.rs"),
        tmp_path.join("shims/src/models/example.rs"),
    )?;

    Ok(())
}
