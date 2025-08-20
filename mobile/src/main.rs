use dioxus::prelude::*;

use ui::{I18nContext, Navbar};
use views::MobileRoomDashboard;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileNavbar)]
    #[route("/")]
    Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A mobile-specific Router around the shared `Navbar` component
/// which allows us to use the mobile-specific `Route` enum.
#[component]
fn MobileNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
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
