/*  This file is part of a secure messaging project codename meeseeks-nuntius
 *  Copyright (C) 2025  Grant DeFayette
 *
 *  meeseeks-nuntius is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  meeseeks-nuntius is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with meeseeks-nuntius.  If not, see <https://www.gnu.org/licenses/>.
 */

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