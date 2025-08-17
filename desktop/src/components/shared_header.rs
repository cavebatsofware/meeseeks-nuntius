use dioxus::prelude::*;
use ui::{Icon, IconName, I18nContext};

const SHARED_HEADER_CSS: Asset = asset!("/assets/shared_header.css");
// TODO: needs the real app logo

#[derive(Props, Clone, PartialEq)]
pub struct SharedHeaderProps {
    #[props(default = "Cavebat".to_string())]
    pub brand_name: String,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
}

#[component]
pub fn SharedHeader(props: SharedHeaderProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SHARED_HEADER_CSS }

        div {
            class: "sh-header",

            // Logo
            div {
                class: "sh-logo-container",
                Icon {
                    name: IconName::Dashboard,
                    i18n: props.i18n.clone(),
                    alt_key: Some("icons.logo".to_string()),
                    class: "sh-logo-image".to_string()
                }
            }

            // Brand name
            h1 {
                class: "sh-brand-name",
                "{props.brand_name}"
            }
        }
    }
}
