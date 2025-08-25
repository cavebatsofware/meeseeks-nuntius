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

// Platform-specific CSS assets
const MOBILE_HEADER_IOS_CSS: Asset = asset!("/assets/mobile_header_ios.css");
const MOBILE_MESSAGES_IOS_CSS: Asset = asset!("/assets/mobile_messages_ios.css");
const MOBILE_HEADER_ANDROID_CSS: Asset = asset!("/assets/mobile_header_android.css");
const MOBILE_MESSAGES_ANDROID_CSS: Asset = asset!("/assets/mobile_messages_android.css");

fn main() {
    // Set the server endpoint for mobile app to connect to
    // Read server URL from environment variable, fallback to default if not set
    let server_url =
        std::env::var("DIOXUS_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    std::env::set_var("DIOXUS_SERVER_URL", &server_url);
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url(Box::leak(server_url.into_boxed_str()));
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Stylesheet { href: VARIABLES_CSS }
        document::Stylesheet { href: MAIN_CSS }

        // Platform-specific styles
        if cfg!(target_os = "ios") {
            document::Stylesheet { href: MOBILE_HEADER_IOS_CSS }
            document::Stylesheet { href: MOBILE_MESSAGES_IOS_CSS }
        } else {
            document::Stylesheet { href: MOBILE_HEADER_ANDROID_CSS }
            document::Stylesheet { href: MOBILE_MESSAGES_ANDROID_CSS }
        }

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
