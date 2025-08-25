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

use dioxus::prelude::*;
use views::{ContactsManager, DesktopUserProfileEdit, Messages, RoomDashboard};
mod components;
mod views;
pub use components::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        RoomDashboard {},
        #[route("/room/:room_id/messages")]
        Messages { room_id: String },
        #[route("/profile/edit")]
        DesktopUserProfileEdit {},
        #[route("/contacts")]
        ContactsManager {},
}

const VARIABLES_CSS: Asset = asset!("/assets/variables.css");
const SHARED_CSS: Asset = asset!("/assets/shared.css");

fn main() {
    // Set the server endpoint for desktop app to connect to
    std::env::set_var("DIOXUS_SERVER_URL", "http://localhost:8080");
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url("http://127.0.0.1:8080");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources - only variables and shared components
        document::Stylesheet { href: VARIABLES_CSS }
        document::Stylesheet { href: SHARED_CSS }

        Router::<Route> {}
    }
}

/// Main app layout that handles routing between RoomDashboard and Messages
#[component]
fn AppLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
