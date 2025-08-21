use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

const MOBILE_NAV_CSS: Asset = asset!("/assets/mobile_navigation.css");

#[derive(Clone, PartialEq)]
pub struct MobileNavigationItem {
    pub icon: IconName,
    pub text_key: &'static str,
    pub nav_id: &'static str,
    pub is_active: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct MobileNavigationProps {
    pub i18n: I18nContext,
    #[props(default = "rooms".to_string())]
    pub active_tab: String,
    pub on_tab_change: EventHandler<String>,
}

#[component]
pub fn MobileNavigation(props: MobileNavigationProps) -> Element {
    let navigation_items = vec![
        MobileNavigationItem {
            icon: IconName::Home,
            text_key: "rooms.dashboard",
            nav_id: "rooms",
            is_active: props.active_tab == "rooms",
        },
        MobileNavigationItem {
            icon: IconName::Users,
            text_key: "nav.contacts",
            nav_id: "contacts",
            is_active: props.active_tab == "contacts",
        },
        MobileNavigationItem {
            icon: IconName::Settings,
            text_key: "nav.settings",
            nav_id: "settings",
            is_active: props.active_tab == "settings",
        },
        MobileNavigationItem {
            icon: IconName::User,
            text_key: "user_profile.title",
            nav_id: "profile",
            is_active: props.active_tab == "profile",
        },
    ];

    rsx! {
        document::Stylesheet { href: MOBILE_NAV_CSS }

        nav {
            class: "mn-nav",

            div {
                class: "mn-nav-items",

                for item in navigation_items {
                    MobileNavigationItemComponent {
                        key: "{item.nav_id}",
                        icon: item.icon,
                        text: props.i18n.translate(item.text_key),
                        nav_id: item.nav_id.to_string(),
                        is_active: item.is_active,
                        i18n: props.i18n.clone(),
                        onclick: {
                            let on_tab_change = props.on_tab_change;
                            let nav_id = item.nav_id.to_string();
                            move |_| {
                                on_tab_change.call(nav_id.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MobileNavigationItemComponentProps {
    icon: IconName,
    text: String,
    nav_id: String,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
    i18n: I18nContext,
}

#[component]
fn MobileNavigationItemComponent(props: MobileNavigationItemComponentProps) -> Element {
    let class = if props.is_active {
        "mn-nav-item mn-nav-item-active"
    } else {
        "mn-nav-item"
    };

    rsx! {
        button {
            class: class,
            onclick: move |evt| props.onclick.call(evt),

            Icon {
                name: props.icon,
                i18n: props.i18n.clone(),
                class: "mn-nav-icon".to_string(),
                color: if props.is_active {
                    Some("var(--color-accent-primary)".to_string())
                } else {
                    Some("var(--color-text-secondary)".to_string())
                }
            }

            span {
                class: "mn-nav-text",
                "{props.text}"
            }
        }
    }
}
