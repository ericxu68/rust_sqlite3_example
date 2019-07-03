use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
// use std::time::Timespec;
extern crate chrono;
use chrono::prelude::*;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    created_at: String,
    data: Option<Vec<u8>>,
}

pub fn run_sql() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      created_at TEXT NOT NULL,
      data BLOB)",
        NO_PARAMS,
    )?;

    let me = Person {
        id: 1,
        name: "Steven".to_string(),
        created_at: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, created_at, data) VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.created_at, &me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, created_at, data FROM person")?;
    let person_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    for person in person_iter {
        println!("Fround person {:?}", person.unwrap());
    }
    Ok(())
}
