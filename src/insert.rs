use postgres::{Connection, TlsMode};
use serde_json::Value;

struct newUser {
    username: String,
    email: String,
    password: String,
}


pub fn insertUser(){
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let new_u = newUser {
        username: String::from("diego"),
        email: String::from("diegotest@gmail.com"),
        password: String::from("password123")
    };
    conn.execute("INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3)", &[&new_u.username, &new_u.email, &new_u.password]).unwrap();
    println!("Just inserted.");
}
