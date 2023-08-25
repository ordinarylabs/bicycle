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
use rocksdb::{DBAccess, DBIteratorWithThreadMode, Direction, Error, IteratorMode, WriteBatch};
use std::str::from_utf8;

use crate::proto;
use proto::{index_query::Expression, IndexQuery};

use crate::ROCKSDB_CONNECTION;

// TODO: this needs to switch to use column families instead of string prefix
const MODEL_PREFIX: &'static str = "EXAMPLE#";

fn handle_gt_lt_get_itr<'a, D>(itr: &mut DBIteratorWithThreadMode<'a, D>) -> Vec<proto::Example>
where
    D: DBAccess,
{
    let mut items = vec![];

    while let Some(Ok((k, v))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            if key.starts_with(MODEL_PREFIX) {
                if let Ok(item) = proto::Example::decode(&*v) {
                    items.push(item);
                }
            } else {
                break;
            }
        }
    }

    items
}

fn handle_gt_lt_delete_itr<'a, D>(itr: &mut DBIteratorWithThreadMode<'a, D>)
where
    D: DBAccess,
{
    let mut batch = WriteBatch::default();

    while let Some(Ok((k, ..))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            if key.starts_with(MODEL_PREFIX) {
                batch.delete(key);
            } else {
                break;
            }
        }
    }

    match ROCKSDB_CONNECTION.write(batch) {
        Ok(_) => (),
        Err(err) => println!("{}", err.to_string()),
    };
}

fn get_examples_from_query(
    query: IndexQuery,
    should_delete: bool,
) -> Result<proto::Examples, Error> {
    let examples = match query.expression {
        Some(exp) => match exp {
            Expression::Eq(v) => {
                let val = format!("{}{}", MODEL_PREFIX, v);

                if should_delete {
                    match ROCKSDB_CONNECTION.delete(val.as_bytes()) {
                        Ok(_) => (),
                        Err(err) => println!("{}", err.to_string()),
                    };

                    vec![]
                } else {
                    if let Ok(Some(v)) = ROCKSDB_CONNECTION.get(val.as_bytes()) {
                        if let Ok(example) = proto::Example::decode(&v[..]) {
                            vec![example]
                        } else {
                            eprintln!("Failed to decode 'Example' from DB");
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                }
            }
            Expression::Gte(v) => {
                let val = format!("{}{}", MODEL_PREFIX, v);

                let mut itr = ROCKSDB_CONNECTION
                    .iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

                if should_delete {
                    handle_gt_lt_delete_itr(&mut itr);
                    vec![]
                } else {
                    handle_gt_lt_get_itr(&mut itr)
                }
            }
            Expression::Lte(v) => {
                let val = format!("{}{}", MODEL_PREFIX, v);

                let mut itr = ROCKSDB_CONNECTION
                    .iterator(IteratorMode::From(val.as_bytes(), Direction::Reverse));

                if should_delete {
                    handle_gt_lt_delete_itr(&mut itr);
                    vec![]
                } else {
                    handle_gt_lt_get_itr(&mut itr)
                }
            }
            Expression::BeginsWith(v) => {
                let val = format!("{}{}", MODEL_PREFIX, v);

                let mut itr = ROCKSDB_CONNECTION
                    .iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

                let mut items = vec![];

                if should_delete {
                    let mut batch = WriteBatch::default();

                    while let Some(Ok((k, ..))) = itr.next() {
                        if let Ok(key) = from_utf8(&*k) {
                            if key.starts_with(&val) {
                                batch.delete(key)
                            } else {
                                break;
                            }
                        }
                    }

                    ROCKSDB_CONNECTION.write(batch)?;
                } else {
                    while let Some(Ok((k, v))) = itr.next() {
                        if let Ok(key) = from_utf8(&*k) {
                            if key.starts_with(&val) {
                                if let Ok(item) = proto::Example::decode(&*v) {
                                    items.push(item);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }

                items
            }
        },
        None => return Ok(proto::Examples { examples: vec![] }),
    };

    Ok(proto::Examples { examples })
}

pub fn get_examples_by_pk(query: IndexQuery) -> Result<proto::Examples, Error> {
    get_examples_from_query(query, false)
}

pub fn delete_examples_by_pk(query: IndexQuery) -> Result<proto::Examples, Error> {
    get_examples_from_query(query, true)
}

pub fn put_example(example: proto::Example) -> Result<(), Error> {
    let k = format!("{}{}", MODEL_PREFIX, example.pk.clone());
    let v = example.encode_to_vec();

    ROCKSDB_CONNECTION.put(k.as_bytes(), v)
}

pub fn batch_put_examples(examples: proto::Examples) -> Result<(), Error> {
    let mut batch = WriteBatch::default();

    for example in examples.examples {
        let k = format!("{}{}", MODEL_PREFIX, example.pk.clone());
        let v = example.encode_to_vec();

        batch.put(k.as_bytes(), v);
    }

    ROCKSDB_CONNECTION.write(batch)
}
