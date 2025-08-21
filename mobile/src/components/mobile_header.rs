use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

const MOBILE_HEADER_CSS: Asset = asset!("/assets/mobile_header.css");

#[derive(Props, Clone, PartialEq)]
pub struct MobileHeaderProps {
    #[props(default = "Cavebat".to_string())]
    pub brand_name: String,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = false)]
    pub has_notifications: bool,
    #[props(default = 0)]
    pub notification_count: i32,
}

#[component]
pub fn MobileHeader(props: MobileHeaderProps) -> Element {
    rsx! {
        document::Stylesheet { href: MOBILE_HEADER_CSS }

        div {
            class: "mh-header",

            div {
                class: "mh-header-content",

                // Left side - Back button and brand
                div {
                    class: "mh-left-section",

                    button {
                        class: "mh-back-btn",
                        Icon {
                            name: IconName::ArrowLeft,
                            i18n: props.i18n.clone(),
                            class: "mh-back-icon".to_string(),
                        }
                    }

                    div {
                        class: "mh-brand-container",
                        Icon {
                            name: IconName::MessageSquare,
                            i18n: props.i18n.clone(),
                            class: "mh-brand-icon".to_string()
                        }
                        div {
                            class: "mh-brand-name",
                            "{props.brand_name}"
                        }
                    }
                }

                // Right side - Notifications and menu
                div {
                    class: "mh-right-section",

                    button {
                        class: "mh-notification-btn",
                        Icon {
                            name: IconName::Bell,
                            i18n: props.i18n.clone(),
                            class: "mh-notification-icon".to_string()
                        }
                        if props.has_notifications && props.notification_count > 0 {
                            div {
                                class: "mh-notification-badge"
                            }
                        }
                    }

                    button {
                        class: "mh-menu-btn",
                        Icon {
                            name: IconName::MoreVertical,
                            i18n: props.i18n.clone(),
                            class: "mh-menu-icon".to_string()
                        }
                    }
                }
            }
        }
    }
}
