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

use std::error::Error;

use prost::Message;

use bicycle_proto::{index_query::Expression, IndexQuery};

use engine::{
    batch_put, delete_begins_with, delete_eq, delete_gte, delete_lte, get_begins_with, get_eq,
    get_gte, get_lte, put,
};

const MODEL_NAME: &'static str = "EXAMPLE";

pub fn get_examples_by_pk(
    query: IndexQuery,
) -> Result<Vec<bicycle_proto::Example>, Box<dyn Error>> {
    if let Some(expression) = query.expression {
        match expression {
            Expression::Eq(val) => get_eq::<bicycle_proto::Example>(MODEL_NAME, val),
            Expression::Gte(val) => get_gte::<bicycle_proto::Example>(MODEL_NAME, val),
            Expression::Lte(val) => get_lte::<bicycle_proto::Example>(MODEL_NAME, val),
            Expression::BeginsWith(val) => {
                get_begins_with::<bicycle_proto::Example>(MODEL_NAME, val)
            }
        }
    } else {
        Err("no expression provided".into())
    }
}

pub fn delete_examples_by_pk(query: IndexQuery) -> Result<(), Box<dyn Error>> {
    if let Some(expression) = query.expression {
        match expression {
            Expression::Eq(val) => delete_eq(MODEL_NAME, val),
            Expression::Gte(val) => delete_gte(MODEL_NAME, val),
            Expression::Lte(val) => delete_lte(MODEL_NAME, val),
            Expression::BeginsWith(val) => delete_begins_with(MODEL_NAME, val),
        }
    } else {
        Err("no expression provided".into())
    }
}

#[inline(always)]
pub fn put_example(example: bicycle_proto::Example) -> Result<(), Box<dyn Error>> {
    put(MODEL_NAME, example.pk.clone(), example.encode_to_vec())
}

#[inline]
pub fn batch_put_examples(examples: bicycle_proto::Examples) -> Result<(), Box<dyn Error>> {
    let mut params = vec![];

    for example in examples.examples {
        params.push((example.pk.clone(), example.encode_to_vec()));
    }

    batch_put(MODEL_NAME, params)
}
