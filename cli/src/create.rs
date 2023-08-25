/*
Bicycle is a database, used for things databases are used for.

Copyright (C) 2023  sean watters

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

use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

use prost::Message;
use prost_types::FileDescriptorSet;

use crate::utils::construct_model;
use crate::{gen, utils::Model, OUT_DIR};

pub fn create(schema_path: &str) {
    fs::create_dir(OUT_DIR).unwrap();

    std::env::set_var("OUT_DIR", OUT_DIR);
    let out_dir = PathBuf::from(OUT_DIR);

    let tmp_desc_path = out_dir.join("tmp_desc.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&tmp_desc_path)
        .compile(&[&schema_path], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let descriptor_bytes = fs::read(&tmp_desc_path).unwrap();
    let file_descriptor_set = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    let mut models: Vec<Model> = vec![];

    for file in file_descriptor_set.file {
        for message in file.message_type {
            match construct_model(&message, true) {
                Ok(model) => models.push(model),
                Err(err) => eprintln!("{}", err),
            }
        }
    }

    fs::remove_file(tmp_desc_path).unwrap();
    fs::remove_file(out_dir.join("database.rs")).unwrap();

    gen::gen(models);

    if let Err(err) = env::set_current_dir(OUT_DIR) {
        eprintln!("Failed to change directory: {}", err);
        exit(1);
    }

    Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    fs::create_dir("../out").unwrap();

    Command::new("mv")
        .args(["./core/database.proto", "../out"])
        .output()
        .unwrap();

    Command::new("mv")
        .args(["./target/release/bicycle_server", "../out/server"])
        .output()
        .unwrap();

    fs::remove_dir_all(&format!("../{}", OUT_DIR)).unwrap();
}
