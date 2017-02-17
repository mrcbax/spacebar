use postgres::{Connection, TlsMode};
use serde_json::Value;

struct newUser {
    username: String,
    email: String,
    password: String,
}

struct barUser {
    name: String,
    description: String,
    user_ID: u32,
    barcode: String
}


pub fn insertUser(){
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let new_u = newUser {
        username: String::from("diego"),
        email: String::from("diegotest@gmail.com"),
        password: String::from("password123")
    };
    //conn.execute("INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3)", &[&new_u.username, &new_u.email, &new_u.password]).unwrap();
    println!("Just inserted.");
}


// Pass through User ID, Name, Description
pub fn insertBarCode(){
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let bar =  barUser {
        name: String::from("diego"),
        description: String::from("diego"),
        user_ID: 1,
        barcode: String::from("diego"),
    };
    // conn.execute("INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3)", &[&new_u.username, &new_u.email, &new_u.password]).unwrap();
    println!("Just inserted.");
}
