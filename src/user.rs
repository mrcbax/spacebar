
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
    let mut owned_string: String = "".to_owned();
    let another_owned_string: &str = "";
    let together: &str = "";
    let mut serialized = serde_json::to_string(&users).unwrap();

    for row in &conn.query("SELECT user_id, user_name, email, password FROM users", &[]).unwrap() {
        users = User {
            userID: row.get(0),
            username: row.get(1),
            email: row.get(2),
            password: row.get(3),
        };
         owned_string = serde_json::to_string(&users).unwrap();
         println!("HERE: {:?}", owned_string);

         serialized.push_str(&owned_string);
    }

    // println!("Here:{:?}", users);
    // owned_string = serialized;

    // 

    return serialized;
}
