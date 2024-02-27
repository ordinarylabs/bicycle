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

use std::fs::{copy, create_dir, metadata};
use std::io;

fn main() -> io::Result<()> {
    let tmp_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp");

    if !metadata(tmp_path).is_ok() {
        create_dir(tmp_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/Freight.toml"),
    )?;

    let tmp_core_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core");

    if !metadata(tmp_core_path).is_ok() {
        create_dir(tmp_core_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/build.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/build.rs"),
    )?;

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/Cargo.toml"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/Freight.toml"),
    )?;

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/bicycle.proto"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/bicycle.proto"),
    )?;

    let tmp_core_src_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/src");

    if !metadata(tmp_core_src_path).is_ok() {
        create_dir(tmp_core_src_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/src/engine.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/src/engine.rs"),
    )?;

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/src/lib.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/src/lib.rs"),
    )?;

    let tmp_core_src_models_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/core/src/models");

    if !metadata(tmp_core_src_models_path).is_ok() {
        create_dir(tmp_core_src_models_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/core/src/models/example.rs"),
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/cli/tmp/core/src/models/example.rs"
        ),
    )?;

    let tmp_server_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/server");

    if !metadata(tmp_server_path).is_ok() {
        create_dir(tmp_server_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/server/Cargo.toml"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/server/Freight.toml"),
    )?;

    let tmp_server_src_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/server/src");

    if !metadata(tmp_server_src_path).is_ok() {
        create_dir(tmp_server_src_path)?;
    }

    copy(
        concat!(env!("CARGO_MANIFEST_DIR"), "/server/src/main.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/server/src/main.rs"),
    )?;

    Ok(())
}
