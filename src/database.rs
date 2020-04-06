use super::generator::Spacebar;

use log::*;
use rusqlite::{params, NO_PARAMS, Connection};

fn ensure_integrity(conn: &Connection) {
    let mut statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='spacebars'").unwrap();
    let _ = match statement.query(NO_PARAMS) {
        Ok(mut o) => {
            if o.next().unwrap().is_none() {
                conn.execute("CREATE TABLE spacebars (id SERIAL PRIMARY KEY, spacebar INTEGER, name TEXT NOT NULL, description TEXT)", NO_PARAMS).unwrap();
            }
        },
        Err(_) => {
            error!("Database integrity check failed.");
            std::process::exit(1);
        }
    };
}

pub fn connect(path: &str) -> Option<Connection> {
    match Connection::open(path) {
        Ok(o) =>  {
            ensure_integrity(&o);
            return Some(o);
        },
        Err(_) => {
            error!("Failed to connect to database: {}", path);
            return None;
        },
    };
}

pub fn insert_spacebar(conn: &Connection, spacebar: Spacebar) {
    conn.execute("INSERT INTO spacebars (spacebar, name, description) VALUES ($1, $2, $3)", params![spacebar.spacebar, spacebar.name, spacebar.description]).unwrap();
}

pub fn update_spacebar(conn: &Connection, spacebar: Spacebar) {
    unimplemented!();
}

pub fn delete_spacebar(conn: &Connection, spacebar: Spacebar) {
    match conn.execute("DELETE FROM spacebars WHERE spacebar = $1", params![spacebar.spacebar]) {
        Ok(o) => info!("Deleted {} spacebar(s)", o),
        Err(_) => error!("Failed to delete spacebar."),
    }
}

pub fn select_spacebar(conn: &Connection, spacebar: Spacebar) -> Option<Spacebar> {
    //SELECT * FROM spacebars WHERE spacebar = $1
    let mut statement = conn.prepare("SELECT * FROM spacebars WHERE spacebar=$1").unwrap();
    let mut spacebars_iter = match statement.query_map(params![spacebar.spacebar], |row| {
        Ok(Spacebar {
            spacebar: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            description: row.get(3).unwrap(),
        })
    }) {
            Ok(o) => o,
            Err(_) => {
                error!("Failed to parse database response.");
                std::process::exit(1);
            },
        };
    if spacebars_iter.next().is_some() {
        return match spacebars_iter.next().unwrap() {
            Ok(o) => Some(o),
            Err(_) => None,
        }
    } else {
        return None;
    }
}
