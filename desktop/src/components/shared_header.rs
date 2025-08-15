use dioxus::prelude::*;

const SHARED_HEADER_CSS: Asset = asset!("/assets/shared_header.css");
const DASHBOARD_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1H7V7H1V1ZM9 1H15V7H9V1ZM1 9H7V15H1V9ZM9 9H15V15H9V9Z' fill='%23ffffff'/%3E%3C/svg%3E";
const IMG_GROUP: &str = DASHBOARD_ICON; // needs the real app logo

#[derive(Props, Clone, PartialEq)]
pub struct SharedHeaderProps {
    #[props(default = "Cavebat".to_string())]
    pub brand_name: String,
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
                img {
                    src: IMG_GROUP,
                    alt: "{props.brand_name} Logo",
                    class: "sh-logo-image"
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
