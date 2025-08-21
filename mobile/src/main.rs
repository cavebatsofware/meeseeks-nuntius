use dioxus::prelude::*;

use ui::I18nContext;
use views::{MobileMessages, MobileRoomDashboard};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/messages/:room_id")]
    Messages { room_id: String },
}

const VARIABLES_CSS: Asset = asset!("/assets/variables.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Stylesheet { href: VARIABLES_CSS }
        document::Stylesheet { href: MAIN_CSS }

        // Enable safe area support for iOS
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover"
        }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        MobileRoomDashboard {
            username: "Mobile User".to_string(),
            user_subtitle: "Welcome to Mobile App".to_string(),
            i18n: I18nContext::new("en")
        }
    }
}

#[component]
fn Messages(room_id: String) -> Element {
    rsx! {
        MobileMessages {
            room_id: room_id,
            room_name: "First Mac Room".to_string(),
            i18n: I18nContext::new("en"),
            show_side_panel: false
        }
    }
}
