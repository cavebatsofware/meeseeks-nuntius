//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    println!("ECHO!\n");

    #[cfg(not(target_arch = "wasm32"))]
    println!("Not running on WASM (web)");

    #[cfg(target_arch = "wasm32")]
    println!("Running on WASM (web)");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let conn = rusqlite::Connection::open("local.db").expect("Failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL)", ())?;
        conn.execute("INSERT INTO users (name) values (?1)", ((input).clone(),))?;
        
        let mut stmt = conn.prepare("SELECT id, name FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        println!("Users Found:");
        for user in user_iter {
            println!("Found user {:?}", user.unwrap());
        }
    }

    Ok(input)
}
