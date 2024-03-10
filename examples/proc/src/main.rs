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

use host::prost_types::{value::Kind, ListValue, Value};
use host::{get_input, set_output};

use host::models::example;
use host::proto::{index_query::Expression, Examples, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let val: Option<Value> = get_input()?;

    let val = match val {
        Some(val) => match val.kind {
            Some(Kind::StringValue(val)) => val,
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let Examples { examples } = example::get_by_pk(IndexQuery {
        expression: Some(Expression::BeginsWith(val)),
    })?;

    let pks = examples
        .into_iter()
        .map(|example| Value {
            kind: Some(Kind::StringValue(example.pk)),
        })
        .collect::<Vec<Value>>();

    set_output(Some(Value {
        kind: Some(Kind::ListValue(ListValue { values: pks })),
    }))?;

    Ok(())
}
