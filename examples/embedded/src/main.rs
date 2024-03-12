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

use bicycle_core;
use bicycle_core::proto::{index_query::Expression, Dog, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // write a dog to local Bicycle
    bicycle_core::put_dog(Dog {
        pk: "0".to_string(),
        name: "Ollie".to_string(),
        age: 7,
        breed: "Pitty".to_string(),
    })?;

    // get that dog back from local Bicycle
    let dogs = bicycle_core::get_dogs_by_pk(IndexQuery {
        expression: Some(Expression::Eq("0".to_string())),
    })?;

    println!("{:#?}", dogs);

    Ok(())
}
