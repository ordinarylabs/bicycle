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
use bicycle::prost_types::{value::Kind, ListValue, Value};
use bicycle::proto::{index_query::Expression, Examples, IndexQuery};
use bicycle::{recv_in, send_out};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let val: Option<Value> = recv_in()?;

    let val = match val {
        Some(val) => match val.kind {
            Some(Kind::StringValue(val)) => val,
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let Examples { examples } = bicycle::get_examples_by_pk(IndexQuery {
        expression: Some(Expression::BeginsWith(val)),
    })?;

    let pks = examples
        .into_iter()
        .map(|example| Value {
            kind: Some(Kind::StringValue(example.pk)),
        })
        .collect::<Vec<Value>>();

    send_out(Some(Value {
        kind: Some(Kind::ListValue(ListValue { values: pks })),
    }))?;

    Ok(())
}
