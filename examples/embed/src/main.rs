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

use bicycle;
use bicycle::proto::{index_query::Expression, Example, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    bicycle::put_example(Example {
        pk: "SOME_STR".to_string(),
    })?;

    let examples = bicycle::get_examples_by_pk(IndexQuery {
        expression: Some(Expression::Eq("SOME_STR".to_string())),
    })?;

    println!("{:#?}", examples);

    Ok(())
}
