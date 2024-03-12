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

use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;
use std::{fs, fs::File};

use prost::Message;
use prost_types::FileDescriptorSet;

use crate::utils::construct_model;
use crate::{gen, utils::Model, PRECOMPILE_DIR};

/// builds Bicycle components.
///
/// * `schema_path` - path to the schema.proto file
/// * `engine` - the database engine used
pub fn build(schema_path: &str, engine: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(PRECOMPILE_DIR).exists() {
        fs::create_dir(PRECOMPILE_DIR)?;
    }

    std::env::set_var("OUT_DIR", PRECOMPILE_DIR);
    let precompile_dir = PathBuf::from(PRECOMPILE_DIR);

    let tmp_desc_path = precompile_dir.join("tmp_desc.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&tmp_desc_path)
        .compile(&[&schema_path], &[schema_path.replace("schema.proto", "")])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let descriptor_bytes = fs::read(&tmp_desc_path)?;
    let file_descriptor_set = FileDescriptorSet::decode(&descriptor_bytes[..])?;

    let mut models: Vec<Model> = vec![];

    for file in file_descriptor_set.file {
        for message in file.message_type {
            match construct_model(&message, true) {
                Ok(model) => models.push(model),
                Err(err) => eprintln!("{}", err),
            }
        }
    }

    fs::remove_file(tmp_desc_path)?;
    fs::remove_file(precompile_dir.join("bicycle.rs"))?;

    let now = Instant::now();
    println!("📁 generating files...");

    gen::gen(models, engine)?;

    env::set_current_dir(PRECOMPILE_DIR)?;

    if !Path::new("cli").exists() {
        fs::create_dir("cli")?;
        fs::create_dir("cli/src")?;

        let code = r#"
fn main() {
    println!("tmp");
}
        "#;

        let src_lib_path = Path::new("cli").join("src/lib.rs");
        let src_main_path = Path::new("cli").join("src/main.rs");
        let build_path = Path::new("cli").join("build.rs");

        let mut src_lib_file = File::create(src_lib_path)?;
        src_lib_file.write_all(code.as_bytes())?;

        let mut src_main_file = File::create(src_main_path)?;
        src_main_file.write_all(code.as_bytes())?;

        let mut build_file = File::create(build_path)?;
        build_file.write_all(code.as_bytes())?;
    }

    println!(
        "📁 done generating files. [{}ms]",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    println!("🛠️  building server...");

    let out = std::process::Command::new("cargo")
        .args(["build", "--release", "-p", "bicycle_server"])
        .stderr(std::process::Stdio::piped())
        .output()?;

    if !out.status.success() {
        println!("failed to build server: {}", String::from_utf8(out.stderr)?);
        exit(1)
    }

    println!("🛠️  done building server. [{}ms]", now.elapsed().as_millis());

    println!("✅ done!");

    println!(
        "\n🚀 start server with `bicycle start`\n🚲 codegen with ./__bicycle__/proto/bicycle.proto"
    );

    Ok(())
}
