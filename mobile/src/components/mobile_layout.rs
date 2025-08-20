use super::{MobileHeader, MobileNavigation};
use dioxus::prelude::*;
use ui::I18nContext;

const MOBILE_LAYOUT_CSS: Asset = asset!("/assets/mobile_layout.css");

#[derive(Props, Clone, PartialEq)]
pub struct MobileLayoutProps {
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = "Cavebat".to_string())]
    pub brand_name: String,
    #[props(default = "rooms".to_string())]
    pub active_tab: String,
    #[props(default = false)]
    pub has_notifications: bool,
    #[props(default = 0)]
    pub notification_count: i32,
    pub on_tab_change: EventHandler<String>,
    pub children: Element,
}

#[component]
pub fn MobileLayout(props: MobileLayoutProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MOBILE_LAYOUT_CSS }

        div {
            class: "ml-container",

            // Header
            MobileHeader {
                brand_name: props.brand_name,
                i18n: props.i18n.clone(),
                has_notifications: props.has_notifications,
                notification_count: props.notification_count,
            }

            // Main content
            main {
                class: "ml-main-content",
                {props.children}
            }

            // Footer Navigation
            MobileNavigation {
                i18n: props.i18n.clone(),
                active_tab: props.active_tab,
                on_tab_change: props.on_tab_change,
            }
        }
    }
}