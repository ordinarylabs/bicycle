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

use tonic::transport::Server;

use bicycle_core::proto;

use proto::bicycle_server::BicycleServer;
use proto::FILE_DESCRIPTOR_SET;

mod bicycle;
use bicycle::BicycleService;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:50051".parse()?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let bicycle_service = BicycleService {};

    println!("Bicycle Server 🚲 listening at: {}", addr);

    // TODO: add tracing and logs (`tracing` crate and `tower` middleware for logging)
    // TODO: make all instrumentation OTel compatible

    Server::builder()
        .add_service(reflection_service)
        .add_service(BicycleServer::new(bicycle_service))
        .serve(addr)
        .await?;

    Ok(())
}
