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

use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

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
