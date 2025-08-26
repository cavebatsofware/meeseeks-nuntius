/*  This file is part of a secure messaging project codename meeseeks-nuntius
 *  Copyright (C) 2025  Grant DeFayette
 *
 *  meeseeks-nuntius is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  meeseeks-nuntius is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with meeseeks-nuntius.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::DesktopLayout;
use dioxus::prelude::*;
use shared::user_data::UserData;
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
