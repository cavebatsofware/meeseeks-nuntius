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
use api::persistence::postgres::{establish_connection, run_migrations};
use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;
use dioxus_logger::init;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init(Level::INFO).expect("failed to init logger");

    // Load environment variables and check for migration argument
    #[cfg(not(target_arch = "wasm32"))]
    {
        dotenvy::dotenv().ok();

        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1] == "migrate" {
            match run_migrations_sync() {
                Ok(_) => return Ok(()),
                Err(e) => return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Migration failed: {}", e)
                ))),
            }
        }
    }

    info!("Starting Meeseeks Nuntius server...");

    // Launch the server
    launch(app);
    Ok(())
}

fn run_migrations_sync() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;
        rt.block_on(async {
            info!("Running database migrations...");

            let db = establish_connection().await?;
            info!("Database connection established");

            run_migrations(&db).await?;
            info!("Database migrations completed successfully");

            Ok(())
        })
    }

    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

fn app() -> Element {
    let mut server_data = use_signal(|| None::<String>);

    rsx! {
        div {
            h1 { "Meeseeks Nuntius Server" }
            p { "Server is running and ready to handle requests." }

            button {
                onclick: move |_| {
                    spawn(async move {
                        println!("Received server data");
                        match api::get_server_data().await {
                            Ok(data) => server_data.set(Some(data)),
                            Err(e) => server_data.set(Some(format!("Error: {}", e))),
                        }
                    });
                },
                "Test Server Connection"
            }

            if let Some(data) = server_data() {
                p { style: "margin-top: 20px; padding: 10px; background-color: #f0f0f0; border-radius: 5px;",
                    "{data}"
                }
            }
        }
    }
}
