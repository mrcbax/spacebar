use super::generator::Spacebar;

use log::*;
use rusqlite::{params, NO_PARAMS, Connection, OpenFlags};

fn ensure_integrity(conn: &Connection) {
    let mut statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='spacebars'").unwrap();
    let _ = match statement.query(NO_PARAMS) {
        Ok(mut o) => {
            if o.next().unwrap().is_none() {
                conn.execute("CREATE TABLE spacebars (id SERIAL PRIMARY KEY, spacebar INTEGER, name TEXT NOT NULL, description TEXT)", NO_PARAMS).unwrap();
            }
        },
        Err(e) => {
            error!("Database integrity check failed.");
            debug!("{}", e);
            std::process::exit(1);
        }
    };
}

pub fn connect(path: &str) -> Option<Connection> {
    match Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE) {
        Ok(o) =>  {
            ensure_integrity(&o);
            return Some(o);
        },
        Err(e) => {
            error!("Could not find database. Try running init.");
            debug!("{}", e);
            return None;
        },
    };
}

pub fn insert_spacebar(conn: &Connection, spacebar: Spacebar) {
    if spacebar.description.is_some() {
        conn.execute("INSERT INTO spacebars (spacebar, name, description) VALUES ($1, $2, $3)", params![spacebar.spacebar, spacebar.name, spacebar.description.unwrap()]).unwrap();
    } else {
        conn.execute("INSERT INTO spacebars (spacebar, name) VALUES ($1, $2)", params![spacebar.spacebar, spacebar.name]).unwrap();
    }
}

pub fn update_spacebar(conn: &Connection, spacebar: Spacebar) {
    unimplemented!();
}

pub fn delete_spacebar(conn: &Connection, spacebar: Spacebar) {
    match conn.execute("DELETE FROM spacebars WHERE spacebar = $1", params![spacebar.spacebar]) {
        Ok(o) => info!("Deleted {} spacebar(s)", o),
        Err(e) => {
            error!("Failed to delete spacebar.");
            debug!("{}", e);
        },
    }
}

pub fn select_spacebar(conn: &Connection, spacebar: Spacebar) -> Option<Spacebar> {
    //SELECT * FROM spacebars WHERE spacebar = $1
    let mut statement = conn.prepare("SELECT * FROM spacebars WHERE spacebar=$1").unwrap();
    let mut spacebars_iter = match statement.query_map(params![spacebar.spacebar], |row| {
        Ok(Spacebar {
            spacebar: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            description: Some(row.get(3).unwrap()),
        })
    }) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to parse database response.");
                debug!("{}", e);
                std::process::exit(1);
            },
        };
    if spacebars_iter.next().is_some() {
        match spacebars_iter.next().unwrap() {
            Ok(o) => return Some(o),
            Err(e) => {
                debug!("{}", e);
                return None;
            },
        }
    } else {
        return None;
    }
}
