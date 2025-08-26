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

use super::{DesktopNavigation, SharedHeader};
use crate::Route;
use dioxus::prelude::*;
use ui::{I18nContext, UserProfileMini};

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
                    brand_name: props.brand_name,
                    i18n: props.i18n.clone()
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
