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

use std::error::Error;

use tonic::transport::Server;
use tonic::{Request, Response, Status};

use bicycle_core;
use bicycle_proto as proto;

use proto::bicycle_server::{Bicycle, BicycleServer};
use proto::IndexQuery;
use proto::FILE_DESCRIPTOR_SET;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub struct BicycleService {}

#[tonic::async_trait]
impl Bicycle for BicycleService {
    // ##START_HANDLERS##
    async fn get_examples_by_pk(
        &self,
        req: Request<IndexQuery>,
    ) -> Result<Response<proto::Examples>, Status> {
        match bicycle_core::get_examples_by_pk(req.into_inner()) {
            Ok(items) => Ok(Response::new(proto::Examples { examples: items })),
            Err(err) => {
                let msg = format!("failed to GET 'Examples': {}", err.to_string());
                Err(Status::internal(msg))
            }
        }
    }

    async fn delete_examples_by_pk(
        &self,
        req: Request<IndexQuery>,
    ) -> Result<Response<()>, Status> {
        match bicycle_core::delete_examples_by_pk(req.into_inner()) {
            Ok(_) => Ok(Response::new(())),
            Err(err) => {
                let msg = format!("failed to DELETE 'Examples': {}", err.to_string());
                Err(Status::internal(msg))
            }
        }
    }

    async fn put_example(&self, req: Request<proto::Example>) -> Result<Response<()>, Status> {
        if let Err(err) = bicycle_core::put_example(req.into_inner()) {
            let msg = format!("failed to PUT 'Example': {}", err.to_string());

            return Err(Status::internal(msg));
        }

        Ok(Response::new(()))
    }

    async fn batch_put_examples(
        &self,
        req: Request<proto::Examples>,
    ) -> Result<Response<()>, Status> {
        if let Err(err) = bicycle_core::batch_put_examples(req.into_inner()) {
            let msg = format!("failed to BATCH PUT 'Examples': {}", err.to_string());

            return Err(Status::internal(msg));
        }

        Ok(Response::new(()))
    }
    // ##END_HANDLERS##
}

use std::collections::BTreeMap;
use std::fs::{create_dir, read_dir, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::RwLock;

use bicycle_core::exec;

use proto::biplane_server::{Biplane, BiplaneServer};
use proto::{Fn, FnName, Fns, OneOff, Stored};

const SCRIPT_DIR: &'static str = "__bicycle.biplane__";

pub struct BiplaneService {
    functions: RwLock<BTreeMap<String, Vec<u8>>>,
}

impl BiplaneService {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let function_dir = Path::new(SCRIPT_DIR);

        let mut functions = BTreeMap::new();

        if !function_dir.exists() {
            create_dir(function_dir)?;
        } else {
            let paths = read_dir(function_dir)?;

            for path in paths {
                let path = path.unwrap().path();

                let name = path.file_name().unwrap().to_str().unwrap();
                let mut function = vec![];

                File::open(&path)?.read_to_end(&mut function)?;

                functions.insert(name.to_string(), function);
            }
        }

        Ok(Self {
            functions: RwLock::new(functions),
        })
    }
}

#[tonic::async_trait]
impl Biplane for BiplaneService {
    async fn remove(&self, req: Request<FnName>) -> Result<Response<()>, Status> {
        let name = req.into_inner().name;
        let function_dir = Path::new(SCRIPT_DIR);

        remove_file(function_dir.join(&name))?;

        self.functions.write().unwrap().remove(&name);

        Ok(Response::new(()))
    }

    async fn deploy(&self, req: Request<Fn>) -> Result<Response<()>, Status> {
        let Fn { name, function } = req.into_inner();
        let function_dir = Path::new(SCRIPT_DIR);

        let mut file = File::create(function_dir.join(&name))?;
        file.write_all(&function)?;

        self.functions.write().unwrap().insert(name, function);

        Ok(Response::new(()))
    }

    async fn list(&self, _req: Request<()>) -> Result<Response<Fns>, Status> {
        let mut functions = vec![];

        for (name, function) in &*self.functions.read().unwrap() {
            functions.push(Fn {
                name: name.to_string(),
                function: function.clone(),
            })
        }

        Ok(Response::new(Fns { functions }))
    }

    async fn invoke_one_off(
        &self,
        req: Request<OneOff>,
    ) -> Result<Response<prost_types::Value>, Status> {
        let OneOff { function, args } = req.into_inner();

        match exec(&function, &args) {
            Ok(value) => Ok(Response::new(value)),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn invoke_stored(
        &self,
        req: Request<Stored>,
    ) -> Result<Response<prost_types::Value>, Status> {
        let Stored { name, args } = req.into_inner();

        if let Some(function) = self.functions.read().unwrap().get(&name) {
            match exec(function, &args) {
                Ok(value) => Ok(Response::new(value)),
                Err(err) => Err(Status::internal(err.to_string())),
            }
        } else {
            Err(Status::not_found(format!(
                "function with name \"{}\" not found",
                name
            )))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::0]:50051".parse()?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Bicycle Server 🚲 listening at: {}", addr);

    Server::builder()
        .add_service(BicycleServer::new(BicycleService {}))
        .add_service(BiplaneServer::new(BiplaneService::new()?))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
