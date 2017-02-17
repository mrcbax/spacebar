
extern crate serde_json;
use postgres::{Connection, TlsMode};
use serde_json::*;
use User;

pub fn readPostgreSQL() -> String{
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let mut users = User{
        userID: 0,
        username: String::new(),
        email: String::new(),
        password: String::new(),
    };

    for row in &conn.query("SELECT user_id, user_name, email, password FROM users", &[]).unwrap() {
        users = User {
            userID: row.get(0),
            username: row.get(1),
            email: row.get(2),
            password: row.get(3),
        };
    }

    let serialized = serde_json::to_string(&users).unwrap();
    println!("serialized = {:?}", serialized);

    return serialized;
}
