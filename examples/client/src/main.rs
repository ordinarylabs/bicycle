/*
BicycleDB is a protobuf-defined database management system.

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

use bicycle_core;
use bicycle_core::proto::{
    bicycle_client::BicycleClient, index_query::Expression, Dog, IndexQuery,
};
use bicycle_core::tonic::Request;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // establish connection to remote Bicycle server
    let mut client = BicycleClient::connect("http://0.0.0.0:50051").await?;

    // write a dog to remote Bicycle server
    client
        .put_dog(Request::new(Dog {
            pk: "4".to_string(),
            name: "Sam".to_string(),
            age: 6,
            breed: "Labrador".to_string(),
        }))
        .await?;

    // get dog back from remote Bicycle server
    let dogs = client
        .get_dogs_by_pk(Request::new(IndexQuery {
            expression: Some(Expression::Eq("4".to_string())),
        }))
        .await?;

    println!("{:#?}", dogs.into_inner());

    Ok(())
}
