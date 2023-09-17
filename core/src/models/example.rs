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

use prost::Message;

use crate::proto;
use proto::{index_query::Expression, IndexQuery};

use crate::engine::{
    batch_put, delete_begins_with, delete_eq, delete_gte, delete_lte, get_begins_with, get_eq,
    get_gte, get_lte, put,
};

const MODEL_NAME: &'static str = "EXAMPLE";

pub fn get_examples_by_pk(query: IndexQuery) -> Result<Vec<proto::Example>, tonic::Status> {
    if let Some(expression) = query.expression {
        match expression {
            Expression::Eq(val) => {
                return get_eq::<proto::Example>(format!("{}#{}", MODEL_NAME, val))
            }
            Expression::Gte(val) => {
                return get_gte::<proto::Example>(format!("{}#{}", MODEL_NAME, val))
            }
            Expression::Lte(val) => {
                return get_lte::<proto::Example>(format!("{}#{}", MODEL_NAME, val))
            }
            Expression::BeginsWith(val) => {
                return get_begins_with::<proto::Example>(format!("{}#{}", MODEL_NAME, val))
            }
        }
    }

    Err(tonic::Status::internal("invalid expression"))
}

pub fn delete_examples_by_pk(query: IndexQuery) -> Result<(), tonic::Status> {
    if let Some(expression) = query.expression {
        match expression {
            Expression::Eq(val) => return delete_eq(format!("{}#{}", MODEL_NAME, val)),
            Expression::Gte(val) => return delete_gte(format!("{}#{}", MODEL_NAME, val)),
            Expression::Lte(val) => return delete_lte(format!("{}#{}", MODEL_NAME, val)),
            Expression::BeginsWith(val) => {
                return delete_begins_with(format!("{}#{}", MODEL_NAME, val))
            }
        }
    }

    Err(tonic::Status::internal("invalid expression"))
}

pub fn put_example(example: proto::Example) -> Result<(), tonic::Status> {
    put(
        format!("{}#{}", MODEL_NAME, example.pk.clone()),
        example.encode_to_vec(),
    )
}

pub fn batch_put_examples(examples: proto::Examples) -> Result<(), tonic::Status> {
    let mut params = vec![];

    for example in examples.examples {
        params.push((
            format!("{}#{}", MODEL_NAME, example.pk.clone()),
            example.encode_to_vec(),
        ));
    }

    batch_put(params)
}
