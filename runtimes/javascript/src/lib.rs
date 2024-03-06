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

use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("bicycle.runtime.javascript");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("runtime_descriptor");

pub use proto::runtime_server::RuntimeServer;
use proto::{runtime_server::Runtime, Empty, Name, OneOff, Output, Script, Scripts, Stored};

use deno_core::{v8, JsRuntime, RuntimeOptions};

const SCRIPT_DIR: &'static str = "__bicycle.runtime.javascript__";

fn run_js(src: &str, _arguments: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut js_runtime = JsRuntime::new(RuntimeOptions::default());

    let result = js_runtime.execute_script("<script>", format!("{};main()", src))?;

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
