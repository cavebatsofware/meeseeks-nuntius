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
    //dioxus_logger::init(Level::INFO).expect("failed to init logger");
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
