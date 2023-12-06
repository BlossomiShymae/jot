use miette::miette;
use rusqlite::Connection;
use std::{fmt::Display, str};

#[derive(Debug, Default)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub body: String,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

pub fn execute_schema(conn: &Connection) -> Result<(), miette::ErrReport> {
    let schema =
        "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, body TEXT NOT NULL)";

    conn.execute(schema, ())
        .map(|_| ())
        .map_err(|e| miette!("{}", e.to_string()))
}

pub fn insert(conn: &Connection, note: &Note) -> Result<(), miette::ErrReport> {
    conn.execute(
        "INSERT INTO notes (name, body) VALUES (?1, ?2)",
        (&note.name, &note.body),
    )
    .map(|_| ())
    .map_err(|e| miette!("{}", e.to_string()))
}

pub fn select(conn: &Connection, name: &str) -> Result<Note, miette::ErrReport> {
    let mut stmt = conn
        .prepare("SELECT id, name, body FROM notes WHERE name = :name")
        .map_err(|e| miette!("{}", e.to_string()))?;
    let rows = stmt
        .query_map(&[(":name", &name)], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                body: row.get(2)?,
            })
        })
        .map_err(|e: rusqlite::Error| miette!("{}", e.to_string()))?;
    let mut notes = Vec::new();
    for result in rows {
        notes.push(result.map_err(|e| miette!("{}", e.to_string()))?);
    }

    if notes.len() == 0 {
        return Err(miette!("{}", "Note not found!"));
    }

    Ok(notes.into_iter().next().unwrap())
}

pub fn select_all(conn: &Connection) -> Result<Vec<Note>, miette::ErrReport> {
    let mut stmt = conn
        .prepare("SELECT id, name, body FROM notes")
        .map_err(|e| miette!("{}", e.to_string()))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                body: row.get(2)?,
            })
        })
        .map_err(|e| miette!("{}", e.to_string()))?;

    let mut notes = Vec::new();
    for result in rows {
        notes.push(result.map_err(|e| miette!("{}", e.to_string()))?);
    }

    Ok(notes)
}

pub fn update(conn: &Connection, note: &mut Note) -> Result<(), miette::ErrReport> {
    conn.execute(
        "UPDATE notes SET body = ?1 WHERE name = ?2",
        [&note.body, &note.name],
    )
    .map(|_| ())
    .map_err(|e| miette!("{}", e.to_string()))
}

pub fn delete(conn: &Connection, name: &str) -> Result<(), miette::ErrReport> {
    conn.execute("DELETE FROM notes WHERE name = ?1", [&name])
        .map(|_| ())
        .map_err(|e| miette!("{}", e.to_string()))
}
