use super::generator::Spacebar;
use super::parser::*;

use log::*;
use rusqlite::{params, NO_PARAMS, Connection, OpenFlags};
use prettytable::Table;

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

#[allow(unused)] //for future use.
pub fn update_spacebar(conn: &Connection, spacebar: Spacebar) {
    unimplemented!();
}

#[allow(unused)] //for future use.
pub fn delete_spacebar(conn: &Connection, spacebar: Spacebar) {
    match conn.execute("DELETE FROM spacebars WHERE spacebar = $1", params![spacebar.spacebar]) {
        Ok(o) => info!("Deleted {} spacebar(s)", o),
        Err(e) => {
            error!("Failed to delete spacebar.");
            debug!("{}", e);
        },
    }
}

pub fn select_spacebar(conn: &Connection, spacebar: i64) -> Option<Spacebar> {
    //SELECT * FROM spacebars WHERE spacebar = $1
    let mut statement = conn.prepare("SELECT * FROM spacebars WHERE spacebar=$1").unwrap();
    let response = statement.query(params![spacebar]);
    let spacebar: Option<Spacebar> = match response {
        Ok(mut rows) => {
            match rows.next() {
                Ok(row) => {
                    if row.is_some() {
                        Some(Spacebar {
                            spacebar: row.unwrap().get(1).unwrap(),
                            name: row.unwrap().get(2).unwrap(),
                            description: match row.unwrap().get(3) {
                                Ok(o) => Some(o),
                                Err(_) => None,
                            }
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        },
        Err(e) => {
            error!("Failed to parse database response.");
            debug!("{}", e);
            std::process::exit(1);
        },
    };
    return spacebar;
}

pub fn show_spacebars(conn: &Connection) {
    let mut statement = conn.prepare("SELECT * FROM spacebars;").unwrap();
    let spacebars_iter = match statement.query_map(NO_PARAMS, |row| {
        Ok(Spacebar {
            spacebar: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            description: match row.get(3) {
                Ok(o) => Some(o),
                Err(_) => None,
            }
        })
    }) {
        Ok(o) => o,
        Err(e) => {
            debug!("{}", e);
            info!("Failed to parse database output.");
            std::process::exit(1);
        }
    };
    let mut spacebars_gen: Vec<Spacebar> = vec!();
    for spacebar in spacebars_iter {
        spacebars_gen.push(spacebar.unwrap());
    }
    let mut table = Table::new();
    table.add_row(row![BwbFb => "NAME", "DESCRIPTION", "SPACEBAR"]);
    for spacebar in spacebars_gen {
        if spacebar.description.is_some() {
            table.add_row(row![spacebar.name.as_str(), spacebar.description.unwrap().as_str(), c -> format!("⭲{}⭰", bin_to_string(spacebar.spacebar))]);
        } else {
            table.add_row(row![spacebar.name.as_str(), "", c -> format!("⭲{}⭰", bin_to_string(spacebar.spacebar))]);
        }
    }
    table.printstd();
}
