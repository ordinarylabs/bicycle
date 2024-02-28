/*
Bicycle is a database database framework.

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

use std::str::from_utf8;

use rocksdb::{
    DBAccess, DBIteratorWithThreadMode, Direction, IteratorMode, Options, WriteBatch, DB,
};

lazy_static! {
    pub(crate) static ref ROCKSDB: DB = {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        DB::open(&opts, "__bicycle__").unwrap()
    };
}

// HELPERS

fn handle_get_itr<'a, D, T>(
    model: &'static str,
    itr: &mut DBIteratorWithThreadMode<'a, D>,
) -> Result<Vec<T>, tonic::Status>
where
    D: DBAccess,
    T: prost::Message + Default,
{
    let model_str = &format!("{}#", model);
    let mut items = vec![];

    while let Some(Ok((k, v))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            if key.starts_with(model_str) {
                if let Ok(item) = prost::Message::decode(&*v) {
                    items.push(item);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    Ok(items)
}

fn handle_delete_itr<'a, D>(
    model: &'static str,
    itr: &mut DBIteratorWithThreadMode<'a, D>,
) -> Result<(), tonic::Status>
where
    D: DBAccess,
{
    let model_str = &format!("{}#", model);
    let mut batch = WriteBatch::default();

    while let Some(Ok((k, ..))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            if key.starts_with(model_str) {
                batch.delete(key);
            } else {
                break;
            }
        }
    }

    match ROCKSDB.write(batch) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::internal(err.to_string())),
    }
}

// PUT

pub fn put(model: &'static str, k: String, v: Vec<u8>) -> Result<(), tonic::Status> {
    match ROCKSDB.put(format!("{}#{}", model, k).as_bytes(), v) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

pub fn batch_put(model: &'static str, params: Vec<(String, Vec<u8>)>) -> Result<(), tonic::Status> {
    let mut batch = WriteBatch::default();

    for (k, v) in params {
        batch.put(format!("{}#{}", model, k).as_bytes(), v);
    }

    match ROCKSDB.write(batch) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

// GET

pub fn get_eq<T>(model: &'static str, val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    match ROCKSDB.get(format!("{}#{}", model, val).as_bytes()) {
        Ok(res) => {
            if let Some(res) = res {
                match prost::Message::decode(&res[..]) {
                    Ok(decoded) => return Ok(vec![decoded]),
                    Err(err) => return Err(tonic::Status::internal(err.to_string())),
                }
            } else {
                return Ok(vec![]);
            }
        }
        Err(err) => return Err(tonic::Status::internal(err.to_string())),
    }
}

pub fn get_gte<T>(model: &'static str, val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Forward,
    ));

    handle_get_itr(model, &mut itr)
}

pub fn get_lte<T>(model: &'static str, val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Reverse,
    ));

    handle_get_itr(model, &mut itr)
}

pub fn get_begins_with<T>(model: &'static str, val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    let val = format!("{}#{}", model, val);

    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

    let mut items = vec![];

    while let Some(Ok((k, v))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            if key.starts_with(&val) {
                if let Ok(item) = prost::Message::decode(&*v) {
                    items.push(item);
                }
            } else {
                break;
            }
        }
    }

    Ok(items)
}

// DELETE

pub fn delete_eq(model: &'static str, val: String) -> Result<(), tonic::Status> {
    match ROCKSDB.delete(format!("{}#{}", model, val).as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

pub fn delete_gte(model: &'static str, val: String) -> Result<(), tonic::Status> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Forward,
    ));

    handle_delete_itr(model, &mut itr)
}

pub fn delete_lte(model: &'static str, val: String) -> Result<(), tonic::Status> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Reverse,
    ));

    handle_delete_itr(model, &mut itr)
}

pub fn delete_begins_with(model: &'static str, val: String) -> Result<(), tonic::Status> {
    let val = format!("{}#{}", model, val);

    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

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

    match ROCKSDB.write(batch) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}
