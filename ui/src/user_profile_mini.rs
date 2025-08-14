use dioxus::prelude::*;
use crate::I18nContext;

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
}

#[component]
pub fn UserProfileMini(props: UserProfileMiniProps) -> Element {
    rsx! {
        div {
            class: "user-profile-mini",
            div {
                class: "user-avatar-mini",
                "{props.avatar_initial}"
            }
            div {
                class: "user-info-mini",
                div { class: "user-name-mini", "{props.username}" }
                div { class: "user-status-mini", "{props.status}" }
            }
        }
    }
}