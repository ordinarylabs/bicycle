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
    itr: &mut DBIteratorWithThreadMode<'a, D>,
) -> Result<Vec<T>, tonic::Status>
where
    D: DBAccess,
    T: prost::Message + Default,
{
    let mut items = vec![];

    while let Some(Ok((_, v))) = itr.next() {
        if let Ok(item) = prost::Message::decode(&*v) {
            items.push(item);
        }
    }

    Ok(items)
}

fn handle_delete_itr<'a, D>(itr: &mut DBIteratorWithThreadMode<'a, D>) -> Result<(), tonic::Status>
where
    D: DBAccess,
{
    let mut batch = WriteBatch::default();

    while let Some(Ok((k, ..))) = itr.next() {
        if let Ok(key) = from_utf8(&*k) {
            batch.delete(key);
        }
    }

    match ROCKSDB.write(batch) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::internal(err.to_string())),
    }
}

// PUT

pub fn put(k: String, v: Vec<u8>) -> Result<(), tonic::Status> {
    match ROCKSDB.put(k.as_bytes(), v) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

pub fn batch_put(params: Vec<(String, Vec<u8>)>) -> Result<(), tonic::Status> {
    let mut batch = WriteBatch::default();

    for (k, v) in params {
        batch.put(k.as_bytes(), v);
    }

    match ROCKSDB.write(batch) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

// GET

pub fn get_eq<T>(val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    match ROCKSDB.get(val.as_bytes()) {
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

pub fn get_gte<T>(val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

    handle_get_itr(&mut itr)
}

pub fn get_lte<T>(val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Reverse));

    handle_get_itr(&mut itr)
}

pub fn get_begins_with<T>(val: String) -> Result<Vec<T>, tonic::Status>
where
    T: prost::Message + Default,
{
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

pub fn delete_eq(val: String) -> Result<(), tonic::Status> {
    match ROCKSDB.delete(val.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(tonic::Status::aborted(err.to_string())),
    }
}

pub fn delete_gte(val: String) -> Result<(), tonic::Status> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Forward));

    handle_delete_itr(&mut itr)
}

pub fn delete_lte(val: String) -> Result<(), tonic::Status> {
    let mut itr = ROCKSDB.iterator(IteratorMode::From(val.as_bytes(), Direction::Reverse));

    handle_delete_itr(&mut itr)
}

pub fn delete_begins_with(val: String) -> Result<(), tonic::Status> {
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
