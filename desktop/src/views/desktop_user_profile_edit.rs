use crate::DesktopLayout;
use api::user_data::UserData;
use dioxus::prelude::*;
use ui::{I18nContext, UserProfileEdit};

#[derive(Props, Clone, PartialEq)]
pub struct DesktopUserProfileEditProps {
    #[props(default = "Thom T.".to_string())]
    username: String,
    #[props(default = "Online".to_string())]
    user_status: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[component]
pub fn DesktopUserProfileEdit(props: DesktopUserProfileEditProps) -> Element {
    // Create initial user data
    let initial_data = UserData::default_for_user(&props.username);

    let handle_save = move |user_data: UserData| {
        // TODO: Implement save logic
        println!("Saving user data: {user_data:?}");
    };

    let handle_cancel = move |_| {
        // TODO: Navigate back or show confirmation
        println!("User cancelled profile edit");
    };

    rsx! {
        DesktopLayout {
            i18n: props.i18n.clone(),
            username: props.username.clone(),
            user_status: props.user_status.clone(),
            user_avatar_initial: props.username.chars().next().unwrap_or('U').to_string(),
            brand_name: "Cavebat".to_string(),

            div {
                class: "content-wrapper",

                UserProfileEdit {
                    initial_data: initial_data,
                    i18n: props.i18n.clone(),
                    on_save: handle_save,
                    on_cancel: handle_cancel,
                    is_saving: false
                }
            }
        }
    }
}
