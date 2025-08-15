use crate::Route;
use dioxus::prelude::*;
use ui::I18nContext;

const DESKTOP_NAV_CSS: Asset = asset!("/assets/desktop_navigation.css");

// Navigation icons - svg strings pulled from figma
const DASHBOARD_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1H7V7H1V1ZM9 1H15V7H9V1ZM1 9H7V15H1V9ZM9 9H15V15H9V9Z' fill='%23ffffff'/%3E%3C/svg%3E";
const USERS_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M5.5 7A3 3 0 1 0 5.5 1a3 3 0 0 0 0 6ZM1 12v2h9v-2a3 3 0 0 0-3-3H4a3 3 0 0 0-3 3Zm10-4.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5ZM15 14v-1a2 2 0 0 0-1.18-1.83A3.01 3.01 0 0 1 15 14Z' fill='%23ffffff'/%3E%3C/svg%3E";
const SETTINGS_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z' fill='%23ffffff'/%3E%3Cpath d='M14 8a6 6 0 1 1-12 0 6 6 0 0 1 12 0Z' stroke='%23ffffff' stroke-width='1.5'/%3E%3C/svg%3E";

#[derive(Clone, PartialEq)]
pub struct DesktopNavigationItem {
    pub icon: &'static str,
    pub text_key: &'static str,
    pub route: Route,
    pub nav_id: &'static str, // Unique identifier for this nav item
}

#[derive(Props, Clone, PartialEq)]
pub struct DesktopNavigationProps {
    pub i18n: I18nContext,
}

#[component]
pub fn DesktopNavigation(props: DesktopNavigationProps) -> Element {
    let nav = navigator();
    let current_route = use_route::<Route>();

    let navigation_items = vec![
        DesktopNavigationItem {
            icon: DASHBOARD_ICON,
            text_key: "rooms.dashboard",
            route: Route::RoomDashboard {},
            nav_id: "dashboard",
        },
        DesktopNavigationItem {
            icon: USERS_ICON,
            text_key: "rooms.select",
            route: Route::RoomDashboard {}, // TODO: Update when room selection page exists
            nav_id: "room_select",
        },
        DesktopNavigationItem {
            icon: USERS_ICON,
            text_key: "rooms.management",
            route: Route::RoomDashboard {}, // TODO: Update when room management page exists
            nav_id: "room_management",
        },
        DesktopNavigationItem {
            icon: SETTINGS_ICON,
            text_key: "nav.settings",
            route: Route::DesktopUserProfileEdit {},
            nav_id: "settings",
        },
    ];

    rsx! {
        document::Link { rel: "stylesheet", href: DESKTOP_NAV_CSS }

        nav {
            class: "dn-nav",

            // Navigation items
            div {
                class: "dn-nav-items",

                for item in navigation_items {
                    DesktopNavigationItemComponent {
                        key: "{item.text_key}",
                        icon: item.icon,
                        text: props.i18n.translate(item.text_key),
                        is_active: is_nav_item_active(&current_route, item.nav_id),
                        onclick: {
                            let route = item.route.clone();
                            move |_| {
                                nav.push(route.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DesktopNavigationItemComponentProps {
    icon: &'static str,
    text: String,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn DesktopNavigationItemComponent(props: DesktopNavigationItemComponentProps) -> Element {
    let class = if props.is_active {
        "dn-nav-item dn-nav-item-active"
    } else {
        "dn-nav-item"
    };

    rsx! {
        div {
            class: class,
            onclick: move |evt| props.onclick.call(evt),

            img {
                src: props.icon,
                alt: props.text,
                class: "dn-nav-icon"
            }

            span {
                class: "dn-nav-text",
                "{props.text}"
            }
        }
    }
}

// Helper function to determine if a nav item should be active based on current route
fn is_nav_item_active(current_route: &Route, nav_id: &str) -> bool {
    match (current_route, nav_id) {
        (Route::RoomDashboard {}, "dashboard") => true,
        (Route::DesktopUserProfileEdit {}, "settings") => true,
        // For future routes, add more specific matching here
        // (Route::RoomSelect {}, "room_select") => true,
        // (Route::RoomManagement {}, "room_management") => true,
        _ => false,
    }
}
