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

use r2d2_sqlite::rusqlite::Statement;

use r2d2_sqlite::rusqlite;
use r2d2_sqlite::SqliteConnectionManager;

lazy_static! {
    static ref SQLITE_POOL: r2d2::Pool<SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("__bicycle.sqlite__");

        let pool = r2d2::Pool::new(manager).expect("unable to create connection pool");

        let conn = pool.get().expect("unable to get connection from pool");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS records (
            pk TEXT PRIMARY KEY,
            b BLOB NOT NULL
        )",
            (),
        )
        .expect("unable to create 'records' table");

        pool
    };
}

// HELPERS

fn get_from_statement<T>(stmt: &mut Statement, val: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let rows = stmt.query_map(&[val], |row| {
        let v: Vec<u8> = row.get(0)?;
        let res: Result<T, rusqlite::Error> = match prost::Message::decode(&*v) {
            Ok(decoded) => Ok(decoded),
            Err(_) => Err(rusqlite::Error::InvalidQuery),
        };

        res
    })?;

    let mut items = vec![];

    for row in rows {
        if let Ok(v) = row {
            items.push(v)
        }
    }

    Ok(items)
}

// PUT

pub fn put(model: &'static str, k: String, v: Vec<u8>) -> Result<(), Box<dyn Error>> {
    SQLITE_POOL.get()?.execute(
        "INSERT INTO records (pk, b) VALUES (?1, ?2)",
        (&format!("{}#{}", model, k), &v),
    )?;
    Ok(())
}

pub fn batch_put(
    model: &'static str,
    params: Vec<(String, Vec<u8>)>,
) -> Result<(), Box<dyn Error>> {
    let mut conn = SQLITE_POOL.get()?;
    let tx = conn.transaction()?;

    for (k, v) in params {
        tx.execute(
            "INSERT INTO records (pk, b) VALUES (?1, ?2)",
            (&format!("{}#{}", model, k), &v),
        )?;
    }

    tx.commit()?;
    Ok(())
}

// GET

pub fn get_eq<T>(model: &'static str, val: String) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let conn = SQLITE_POOL.get()?;
    let mut stmt = conn.prepare("SELECT b FROM records WHERE pk = ?")?;

    get_from_statement(&mut stmt, &format!("{}#{}", model, val))
}

pub fn get_gte<T>(model: &'static str, val: String) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let conn = SQLITE_POOL.get()?;
    let mut stmt = conn.prepare("SELECT b FROM records WHERE pk >= ?")?;

    get_from_statement(&mut stmt, &format!("{}#{}", model, val))
}

pub fn get_lte<T>(model: &'static str, val: String) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let conn = SQLITE_POOL.get()?;
    let mut stmt = conn.prepare("SELECT b FROM records WHERE pk <= ?")?;

    get_from_statement(&mut stmt, &format!("{}#{}", model, val))
}

pub fn get_begins_with<T>(model: &'static str, val: String) -> Result<Vec<T>, Box<dyn Error>>
where
    T: prost::Message + Default,
{
    let conn = SQLITE_POOL.get()?;
    let mut stmt = conn.prepare("SELECT b FROM records WHERE pk LIKE ?")?;

    get_from_statement(&mut stmt, &format!("{}#{}%", model, val))
}

// DELETE

pub fn delete_eq(model: &'static str, val: String) -> Result<(), Box<dyn Error>> {
    SQLITE_POOL.get()?.execute(
        "DELETE FROM records WHERE pk = ?",
        &[&format!("{}#{}", model, val)],
    )?;
    Ok(())
}

pub fn delete_gte(model: &'static str, val: String) -> Result<(), Box<dyn Error>> {
    SQLITE_POOL.get()?.execute(
        "DELETE FROM records WHERE pk >= ?",
        &[&format!("{}#{}", model, val)],
    )?;
    Ok(())
}

pub fn delete_lte(model: &'static str, val: String) -> Result<(), Box<dyn Error>> {
    SQLITE_POOL.get()?.execute(
        "DELETE FROM records WHERE pk <= ?",
        &[&format!("{}#{}", model, val)],
    )?;
    Ok(())
}

pub fn delete_begins_with(model: &'static str, val: String) -> Result<(), Box<dyn Error>> {
    SQLITE_POOL.get()?.execute(
        "DELETE FROM records WHERE pk LIKE ?",
        &[&format!("{}#{}%", model, val)],
    )?;
    Ok(())
}
