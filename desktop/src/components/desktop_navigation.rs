use crate::Route;
use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

const DESKTOP_NAV_CSS: Asset = asset!("/assets/desktop_navigation.css");


#[derive(Clone, PartialEq)]
pub struct DesktopNavigationItem {
    pub icon: IconName,
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
            icon: IconName::Dashboard,
            text_key: "rooms.dashboard",
            route: Route::RoomDashboard {},
            nav_id: "dashboard",
        },
        DesktopNavigationItem {
            icon: IconName::Users,
            text_key: "rooms.select",
            route: Route::RoomDashboard {}, // TODO: Update when room selection page exists
            nav_id: "room_select",
        },
        DesktopNavigationItem {
            icon: IconName::Users,
            text_key: "rooms.management",
            route: Route::RoomDashboard {}, // TODO: Update when room management page exists
            nav_id: "room_management",
        },
        DesktopNavigationItem {
            icon: IconName::Settings,
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
                        i18n: props.i18n.clone(),
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
    icon: IconName,
    text: String,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
    i18n: I18nContext,
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

            Icon {
                name: props.icon,
                i18n: props.i18n.clone(),
                class: "dn-nav-icon".to_string()
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
