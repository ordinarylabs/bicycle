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

use std::fs;
use std::io::prelude::*;
use std::path::Path;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use lazy_static::lazy_static;

use crate::{utils::Model, PRECOMPILE_DIR};

// BASE

const WORKSPACE_CARGO_TOML: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/tmp/Freight.toml"));

// CORE

const CORE_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/Freight.toml"
));
const CORE_SRC_MODELS_EXAMPLE_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/src/models/example.rs"
));
const CORE_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/src/lib.rs"
));

// ENGINES

// RocksDB
const ENGINES_ROCKSDB_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/rocksdb/Freight.toml"
));
const ENGINES_ROCKSDB_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/rocksdb/src/lib.rs"
));

// SQLite
const ENGINES_SQLITE_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/sqlite/Freight.toml"
));
const ENGINES_SQLITE_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/sqlite/src/lib.rs"
));

// PROTO

const PROTO_BUILD_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/proto/build.rs"
));
const PROTO_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/proto/Freight.toml"
));
const PROTO_BICYCLE_PROTO: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/proto/bicycle.proto"
));
const PROTO_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/proto/src/lib.rs"
));

// SERVER

const SERVER_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/server/Freight.toml"
));
const SERVER_SRC_MAIN_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/server/src/main.rs"
));

// SHIMS

const SHIMS_BUILD_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/shims/build.rs"
));
const SHIMS_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/shims/Freight.toml"
));
const SHIMS_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/shims/src/lib.rs"
));
const SHIMS_SRC_MODELS_EXAMPLE_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/shims/src/models/example.rs"
));

fn get_between(content: &str, from: &str, to: Option<&str>) -> String {
    let start = match content.find(from) {
        Some(i) => i + from.len() + 1,
        None => 0,
    };

    if let Some(to) = to {
        let end = match content.find(to) {
            Some(i) => i - 4,
            None => 0,
        };

        &content[start..end]
    } else {
        &content[start..]
    }
    .to_string()
}

lazy_static! {
    static ref WORKSPACE_ENGINE: String = get_between(
        WORKSPACE_CARGO_TOML,
        "##START_WORKSPACE_ENGINE##",
        Some("##END_WORKSPACE_ENGINE##"),
    );
    static ref PROTO_MODEL_MESSAGES: String = get_between(
        PROTO_BICYCLE_PROTO,
        "##MODEL_MESSAGES_START##",
        Some("##MODEL_MESSAGES_END##"),
    );
    static ref PROTO_MODEL_RPCS: String = get_between(
        PROTO_BICYCLE_PROTO,
        "##MODEL_RPCS_START##",
        Some("##MODEL_RPCS_END##"),
    );
    static ref SERVER_HANDLERS: String = get_between(
        SERVER_SRC_MAIN_RS,
        "##START_HANDLERS##",
        Some("##END_HANDLERS##")
    );
    static ref SPROC_HOST_FNS: String = get_between(
        CORE_SRC_LIB_RS,
        "##START_HOST_FNS##",
        Some("##END_HOST_FNS##")
    );
}

fn create_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}/{}", PRECOMPILE_DIR, path);

    if Path::new(&path).exists() {
        fs::remove_dir_all(path.clone())?;
    }
    fs::create_dir(&path)?;

    Ok(())
}

fn write_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(format!("{}/{}", PRECOMPILE_DIR, path))?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub(crate) fn gen(models: Vec<Model>, engine: &str) -> Result<(), Box<dyn std::error::Error>> {
    // BASE

    let mut sanitized_workspace_cargo_toml = WORKSPACE_CARGO_TOML.to_string();

    let workspace_toml = sanitized_workspace_cargo_toml
        .parse::<toml::Table>()
        .unwrap();

    if let Some(version) = workspace_toml["workspace"]["package"]["version"].as_str() {
        sanitized_workspace_cargo_toml = sanitized_workspace_cargo_toml
            .replace(&format!("version = \"{}\"", version), "version = \"0.0.0\"");
    }

    let workspace_engine = WORKSPACE_ENGINE.replace("sqlite", engine);

    let sanitized_workspace_cargo_toml =
        sanitized_workspace_cargo_toml.replace(&WORKSPACE_ENGINE.to_string(), &workspace_engine);

    write_file("Cargo.toml", &sanitized_workspace_cargo_toml)?;

    // CORE
    create_dir("core")?;
    write_file("core/Cargo.toml", CORE_CARGO_TOML)?;

    create_dir("core/src")?;
    create_dir("core/src/models")?;

    // ENGINES
    create_dir("engines")?;

    // RocksDB
    create_dir("engines/rocksdb")?;
    write_file("engines/rocksdb/Cargo.toml", ENGINES_ROCKSDB_CARGO_TOML)?;

    create_dir("engines/rocksdb/src")?;
    write_file("engines/rocksdb/src/lib.rs", ENGINES_ROCKSDB_SRC_LIB_RS)?;

    // SQLite
    create_dir("engines/sqlite")?;
    write_file("engines/sqlite/Cargo.toml", ENGINES_SQLITE_CARGO_TOML)?;

    create_dir("engines/sqlite/src")?;
    write_file("engines/sqlite/src/lib.rs", ENGINES_SQLITE_SRC_LIB_RS)?;

    // PROTO

    create_dir("proto")?;
    write_file("proto/build.rs", PROTO_BUILD_RS)?;
    write_file("proto/Cargo.toml", PROTO_CARGO_TOML)?;

    create_dir("proto/src")?;
    write_file("proto/src/lib.rs", PROTO_SRC_LIB_RS)?;

    // SERVER
    create_dir("server")?;
    create_dir("server/src")?;
    write_file("server/Cargo.toml", &SERVER_CARGO_TOML)?;

    // SHIMS

    create_dir("shims")?;
    write_file("shims/build.rs", SHIMS_BUILD_RS)?;
    write_file("shims/Cargo.toml", SHIMS_CARGO_TOML)?;

    create_dir("shims/src")?;
    write_file("shims/src/lib.rs", SHIMS_SRC_LIB_RS)?;

    create_dir("shims/src/models")?;

    let mut rpc_block = "".to_string();
    let mut messages_block = "".to_string();

    let mut core_models_mod_rs = "".to_string();
    let mut shims_models_mod_rs = "".to_string();

    let mut server_handlers_block = "".to_string();

    let mut sprocs_host_fns_block = "".to_string();

    for (i, model) in models.iter().enumerate() {
        let (rpc_chunk, messages_chunk) = gen_proto(&model);

        rpc_block = format!(
            "{}{}{}",
            rpc_block,
            if i == 0 { "" } else { "\n" },
            rpc_chunk
        );
        messages_block = format!(
            "{}{}{}",
            messages_block,
            if i == 0 { "" } else { "\n" },
            messages_chunk
        );

        core_models_mod_rs = format!(
            "{}mod {};\npub use {}::*;",
            core_models_mod_rs,
            model.name.to_snake_case(),
            model.name.to_snake_case()
        );

        let core_src_model_rs_content =
            replace_model_name(&model, &CORE_SRC_MODELS_EXAMPLE_RS.to_string());

        write_file(
            &format!("core/src/models/{}.rs", model.name.to_snake_case()),
            &core_src_model_rs_content,
        )?;

        server_handlers_block = format!(
            "{}{}{}",
            server_handlers_block,
            if i == 0 { "" } else { "\n" },
            replace_model_name(&model, &SERVER_HANDLERS)
        );

        shims_models_mod_rs = format!(
            "{}mod {};\npub use {}::*;",
            shims_models_mod_rs,
            model.name.to_snake_case(),
            model.name.to_snake_case()
        );

        let shims_src_model_rs_content =
            replace_model_name(&model, &SHIMS_SRC_MODELS_EXAMPLE_RS.to_string());

        write_file(
            &format!("shims/src/models/{}.rs", model.name.to_snake_case()),
            &shims_src_model_rs_content,
        )?;

        sprocs_host_fns_block = format!(
            "{}{}{}",
            sprocs_host_fns_block,
            if i == 0 { "" } else { "\n" },
            replace_model_name(&model, &SPROC_HOST_FNS)
        );
    }

    // CORE
    let core_src_lib_rs =
        CORE_SRC_LIB_RS.replace(&SPROC_HOST_FNS.to_string(), &sprocs_host_fns_block);
    write_file("core/src/lib.rs", &core_src_lib_rs)?;
    write_file("core/src/models/mod.rs", &core_models_mod_rs)?;

    // PROTO
    let proto = PROTO_BICYCLE_PROTO
        .replace(&PROTO_MODEL_RPCS.to_string(), &rpc_block)
        .replace(&PROTO_MODEL_MESSAGES.to_string(), &messages_block);

    write_file("proto/bicycle.proto", &proto)?;

    // SERVER
    let server_src_main_rs =
        SERVER_SRC_MAIN_RS.replace(&SERVER_HANDLERS.to_string(), &server_handlers_block);
    write_file("server/src/main.rs", &server_src_main_rs)?;

    // SHIMS
    write_file("shims/src/models/mod.rs", &shims_models_mod_rs)?;

    Ok(())
}

fn gen_proto(model: &Model) -> (String, String) {
    let rpc_chunk = PROTO_MODEL_RPCS.replace("Example", &model.name);

    let mut messages_chunk = replace_model_name(&model, &PROTO_MODEL_MESSAGES);

    let nested_messages_block = get_nested_messages_block(&model);
    let properties_block = get_properties_block(&model);

    messages_chunk = messages_chunk.replace(
        "  string pk = 1;",
        &format!(
            "{}{}{}",
            properties_block,
            if nested_messages_block != "" {
                "\n\n"
            } else {
                ""
            },
            nested_messages_block,
        ),
    );

    (rpc_chunk, messages_chunk)
}

fn get_properties_block(model: &Model) -> String {
    let mut properties_block = "".to_string();

    for (i, property) in model.properties.iter().enumerate() {
        properties_block = format!(
            "{}{}  {} {} = {};",
            properties_block,
            if i == 0 { "" } else { "\n" },
            property._type,
            property.name,
            property.number
        )
    }

    properties_block
}

fn get_nested_messages_block(model: &Model) -> String {
    let mut nested_messages_block = "".to_string();

    for model in model.nested_models.iter() {
        let nested_messages_chunk = get_nested_messages_block(&model);
        let properties_block = get_properties_block(&model);

        nested_messages_block = format!(
            "\nmessage {} {{\n{}{}{}\n}}",
            model.name,
            properties_block,
            if nested_messages_block != "" {
                "\n\n"
            } else {
                ""
            },
            nested_messages_chunk,
        );
    }

    nested_messages_block
}

fn replace_model_name(model: &Model, template: &String) -> String {
    template
        .replace("example", &model.name.to_snake_case())
        .replace("Example", &model.name)
        .replace("EXAMPLE", &model.name.to_shouty_snake_case())
}
