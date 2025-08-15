use dioxus::prelude::*;
use ui::{I18nContext, UserProfileMini};
use super::{SharedHeader, DesktopNavigation};
use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct DesktopLayoutProps {
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = "User".to_string())]
    pub username: String,
    #[props(default = "Online".to_string())]
    pub user_status: String,
    #[props(default = "U".to_string())]
    pub user_avatar_initial: String,
    #[props(default = "Cavebat".to_string())]
    pub brand_name: String,
    pub children: Element,
}

#[component]
pub fn DesktopLayout(props: DesktopLayoutProps) -> Element {
    let nav = navigator();
    
    rsx! {
        div {
            class: "dashboard-container",

            // Sidebar
            aside {
                class: "sidebar",

                SharedHeader {
                    brand_name: props.brand_name
                }

                DesktopNavigation {
                    i18n: props.i18n.clone(),
                }

                // User profile section
                UserProfileMini {
                    username: props.username,
                    status: props.user_status,
                    avatar_initial: props.user_avatar_initial,
                    i18n: props.i18n.clone(),
                    onclick: move |_| {
                        nav.push(Route::DesktopUserProfileEdit {});
                    }
                }
            }

            // Main content
            main {
                class: "main-content",
                {props.children}
            }
        }
    }
}
