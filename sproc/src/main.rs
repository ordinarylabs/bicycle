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

use bicycle_sproc::{SprocServer, SprocService, FILE_DESCRIPTOR_SET};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::0]:50051".parse()?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("SPROC Server 🚀 listening at: {}", addr);

    Server::builder()
        .add_service(SprocServer::new(SprocService::new()?))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
