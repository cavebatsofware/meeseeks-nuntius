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

#[cfg(not(target_arch = "wasm32"))]
pub mod entities;
#[cfg(not(target_arch = "wasm32"))]
pub mod migration;
#[cfg(not(target_arch = "wasm32"))]
pub mod persistence;
use dioxus::prelude::*;

#[server]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Test database connection
        let _db = crate::persistence::postgres::establish_connection()
            .await
            .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;
        println!("Database connection test successful");
    }
    Ok("Hello from the Meeseeks Nuntius server! Database connection verified.".to_string())
}
