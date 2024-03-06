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

use prost::Message;

use crate::proto;
use proto::{index_query::Expression, IndexQuery};

use engine::{
    batch_put, delete_begins_with, delete_eq, delete_gte, delete_lte, get_begins_with, get_eq,
    get_gte, get_lte, put,
};

const MODEL_NAME: &'static str = "EXAMPLE";

pub fn get_examples_by_pk(query: IndexQuery) -> Result<Vec<proto::Example>, tonic::Status> {
    if let Some(expression) = query.expression {
        let res = match expression {
            Expression::Eq(val) => get_eq::<proto::Example>(MODEL_NAME, val),
            Expression::Gte(val) => get_gte::<proto::Example>(MODEL_NAME, val),
            Expression::Lte(val) => get_lte::<proto::Example>(MODEL_NAME, val),
            Expression::BeginsWith(val) => get_begins_with::<proto::Example>(MODEL_NAME, val),
        };

        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(tonic::Status::internal(err.to_string())),
        }
    } else {
        Err(tonic::Status::internal("invalid expression"))
    }
}

pub fn delete_examples_by_pk(query: IndexQuery) -> Result<(), tonic::Status> {
    if let Some(expression) = query.expression {
        let res = match expression {
            Expression::Eq(val) => delete_eq(MODEL_NAME, val),
            Expression::Gte(val) => delete_gte(MODEL_NAME, val),
            Expression::Lte(val) => delete_lte(MODEL_NAME, val),
            Expression::BeginsWith(val) => delete_begins_with(MODEL_NAME, val),
        };

        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(tonic::Status::internal(err.to_string())),
        }
    } else {
        Err(tonic::Status::internal("invalid expression"))
    }
}

pub fn put_example(example: proto::Example) -> Result<(), tonic::Status> {
    match put(MODEL_NAME, example.pk.clone(), example.encode_to_vec()) {
        Ok(res) => Ok(res),
        Err(err) => Err(tonic::Status::internal(err.to_string())),
    }
}

pub fn batch_put_examples(examples: proto::Examples) -> Result<(), tonic::Status> {
    let mut params = vec![];

    for example in examples.examples {
        params.push((example.pk.clone(), example.encode_to_vec()));
    }

    match batch_put(MODEL_NAME, params) {
        Ok(res) => Ok(res),
        Err(err) => Err(tonic::Status::internal(err.to_string())),
    }
}
