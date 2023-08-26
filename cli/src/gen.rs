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

use std::fs;
use std::io::prelude::*;
use std::path::Path;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use lazy_static::lazy_static;

use crate::{utils::Model, PRECOMPILE_DIR};

const WORKSPACE_CARGO_TOML: &'static str = include_str!("../../Cargo.toml");

const CORE_BUILD_RS: &'static str = include_str!("../../core/build.rs");
const CORE_CARGO_TOML: &'static str = include_str!("../../core/Cargo.toml");
const CORE_DATABASE_PROTO: &'static str = include_str!("../../core/bicycle.proto");
const CORE_SRC_LIB_RS: &'static str = include_str!("../../core/src/lib.rs");
const CORE_SRC_MODELS_MOD_RS: &'static str = include_str!("../../core/src/models/example.rs");

const SERVER_CARGO_TOML: &'static str = include_str!("../../server/Cargo.toml");
const SERVER_SRC_BICYCLE_RS: &'static str = include_str!("../../server/src/bicycle.rs");
const SERVER_SRC_MAIN_RS: &'static str = include_str!("../../server/src/main.rs");

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
    static ref PROTO_MODEL_MESSAGES: String = get_between(
        CORE_DATABASE_PROTO,
        "##MODEL_MESSAGES_START##",
        Some("##MODEL_MESSAGES_END##"),
    );
    static ref PROTO_MODEL_RPCS: String = get_between(
        CORE_DATABASE_PROTO,
        "##MODEL_RPCS_START##",
        Some("##MODEL_RPCS_END##"),
    );
    static ref SERVER_HANDLERS: String = get_between(
        SERVER_SRC_BICYCLE_RS,
        "##START_HANDLERS##",
        Some("##END_HANDLERS##")
    );
}

// TODO: remove all .unwrap()s

fn create_dir(path: &str) {
    let path = format!("{}/{}", PRECOMPILE_DIR, path);

    if Path::new(&path).exists() {
        fs::remove_dir_all(path.clone()).unwrap();
    }
    fs::create_dir(&path).unwrap();
}

fn write_file(path: &str, content: &str) {
    let mut file = fs::File::create(format!("{}/{}", PRECOMPILE_DIR, path)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub(crate) fn gen(models: Vec<Model>) {
    let sanitized_workspace_cargo_toml = WORKSPACE_CARGO_TOML.replace("\"cli\", ", "");
    write_file("Cargo.toml", &sanitized_workspace_cargo_toml);

    create_dir("core");
    write_file("core/build.rs", CORE_BUILD_RS);
    write_file("core/Cargo.toml", CORE_CARGO_TOML);

    create_dir("core/src");
    write_file("core/src/lib.rs", CORE_SRC_LIB_RS);

    create_dir("core/src/models");

    create_dir("server");
    write_file("server/Cargo.toml", SERVER_CARGO_TOML);

    create_dir("server/src");

    let mut rpc_block = "".to_string();
    let mut messages_block = "".to_string();

    let mut core_models_mod_rs = "".to_string();

    let mut server_handlers_block = "".to_string();

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
            "{}pub mod {};\n",
            core_models_mod_rs,
            model.name.to_snake_case()
        );

        let model_file_content = replace_model_name(&model, &CORE_SRC_MODELS_MOD_RS.to_string());

        write_file(
            &format!("core/src/models/{}.rs", model.name.to_snake_case()),
            &model_file_content,
        );

        server_handlers_block = format!(
            "{}{}{}",
            server_handlers_block,
            if i == 0 { "" } else { "\n" },
            replace_model_name(&model, &SERVER_HANDLERS)
        );
    }

    let proto = CORE_DATABASE_PROTO
        .replace(&PROTO_MODEL_RPCS.to_string(), &rpc_block)
        .replace(&PROTO_MODEL_MESSAGES.to_string(), &messages_block);

    write_file("core/bicycle.proto", &proto);
    write_file("core/src/models/mod.rs", &core_models_mod_rs);

    write_file("server/src/main.rs", &SERVER_SRC_MAIN_RS);

    let bicycle_rs =
        SERVER_SRC_BICYCLE_RS.replace(&SERVER_HANDLERS.to_string(), &server_handlers_block);

    write_file("server/src/bicycle.rs", &bicycle_rs);
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
