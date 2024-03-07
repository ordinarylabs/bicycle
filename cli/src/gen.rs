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

const CORE_BUILD_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/build.rs"
));
const CORE_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/Freight.toml"
));
const CORE_BICYCLE_PROTO: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/core/bicycle.proto"
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

const ENGINES_ROCKSDB_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/rocksdb/Freight.toml"
));
const ENGINES_ROCKSDB_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/engines/rocksdb/src/lib.rs"
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

// RUNTIMES

const RUNTIMES_JAVASCRIPT_CARGO_TOML: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/runtimes/javascript/Freight.toml"
));
const RUNTIMES_JAVASCRIPT_BUILD_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/runtimes/javascript/build.rs"
));
const RUNTIMES_JAVASCRIPT_RUNTIME_PROTO: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/runtimes/javascript/runtime.proto"
));
const RUNTIMES_JAVASCRIPT_SRC_LIB_RS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/runtimes/javascript/src/lib.rs"
));
const RUNTIMES_JAVASCRIPT_SRC_MODELS_EXAMPLE_JS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/tmp/runtimes/javascript/src/models/example.js"
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
    static ref PROTO_MODEL_MESSAGES: String = get_between(
        CORE_BICYCLE_PROTO,
        "##MODEL_MESSAGES_START##",
        Some("##MODEL_MESSAGES_END##"),
    );
    static ref PROTO_MODEL_RPCS: String = get_between(
        CORE_BICYCLE_PROTO,
        "##MODEL_RPCS_START##",
        Some("##MODEL_RPCS_END##"),
    );
    static ref SERVER_HANDLERS: String = get_between(
        SERVER_SRC_MAIN_RS,
        "##START_HANDLERS##",
        Some("##END_HANDLERS##")
    );
    static ref RUNTIMES_JAVASCRIPT_OPS: String = get_between(
        RUNTIMES_JAVASCRIPT_SRC_LIB_RS,
        "##START_OPS##",
        Some("##END_OPS##")
    );
    static ref RUNTIMES_JAVASCRIPT_EXTENSIONS: String = get_between(
        RUNTIMES_JAVASCRIPT_SRC_LIB_RS,
        "##START_EXTENSIONS##",
        Some("##END_EXTENSIONS##")
    );
}

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

pub(crate) fn gen(models: Vec<Model>, _engine: &str, runtimes: Vec<String>) {
    // BASE
    write_file("Cargo.toml", WORKSPACE_CARGO_TOML);

    // CORE
    create_dir("core");
    write_file("core/build.rs", CORE_BUILD_RS);
    write_file("core/Cargo.toml", CORE_CARGO_TOML);

    create_dir("core/src");
    write_file("core/src/lib.rs", CORE_SRC_LIB_RS);

    create_dir("core/src/models");

    // ENGINES
    create_dir("engines");
    create_dir("engines/rocksdb");
    write_file("engines/rocksdb/Cargo.toml", ENGINES_ROCKSDB_CARGO_TOML);

    create_dir("engines/rocksdb/src");
    write_file("engines/rocksdb/src/lib.rs", ENGINES_ROCKSDB_SRC_LIB_RS);

    // SERVER
    create_dir("server");
    create_dir("server/src");

    // RUNTIMES
    create_dir("runtimes");
    create_dir("runtimes/javascript");
    write_file(
        "runtimes/javascript/Cargo.toml",
        RUNTIMES_JAVASCRIPT_CARGO_TOML,
    );
    write_file("runtimes/javascript/build.rs", RUNTIMES_JAVASCRIPT_BUILD_RS);
    write_file(
        "runtimes/javascript/runtime.proto",
        RUNTIMES_JAVASCRIPT_RUNTIME_PROTO,
    );

    create_dir("runtimes/javascript/src");
    create_dir("runtimes/javascript/src/models");

    let mut rpc_block = "".to_string();
    let mut messages_block = "".to_string();

    let mut core_models_mod_rs = "".to_string();

    let mut server_handlers_block = "".to_string();

    let mut runtimes_javascript_ops_block = "".to_string();
    let mut runtimes_javascript_extensions_block = "".to_string();

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

        let model_file_content =
            replace_model_name(&model, &CORE_SRC_MODELS_EXAMPLE_RS.to_string());

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

        let runtimes_javascript_model_example_content = replace_model_name(
            &model,
            &RUNTIMES_JAVASCRIPT_SRC_MODELS_EXAMPLE_JS.to_string(),
        );

        write_file(
            &format!(
                "runtimes/javascript/src/models/{}.js",
                model.name.to_snake_case()
            ),
            &runtimes_javascript_model_example_content,
        );

        runtimes_javascript_ops_block = format!(
            "{}{}{}",
            runtimes_javascript_ops_block,
            if i == 0 { "" } else { "\n" },
            replace_model_name(&model, &RUNTIMES_JAVASCRIPT_OPS)
        );

        runtimes_javascript_extensions_block = format!(
            "{}{}{}",
            runtimes_javascript_extensions_block,
            if i == 0 { "" } else { "\n" },
            replace_model_name(&model, &RUNTIMES_JAVASCRIPT_EXTENSIONS)
        );
    }

    // CORE
    let proto = CORE_BICYCLE_PROTO
        .replace(&PROTO_MODEL_RPCS.to_string(), &rpc_block)
        .replace(&PROTO_MODEL_MESSAGES.to_string(), &messages_block);

    write_file("core/src/models/mod.rs", &core_models_mod_rs);
    write_file("core/bicycle.proto", &proto);

    // RUNTIMES
    let runtimes_javascript_src_lib_rs = RUNTIMES_JAVASCRIPT_SRC_LIB_RS
        .replace(
            &RUNTIMES_JAVASCRIPT_OPS.to_string(),
            &runtimes_javascript_ops_block,
        )
        .replace(
            &RUNTIMES_JAVASCRIPT_EXTENSIONS.to_string(),
            &runtimes_javascript_extensions_block,
        );

    write_file(
        "runtimes/javascript/src/lib.rs",
        &runtimes_javascript_src_lib_rs,
    );

    let mut runtime_descriptors = "".to_string();
    let mut runtime_services = "".to_string();
    let mut runtime_deps = "".to_string();

    for runtime in runtimes {
        runtime_descriptors = format!(
            r#"{}.register_encoded_file_descriptor_set({}_runtime::FILE_DESCRIPTOR_SET)
        "#,
            runtime_descriptors, runtime
        );
        runtime_services = format!(
            r#"{}.add_service({}_runtime::RuntimeServer::new({}_runtime::RuntimeService::new()?))
        "#,
            runtime_services, runtime, runtime,
        );

        runtime_deps = format!(
            r#"
{}_runtime = {{ workspace = true }}
"#,
            runtime
        );
    }

    let server_cargo_toml = SERVER_CARGO_TOML.replace("##RUNTIME_DEPS##", &runtime_deps);
    write_file("server/Cargo.toml", &server_cargo_toml);

    let server_src_main_rs = SERVER_SRC_MAIN_RS
        .replace(&SERVER_HANDLERS.to_string(), &server_handlers_block)
        .replace("// ##RUNTIME_DESCRIPTORS##", &runtime_descriptors)
        .replace("// ##RUNTIME_SERVICES##", &runtime_services);
    write_file("server/src/main.rs", &server_src_main_rs);
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
