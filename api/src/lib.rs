//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod crypto;

#[cfg(not(target_arch = "wasm32"))]
pub mod persistence;

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
    }

    Ok(input)
}

#[cfg(not(target_arch = "wasm32"))]
#[server(CreateParty)]
pub async fn create_party(name: String) -> Result<String, ServerFnError> {
    let party = Party::new(name);
    
}