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

use crate::{resolve_color, I18nContext};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IconName {
    Dashboard,
    Users,
    Settings,
    Search,
    Plus,
    Bell,
    ArrowRight,
    ArrowLeft,
    DotsVertical,
    MoreVertical,
    MessageSquare,
    Home,
    User,
    Phone,
    Video,
    Paperclip,
    Smiley,
    Chat,
    Folder,
    FileText,
    Image,
    FileSpreadsheet,
    File,
    UserPlus,
    LogOut,
    X,
}

impl IconName {
    fn asset_path(&self) -> Asset {
        match self {
            IconName::Dashboard => asset!("/assets/icons/dashboard.svg"),
            IconName::Users => asset!("/assets/icons/users.svg"),
            IconName::Settings => asset!("/assets/icons/settings.svg"),
            IconName::Search => asset!("/assets/icons/search.svg"),
            IconName::Plus => asset!("/assets/icons/plus.svg"),
            IconName::Bell => asset!("/assets/icons/bell.svg"),
            IconName::ArrowRight => asset!("/assets/icons/arrow-right.svg"),
            IconName::ArrowLeft => asset!("/assets/icons/arrow-left.svg"),
            IconName::DotsVertical => asset!("/assets/icons/dots-vertical.svg"),
            IconName::MoreVertical => asset!("/assets/icons/more-vertical.svg"),
            IconName::MessageSquare => asset!("/assets/icons/message-square.svg"),
            IconName::Home => asset!("/assets/icons/home.svg"),
            IconName::User => asset!("/assets/icons/user.svg"),
            IconName::Phone => asset!("/assets/icons/phone.svg"),
            IconName::Video => asset!("/assets/icons/video.svg"),
            IconName::Paperclip => asset!("/assets/icons/paperclip.svg"),
            IconName::Smiley => asset!("/assets/icons/smiley.svg"),
            IconName::Chat => asset!("/assets/icons/chat.svg"),
            IconName::Folder => asset!("/assets/icons/folder.svg"),
            IconName::FileText => asset!("/assets/icons/file-text.svg"),
            IconName::Image => asset!("/assets/icons/image.svg"),
            IconName::FileSpreadsheet => asset!("/assets/icons/file-spreadsheet.svg"),
            IconName::File => asset!("/assets/icons/file.svg"),
            IconName::UserPlus => asset!("/assets/icons/user-plus.svg"),
            IconName::LogOut => asset!("/assets/icons/log-out.svg"),
            IconName::X => asset!("/assets/icons/x.svg"),
        }
    }

    /// Returns the SVG content string for the icon
    pub fn svg_content(&self) -> &'static str {
        match self {
            IconName::Dashboard => include_str!("../assets/icons/dashboard.svg"),
            IconName::Users => include_str!("../assets/icons/users.svg"),
            IconName::Settings => include_str!("../assets/icons/settings.svg"),
            IconName::Search => include_str!("../assets/icons/search.svg"),
            IconName::Plus => include_str!("../assets/icons/plus.svg"),
            IconName::Bell => include_str!("../assets/icons/bell.svg"),
            IconName::ArrowRight => include_str!("../assets/icons/arrow-right.svg"),
            IconName::ArrowLeft => include_str!("../assets/icons/arrow-left.svg"),
            IconName::DotsVertical => include_str!("../assets/icons/dots-vertical.svg"),
            IconName::MoreVertical => include_str!("../assets/icons/more-vertical.svg"),
            IconName::MessageSquare => include_str!("../assets/icons/message-square.svg"),
            IconName::Home => include_str!("../assets/icons/home.svg"),
            IconName::User => include_str!("../assets/icons/user.svg"),
            IconName::Phone => include_str!("../assets/icons/phone.svg"),
            IconName::Video => include_str!("../assets/icons/video.svg"),
            IconName::Paperclip => include_str!("../assets/icons/paperclip.svg"),
            IconName::Smiley => include_str!("../assets/icons/smiley.svg"),
            IconName::Chat => include_str!("../assets/icons/chat.svg"),
            IconName::Folder => include_str!("../assets/icons/folder.svg"),
            IconName::FileText => include_str!("../assets/icons/file-text.svg"),
            IconName::Image => include_str!("../assets/icons/image.svg"),
            IconName::FileSpreadsheet => include_str!("../assets/icons/file-spreadsheet.svg"),
            IconName::File => include_str!("../assets/icons/file.svg"),
            IconName::UserPlus => include_str!("../assets/icons/user-plus.svg"),
            IconName::LogOut => include_str!("../assets/icons/log-out.svg"),
            IconName::X => include_str!("../assets/icons/x.svg"),
        }
    }

    pub fn default_translation_key(&self) -> &'static str {
        match self {
            IconName::Dashboard => "icons.dashboard",
            IconName::Users => "icons.users",
            IconName::Settings => "icons.settings",
            IconName::Search => "icons.search",
            IconName::Plus => "icons.plus",
            IconName::Bell => "icons.bell",
            IconName::ArrowRight => "icons.arrow_right",
            IconName::ArrowLeft => "icons.arrow_left",
            IconName::DotsVertical => "icons.dots_vertical",
            IconName::MoreVertical => "icons.more_vertical",
            IconName::MessageSquare => "icons.message_square",
            IconName::Home => "icons.home",
            IconName::User => "icons.user",
            IconName::Phone => "icons.phone",
            IconName::Video => "icons.video",
            IconName::Paperclip => "icons.paperclip",
            IconName::Smiley => "icons.smiley",
            IconName::Chat => "icons.chat",
            IconName::Folder => "icons.folder",
            IconName::FileText => "icons.file_text",
            IconName::Image => "icons.image",
            IconName::FileSpreadsheet => "icons.file_spreadsheet",
            IconName::File => "icons.file",
            IconName::UserPlus => "icons.user_plus",
            IconName::LogOut => "icons.log_out",
            IconName::X => "icons.x",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub name: IconName,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = "16".to_string())]
    pub size: String,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = None)]
    pub alt_key: Option<String>, // Custom translation key
    #[props(default = None)]
    pub alt_text: Option<String>, // Direct alt text (overrides translation)
    #[props(default = None)]
    pub color: Option<String>, // CSS color value
    #[props(default = None)]
    pub highlight_color: Option<String>, // Color for highlight elements like notification dots
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let alt_text = if let Some(direct_alt) = props.alt_text {
        // Use direct alt text if provided
        direct_alt
    } else if let Some(custom_key) = props.alt_key {
        // Use custom translation key if provided
        props.i18n.translate(&custom_key)
    } else {
        // Use default translation key for the icon
        props.i18n.translate(props.name.default_translation_key())
    };

    // Check if the SVG contains tokens that need replacement
    let svg_content = props.name.svg_content();
    let needs_token_replacement =
        svg_content.contains("currentColor") || svg_content.contains("highlightColor");

    if props.color.is_some() || props.highlight_color.is_some() || needs_token_replacement {
        // Use inline SVG when colors are specified or tokens need replacement
        let mut processed_svg = svg_content.to_string();

        // Replace currentColor with the specified color or default
        let resolved_color = if let Some(color) = props.color {
            resolve_color(&color)
        } else {
            resolve_color("var(--color-text-primary)")
        };
        processed_svg = processed_svg.replace("currentColor", &resolved_color);

        // Replace highlightColor with the specified highlight color or default
        let resolved_highlight = if let Some(highlight_color) = props.highlight_color {
            resolve_color(&highlight_color)
        } else {
            resolve_color("var(--color-text-primary)")
        };
        processed_svg = processed_svg.replace("highlightColor", &resolved_highlight);

        rsx! {
            div {
                class: "{props.class}",
                dangerous_inner_html: "{processed_svg}",
                "aria-label": "{alt_text}",
            }
        }
    } else {
        rsx! {
            img {
                src: props.name.asset_path(),
                width: "{props.size}",
                height: "{props.size}",
                alt: "{alt_text}",
                class: "{props.class}",
            }
        }
    }
}
