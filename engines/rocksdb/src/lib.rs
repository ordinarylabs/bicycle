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

#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::str::from_utf8;

use rocksdb::{
    DBAccess, DBIteratorWithThreadMode, Direction, IteratorMode, Options, WriteBatch, DB,
};

use log::{error, info};

lazy_static! {
    static ref ROCKSDB: DB = {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        DB::open(&opts, "__bicycle.engine.rocksdb__").expect("unable to open RocksDB")
    };
}

// HELPERS

fn handle_get_itr<'a, D, T>(
    model: &'static str,
    itr: &mut DBIteratorWithThreadMode<'a, D>,
) -> Vec<T>
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
                    error!("failed to decode record");
                    break;
                }
            } else {
                break;
            }
        }
    }

    items
}

fn handle_delete_itr<'a, D>(
    model: &'static str,
    itr: &mut DBIteratorWithThreadMode<'a, D>,
) -> Result<(), Box<dyn Error>>
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

    ROCKSDB.write(batch)?;
    Ok(())
}

// PUT

pub fn put(model: &'static str, k: String, v: Vec<u8>) -> Result<(), Box<dyn Error>> {
    ROCKSDB.put(format!("{}#{}", model, k).as_bytes(), v)?;
    info!("put {}", model);
    Ok(())
}

pub fn batch_put(
    model: &'static str,
    params: Vec<(String, Vec<u8>)>,
) -> Result<(), Box<dyn Error>> {
    let mut batch = WriteBatch::default();

    for (k, v) in params {
        batch.put(format!("{}#{}", model, k).as_bytes(), v);
    }

    ROCKSDB.write(batch)?;
    info!("batch_put {}", model);
    Ok(())
}

// GET

pub fn get_eq<T>(model: &'static str, val: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let res = ROCKSDB.get(format!("{}#{}", model, val).as_bytes())?;

    if let Some(res) = res {
        let decoded = prost::Message::decode(&res[..])?;
        info!("get_eq {}", model);
        Ok(vec![decoded])
    } else {
        info!("get_eq {}", model);
        Ok(vec![])
    }
}

pub fn get_gte<T>(model: &'static str, val: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Forward,
    ));

    let res = handle_get_itr(model, &mut itr);
    info!("get_gte {}", model);

    Ok(res)
}

pub fn get_lte<T>(model: &'static str, val: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Reverse,
    ));

    let res = handle_get_itr(model, &mut itr);
    info!("get_lte {}", model);

    Ok(res)
}

pub fn get_begins_with<T>(model: &'static str, val: &str) -> Result<Vec<T>, Box<dyn Error>>
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
                } else {
                    error!("failed to decode record");
                    break;
                }
            } else {
                break;
            }
        }
    }

    info!("get_begins_with {}", model);

    Ok(items)
}

// DELETE

pub fn delete_eq(model: &'static str, val: &str) -> Result<(), Box<dyn Error>> {
    ROCKSDB.delete(format!("{}#{}", model, val).as_bytes())?;
    info!("delete_eq {}", model);
    Ok(())
}

pub fn delete_gte(model: &'static str, val: &str) -> Result<(), Box<dyn Error>> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Forward,
    ));

    handle_delete_itr(model, &mut itr)?;
    info!("delete_gte {}", model);
    Ok(())
}

pub fn delete_lte(model: &'static str, val: &str) -> Result<(), Box<dyn Error>> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(
        format!("{}#{}", model, val).as_bytes(),
        Direction::Reverse,
    ));

    handle_delete_itr(model, &mut itr)?;
    info!("delete_lte {}", model);
    Ok(())
}

pub fn delete_begins_with(model: &'static str, val: &str) -> Result<(), Box<dyn Error>> {
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

    ROCKSDB.write(batch)?;
    info!("delete_begins_with {}", model);
    Ok(())
}
