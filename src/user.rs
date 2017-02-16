use postgres::{Connection, TlsMode};
use serde_json::Value;
use User;

pub fn readPostgreSQL() -> User{
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let mut users = User{
        id: 0,
        username: String::new(),
        email: String::new(),
        password: String::new(),
        userID: 0,
    };

    for row in &conn.query("SELECT id, username, email, password, userid FROM users", &[]).unwrap() {
        users = User {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            password: row.get(3),
            userID: row.get(4)
        };
    }
    return users;
}
