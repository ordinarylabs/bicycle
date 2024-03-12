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

use bicycle_shims;
use bicycle_shims::prost_types::{value::Kind, ListValue, Value};
use bicycle_shims::proto::{index_query::Expression, Examples, IndexQuery};
use bicycle_shims::{recv_in, send_out};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let val: Option<Value> = recv_in()?;

    let mut begins_with = "".to_string();

    if let Some(Value {
        kind: Some(Kind::StructValue(struct_val)),
    }) = val
    {
        if let Some(Kind::StringValue(val)) = struct_val
            .fields
            .get("begins_with")
            .map(|v| v.kind.as_ref())
            .flatten()
        {
            begins_with = val.clone()
        }
    }

    let Examples { examples } = bicycle_shims::get_examples_by_pk(IndexQuery {
        expression: Some(Expression::BeginsWith(begins_with)),
    })?;

    let pks = examples
        .into_iter()
        .map(|example| Value {
            kind: Some(Kind::StringValue(example.pk)),
        })
        .collect::<Vec<Value>>();

    send_out(Some(Value {
        kind: Some(Kind::ListValue(ListValue { values: pks })),
    }))
}
