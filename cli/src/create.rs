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

use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::time::Instant;
use std::{fs, fs::File};

use prost::Message;
use prost_types::FileDescriptorSet;

use crate::utils::construct_model;
use crate::{gen, utils::Model, PRECOMPILE_DIR};

/// builds a Bicycle database server binary and client proto file.
/// outputs to `out/` directory.
///
/// * `schema_path` - path to the schema.proto file
/// * `engine` - the database engine used
/// * `runtimes` - list of SPROC runtimes to include in build
pub fn create(schema_path: &str, engine: &str, runtimes: Vec<String>) {
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

    let now = Instant::now();
    println!("📁 generating files...");

    gen::gen(models, engine, runtimes);

    if let Err(err) = env::set_current_dir(PRECOMPILE_DIR) {
        eprintln!("Failed to change directory: {}", err);
        exit(1);
    }

    if !Path::new("cli").exists() {
        fs::create_dir("cli").unwrap();
        fs::create_dir("cli/src").unwrap();

        let code = r#"
fn main() {
    println!("tmp");
}
        "#;

        let src_lib_path = Path::new("cli").join("src/lib.rs");
        let src_main_path = Path::new("cli").join("src/main.rs");
        let build_path = Path::new("cli").join("build.rs");

        let mut src_lib_file = File::create(src_lib_path).unwrap();
        src_lib_file.write_all(code.as_bytes()).unwrap();

        let mut src_main_file = File::create(src_main_path).unwrap();
        src_main_file.write_all(code.as_bytes()).unwrap();

        let mut build_file = File::create(build_path).unwrap();
        build_file.write_all(code.as_bytes()).unwrap();
    }

    println!(
        "📁 done generating files. [{}ms]",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    println!("🛠️  building server...");

    Command::new("cargo")
        .args(["build", "--release", "-p", "bicycle_server"])
        .output()
        .unwrap();

    println!("🛠️  done building server. [{}ms]", now.elapsed().as_millis());

    if !Path::new("../out").exists() {
        fs::create_dir("../out").unwrap();
    }

    let now = Instant::now();
    println!("📦 moving proto file...");

    Command::new("mv")
        .args(["./core/bicycle.proto", "../out"])
        .output()
        .unwrap();

    println!(
        "📦 done moving proto file. [{}ms]",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    println!("📦 moving server binary...");

    Command::new("mv")
        .args(["./target/release/bicycle_server", "../out/server"])
        .output()
        .unwrap();

    println!(
        "📦 done moving server binary. [{}ms]",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    println!("📁 clearing {}...", PRECOMPILE_DIR);

    // fs::remove_dir_all(&format!("../{}", PRECOMPILE_DIR)).unwrap();

    println!(
        "📁 cleared {}. [{}ms]",
        PRECOMPILE_DIR,
        now.elapsed().as_millis()
    );

    println!("✅ done!");

    println!("\n🚀 start server: ./out/server\n🚲 client codegen: ./out/bicycle.proto")
}
