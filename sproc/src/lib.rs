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
use std::fs::{create_dir, read_dir, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::RwLock;

use bicycle_core::models;
use bicycle_core::proto as bicycle_proto;

use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("bicycle.sproc");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("sproc_descriptor");

use prost::Message;
pub use proto::sproc_server::SprocServer;
use proto::{sproc_server::Sproc, Empty, Name, OneOff, Proc, Procs, Stored};

use wasmtime::{Caller, Engine, Linker, Module, Store};

const SCRIPT_DIR: &'static str = "__bicycle.sproc__";

fn exec(
    src: &[u8],
    args: &Option<prost_types::Value>,
) -> Result<prost_types::Value, Box<dyn Error>> {
    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let stdin = match args {
        Some(args) => wasi_common::pipe::ReadPipe::from(args.encode_to_vec()),
        None => wasi_common::pipe::ReadPipe::from(vec![]),
    };

    let stdout = wasi_common::pipe::WritePipe::new_in_memory();

    // ##START_HOST_FNS##
    linker.func_wrap(
        "env",
        "get_examples_by_pk",
        move |mut caller: Caller<'_, wasi_common::WasiCtx>, ptr: i32, len: i32| -> i64 {
            // TODO: get `&[u8]` from ptr and len
            // let index_query = bicycle_proto::IndexQuery::decode(index_query)?;
            // let examples = models::example::get_examples_by_pk(index_query)?;

            println!("get_examples_by_pk: ptr{} len{}", ptr, len);

            // combine ptr and len to i64
            let ptr64 = (ptr as i64) << 32;
            let len64 = len as i64;
            ptr64 | len64
        },
    )?;

    linker.func_wrap(
        "env",
        "delete_examples_by_pk",
        move |mut caller: Caller<'_, wasi_common::WasiCtx>, ptr: i32, len: i32| {
            // TODO: get `&[u8]` from ptr and len
            // let index_query = bicycle_proto::IndexQuery::decode(index_query)?;
            // models::example::delete_examples_by_pk(index_query)?;

            println!("delete_examples_by_pk: ptr{} len{}", ptr, len);
        },
    )?;

    linker.func_wrap(
        "env",
        "put_example",
        move |mut caller: Caller<'_, wasi_common::WasiCtx>, ptr: i32, len: i32| {
            // TODO: get `&[u8]` from ptr and len
            // let example = bicycle_proto::Example::decode(example)?;
            // models::example::put_example(example)?;

            println!("put_example: ptr{} len{}", ptr, len);
        },
    )?;

    linker.func_wrap(
        "env",
        "batch_put_examples",
        move |mut caller: Caller<'_, wasi_common::WasiCtx>, ptr: i32, len: i32| {
            // TODO: get `&[u8]` from ptr and len
            // let example = bicycle_proto::Examples::decode(examples)?;
            // models::example::batch_put_examples(example)?;

            println!("batch_put_examples: ptr{} len{}", ptr, len);
        },
    )?;
    // ##END_HOST_FNS##

    let wasi = wasi_common::sync::WasiCtxBuilder::new()
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::new(&engine, src)?;
    linker.module(&mut store, "", &module)?;

    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    drop(store);

    match stdout.try_into_inner() {
        Ok(mut out) => {
            out.set_position(0);
            let decoded = prost_types::Value::decode(out)?;
            Ok(decoded)
        }
        Err(_) => Err("references to write pipe remain".into()),
    }
}

pub struct SprocService {
    procs: RwLock<BTreeMap<String, Vec<u8>>>,
}

impl SprocService {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let proc_dir = Path::new(SCRIPT_DIR);

        let mut procs = BTreeMap::new();

        if !proc_dir.exists() {
            create_dir(proc_dir)?;
        } else {
            let paths = read_dir(proc_dir)?;

            for path in paths {
                let path = path.unwrap().path();

                let name = path.file_name().unwrap().to_str().unwrap();
                let mut proc = vec![];

                File::open(&path)?.read_to_end(&mut proc)?;

                procs.insert(name.to_string(), proc);
            }
        }

        Ok(Self {
            procs: RwLock::new(procs),
        })
    }
}

#[tonic::async_trait]
impl Sproc for SprocService {
    async fn remove(&self, req: Request<Name>) -> Result<Response<Empty>, Status> {
        let name = req.into_inner().name;
        let proc_dir = Path::new(SCRIPT_DIR);

        remove_file(proc_dir.join(&name))?;

        self.procs.write().unwrap().remove(&name);

        Ok(Response::new(Empty {}))
    }

    async fn deploy(&self, req: Request<Proc>) -> Result<Response<Empty>, Status> {
        let Proc { name, proc } = req.into_inner();
        let proc_dir = Path::new(SCRIPT_DIR);

        let mut file = File::create(proc_dir.join(&name))?;
        file.write_all(&proc)?;

        self.procs.write().unwrap().insert(name, proc);

        Ok(Response::new(Empty {}))
    }

    async fn list(&self, _req: Request<Empty>) -> Result<Response<Procs>, Status> {
        let mut procs = vec![];

        for (name, proc) in &*self.procs.read().unwrap() {
            procs.push(Proc {
                name: name.to_string(),
                proc: proc.clone(),
            })
        }

        Ok(Response::new(Procs { procs }))
    }

    async fn exec_one_off(
        &self,
        req: Request<OneOff>,
    ) -> Result<Response<prost_types::Value>, Status> {
        let OneOff { proc, args } = req.into_inner();

        match exec(&proc, &args) {
            Ok(value) => Ok(Response::new(value)),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn exec_stored(
        &self,
        req: Request<Stored>,
    ) -> Result<Response<prost_types::Value>, Status> {
        let Stored { name, args } = req.into_inner();

        if let Some(proc) = self.procs.read().unwrap().get(&name) {
            match exec(proc, &args) {
                Ok(value) => Ok(Response::new(value)),
                Err(err) => Err(Status::internal(err.to_string())),
            }
        } else {
            Err(Status::not_found(format!(
                "proc with name \"{}\" not found",
                name
            )))
        }
    }
}
