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

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{create_dir, read_dir, read_to_string, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::sync::RwLock;

use bicycle_core::models;
use bicycle_core::proto as bicycle_proto;
use prost::Message;

use deno_core::{anyhow, extension, op2, v8, JsRuntime, RuntimeOptions};
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("bicycle.runtime.javascript");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("runtime_descriptor");

pub use proto::runtime_server::RuntimeServer;
use proto::{runtime_server::Runtime, Empty, Name, OneOff, Output, Script, Scripts, Stored};

const SCRIPT_DIR: &'static str = "__bicycle.runtime.javascript__";

// ##MODEL_OPS_START##
#[op2]
#[arraybuffer]
fn op_get_examples_by_pk(#[arraybuffer] index_query: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let index_query = bicycle_proto::IndexQuery::decode(index_query)?;
    let examples = models::example::get_examples_by_pk(index_query)?;

    Ok(bicycle_proto::Examples { examples }.encode_to_vec())
}

#[op2(fast)]
fn op_delete_examples_by_pk(#[arraybuffer] index_query: &[u8]) -> Result<(), anyhow::Error> {
    let index_query = bicycle_proto::IndexQuery::decode(index_query)?;
    models::example::delete_examples_by_pk(index_query)?;

    Ok(())
}

#[op2(fast)]
fn op_put_example(#[arraybuffer] example: &[u8]) -> Result<(), anyhow::Error> {
    let example = bicycle_proto::Example::decode(example)?;
    models::example::put_example(example)?;

    Ok(())
}

#[op2(fast)]
fn op_batch_put_examples(#[arraybuffer] examples: &[u8]) -> Result<(), anyhow::Error> {
    let example = bicycle_proto::Examples::decode(examples)?;
    models::example::batch_put_examples(example)?;

    Ok(())
}

extension!(
    example_extension,
    ops = [
        op_get_examples_by_pk,
        op_delete_examples_by_pk,
        op_put_example,
        op_batch_put_examples
    ],
    js = ["src/models/example.js"]
);
// ##MODEL_OPS_END##

fn run_js(src: &str, arguments: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let extensions = vec![
        // ##MODEL_EXTENSIONS_START##
        example_extension::init_ops_and_esm(),
        // ##MODEL_EXTENSIONS_END##
    ];

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions,
        ..Default::default()
    });

    let result = js_runtime.execute_script(
        "<script>",
        // !! this implementation is ridiculous but works for now
        format!("{};main(new Uint8Array({:?}))", src, arguments),
    )?;

    let scope = &mut js_runtime.handle_scope();
    let local = v8::Local::new(scope, result);

    let buf: serde_v8::JsBuffer = serde_v8::from_v8(scope, local)?;

    Ok(buf.to_vec())
}

pub struct RuntimeService {
    scripts: RwLock<BTreeMap<String, String>>,
}

impl RuntimeService {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let script_dir = Path::new(SCRIPT_DIR);

        let mut scripts = BTreeMap::new();

        if !script_dir.exists() {
            create_dir(script_dir)?;
        } else {
            let paths = read_dir(script_dir)?;

            for path in paths {
                let path = path.unwrap().path();

                let name = path.file_name().unwrap().to_str().unwrap();
                let script = read_to_string(&path)?;

                scripts.insert(name.to_string(), script);
            }
        }

        Ok(Self {
            scripts: RwLock::new(scripts),
        })
    }
}

#[tonic::async_trait]
impl Runtime for RuntimeService {
    async fn del(&self, req: Request<Name>) -> Result<Response<Empty>, Status> {
        let name = req.into_inner().name;
        let script_dir = Path::new(SCRIPT_DIR);

        remove_file(script_dir.join(&name))?;

        self.scripts.write().unwrap().remove(&name);

        Ok(Response::new(Empty {}))
    }

    async fn put(&self, req: Request<Script>) -> Result<Response<Empty>, Status> {
        let Script { name, script } = req.into_inner();
        let script_dir = Path::new(SCRIPT_DIR);

        let mut file = File::create(script_dir.join(&name))?;
        file.write_all(script.as_bytes())?;

        self.scripts.write().unwrap().insert(name, script);

        Ok(Response::new(Empty {}))
    }

    async fn list(&self, _req: Request<Empty>) -> Result<Response<Scripts>, Status> {
        let mut scripts = vec![];

        for (name, script) in &*self.scripts.read().unwrap() {
            scripts.push(Script {
                name: name.to_string(),
                script: script.to_string(),
            })
        }

        Ok(Response::new(Scripts { scripts }))
    }

    async fn run_one_off(&self, req: Request<OneOff>) -> Result<Response<Output>, Status> {
        let OneOff { script, arguments } = req.into_inner();

        match run_js(&script, &arguments) {
            Ok(message) => Ok(Response::new(Output { message })),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn run_stored(&self, req: Request<Stored>) -> Result<Response<Output>, Status> {
        let Stored { name, arguments } = req.into_inner();

        if let Some(script) = self.scripts.read().unwrap().get(&name) {
            match run_js(script, &arguments) {
                Ok(message) => Ok(Response::new(Output { message })),
                Err(err) => Err(Status::internal(err.to_string())),
            }
        } else {
            Err(Status::not_found(format!(
                "script with name \"{}\" not found",
                name
            )))
        }
    }
}
