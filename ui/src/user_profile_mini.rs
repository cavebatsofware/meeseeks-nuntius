use crate::I18nContext;
use dioxus::prelude::*;

const USER_PROFILE_MINI_CSS: Asset = asset!("/assets/styling/user_profile_mini.css");

#[derive(Props, Clone, PartialEq)]
pub struct UserProfileMiniProps {
    #[props(default = "User".to_string())]
    pub username: String,
    #[props(default = "Online".to_string())]
    pub status: String,
    #[props(default = "U".to_string())]
    pub avatar_initial: String,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn UserProfileMini(props: UserProfileMiniProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: USER_PROFILE_MINI_CSS }

        div {
            class: if props.onclick.is_some() { "upm-container upm-container-clickable" } else { "upm-container" },
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            div {
                class: "upm-avatar",
                "{props.avatar_initial}"
            }
            div {
                class: "upm-info",
                div { class: "upm-name", "{props.username}" }
                div { class: "upm-status", "{props.status}" }
            }
        }
    }
}
