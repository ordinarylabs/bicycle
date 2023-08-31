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

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs, fs::File};

use prost::Message;
use prost_types::FileDescriptorSet;

use crate::utils::construct_model;
use crate::{gen, utils::Model, PRECOMPILE_DIR};

pub fn create(schema_path: &str, plugins: Vec<String>) {
    if Path::new(PRECOMPILE_DIR).exists() {
        fs::remove_dir_all(PRECOMPILE_DIR).unwrap();
    }
    fs::create_dir(PRECOMPILE_DIR).unwrap();

    std::env::set_var("OUT_DIR", PRECOMPILE_DIR);
    let precompile_dir = PathBuf::from(PRECOMPILE_DIR);

    let tmp_desc_path = precompile_dir.join("tmp_desc.bin");

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
    fs::remove_file(precompile_dir.join("bicycle.rs")).unwrap();

    gen::gen(models, plugins);

    if let Err(err) = env::set_current_dir(PRECOMPILE_DIR) {
        eprintln!("Failed to change directory: {}", err);
        exit(1);
    }

    if !Path::new("cli").exists() {
        fs::create_dir("cli").unwrap();

        let code = r#"
            fn main() {
                println!("tmp");
            }
        "#;

        let path = Path::new("cli").join("main.rs");

        let mut file = File::create(path).unwrap();
        file.write_all(code.as_bytes()).unwrap();
    }

    Command::new("cargo")
        .args(["build", "--release", "-p", "bicycle_server"])
        .output()
        .unwrap();

    if !Path::new("../out").exists() {
        fs::create_dir("../out").unwrap();
    }

    Command::new("mv")
        .args(["./core/bicycle.proto", "../out"])
        .output()
        .unwrap();

    Command::new("mv")
        .args(["./target/release/bicycle_server", "../out/server"])
        .output()
        .unwrap();

    fs::remove_dir_all(&format!("../{}", PRECOMPILE_DIR)).unwrap();
}
