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

use crate::{DesktopLayout, Route};
use api::get_server_data;
use dioxus::prelude::*;
use shared::local::{create_room, get_all_rooms};
use ui::{get_language_name, get_text_direction, I18nContext, Icon, IconName, RoomData};

const PARTY_DASH_CSS: Asset = asset!("/assets/room_dash.css");

const IMG_IMG: &str = "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face&auto=format"; // Placeholder avatar

#[derive(Props, Clone, PartialEq)]
pub struct RoomDashboardProps {
    #[props(default = "Thom T.".to_string())]
    username: String,
    #[props(default = "View Profile".to_string())]
    user_subtitle: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[component]
pub fn RoomDashboard(props: RoomDashboardProps) -> Element {
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut rooms = use_signal(|| Vec::<RoomData>::new());
    let mut loading_rooms = use_signal(|| true);
    let mut server_data = use_signal(|| None::<String>);
    let locale = props.i18n.get_current_locale();
    // Keep for debugging until language switcher is implemented
    println!("Language: {}", get_language_name(locale));
    println!("Text direction: {}", get_text_direction(locale));

    // Load rooms on component initialization
    use_effect(move || {
        spawn(async move {
            match get_all_rooms().await {
                Ok(room_jsons) => {
                    let mut parsed_rooms = Vec::new();
                    for room_json in room_jsons {
                        if let Ok(room_data) = serde_json::from_str::<RoomData>(&room_json) {
                            parsed_rooms.push(room_data);
                        }
                    }
                    rooms.set(parsed_rooms);
                    loading_rooms.set(false);
                }
                Err(_) => {
                    loading_rooms.set(false);
                }
            }
        });
    });

    // Function to refresh rooms
    let refresh_rooms = move || {
        spawn(async move {
            if let Ok(room_jsons) = get_all_rooms().await {
                let mut parsed_rooms = Vec::new();
                for room_json in room_jsons {
                    if let Ok(room_data) = serde_json::from_str::<RoomData>(&room_json) {
                        parsed_rooms.push(room_data);
                    }
                }
                rooms.set(parsed_rooms);
            }
        });
    };
    rsx! {
    document::Stylesheet { href: PARTY_DASH_CSS }

    DesktopLayout {
        i18n: props.i18n.clone(),
        username: props.username.clone(),
        user_status: props.user_subtitle.clone(),
        user_avatar_initial: props.username.chars().next().unwrap_or('U').to_string(),
        brand_name: "Cavebat".to_string(),

            div {
                class: "content-wrapper",

                // Header
                header {
                    class: "main-header",

                    // Title section
                    div {
                        class: "header-title",

                        h1 {
                            class: "page-title",
                            "{props.i18n.translate(\"room_dashboard.title\")}"
                        }

                        p {
                            class: "page-subtitle",
                            "{props.i18n.translate(\"room_dashboard.subtitle\")}"
                        }
                    }

                    // Search and profile section
                    div {
                        class: "header-actions",

                        // Search bar
                        div {
                            class: "search-container",

                            Icon {
                                name: IconName::Search,
                                i18n: props.i18n.clone(),
                                class: "search-icon".to_string()
                            }

                            input {
                                r#type: "text",
                                placeholder: "{props.i18n.translate(\"room_dashboard.search_placeholder\")}",
                                class: "search-input"
                            }
                        }

                        // Notification button
                        button {
                            class: "notification-button",

                            Icon {
                                name: IconName::Bell,
                                i18n: props.i18n.clone(),
                                class: "notification-icon".to_string()
                            }

                            // Notification badge
                            span {
                                class: "notification-badge"
                            }
                        }

                        // User avatar
                        div {
                            class: "header-avatar",

                            img {
                                src: IMG_IMG,
                                alt: "User Avatar",
                                class: "avatar-image"
                            }
                        }
                    }
                }

                // Top Communication Parties section
                section {
                    class: "parties-section",

                    h2 {
                        class: "section-title",
                        "{props.i18n.translate(\"room_dashboard.sections.top_rooms\")}"
                    }

                    div {
                        class: "parties-grid",

                        if loading_rooms() {
                            div {
                                class: "loading-rooms",
                                "{props.i18n.translate(\"rooms.loading\")}"
                            }
                        } else {
                            // Dynamic rooms from database
                            for room in rooms() {
                                RoomCardComponent {
                                    key: "{room.id.as_deref().unwrap_or(&room.name)}",
                                    room_id: room.id.as_deref().unwrap_or(&room.name).to_string(),
                                    title: room.name.clone(),
                                    description: room.description.as_ref()
                                        .filter(|desc| !desc.is_empty())
                                        .cloned()
                                        .unwrap_or_else(|| props.i18n.translate("rooms.default_description")),
                                    badge_text: "",
                                    badge_color: "",
                                    member_count: room.member_count
                                        .filter(|&count| count > 0)
                                        .map(|count| format!("+{count}"))
                                        .unwrap_or_default(),
                                    i18n: props.i18n.clone()
                                }
                            }
                        }

                        // Create New Room - always show
                        CreateRoomCard {
                            i18n: props.i18n.clone(),
                            on_room_created: refresh_rooms
                        }
                    }

                    // Test Server Connection Section
                    div {
                        class: "test-server-section",
                        style: "margin-top: 20px; padding: 15px; border: 1px solid #ddd; border-radius: 8px;",

                        h3 { "Test Server Connection" }

                        button {
                            onclick: move |_| {
                                spawn(async move {
                                    println!("Received server data on desktop");
                                    match get_server_data().await {
                                        Ok(data) => server_data.set(Some(data)),
                                        Err(e) => server_data.set(Some(format!("Error: {}", e))),
                                    }
                                });
                            },
                            style: "padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            "Test Server Connection"
                        }

                        if let Some(data) = server_data() {
                            p {
                                style: "margin-top: 15px; padding: 10px; background-color: #f0f0f0; border-radius: 5px;",
                                "{data}"
                            }
                        }
                    }
                }

                // Latest Unread Messages section
                section {
                    class: "messages-section",

                    h2 {
                        class: "section-title",
                        "{props.i18n.translate(\"room_dashboard.sections.latest_messages\")}"
                    }

                    div {
                        class: "messages-container",

                        MessageComponent {
                            avatar_color: "purple",
                            username: "Diana P.",
                            room: "Project Overlord",
                            message: "The latest schematics are ready for review. Please check the encrypted folder.",
                            time: "2 min ago",
                            is_first: true,
                            i18n: props.i18n.clone()
                        }

                        MessageComponent {
                            avatar_color: "blue",
                            username: "C. Kent",
                            room: "Stealth Ops",
                            message: "Confirmed. Target acquired. Awaiting final command for phase two.",
                            time: "15 min ago",
                            is_first: false,
                            i18n: props.i18n.clone()
                        }

                        MessageComponent {
                            avatar_color: "blue",
                            username: "Barbara G.",
                            room: "Project Overlord",
                            message: "I've bypassed their firewall. The data stream is now open. Be quick.",
                            time: "48 min ago",
                            is_first: false,
                            i18n: props.i18n.clone()
                        }

                        MessageComponent {
                            avatar_color: "blue",
                            username: "Victor S.",
                            room: "R&D Division",
                            message: "The prototype's energy signature is unstable. I recommend we run more diagnostics.",
                            time: "1 hour ago",
                            is_first: false,
                            i18n: props.i18n.clone()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PartyCardProps {
    room_id: String,
    title: String,
    description: String,
    badge_text: String,
    badge_color: String,
    member_count: String,
    i18n: I18nContext,
}

#[component]
fn RoomCardComponent(props: PartyCardProps) -> Element {
    rsx! {
        div {
            class: "room-card",

            // Header with title and badge
            div {
                class: "room-header",

                h3 {
                    class: "room-title",
                    "{props.title}"
                }

                if !props.badge_text.is_empty() {
                    span {
                        class: "room-badge badge-{props.badge_color}",
                        "{props.badge_text}"
                    }
                }
            }

            // Description
            p {
                class: "room-description",
                "{props.description}"
            }

            // Footer with avatars and enter button
            div {
                class: "room-footer",

                // Member avatars
                div {
                    class: "member-avatars",

                    // Mock avatars
                    div { class: "avatar avatar-1" }
                    div { class: "avatar avatar-2" }
                    div { class: "avatar avatar-3" }

                    if !props.member_count.is_empty() {
                        div {
                            class: "avatar avatar-count",
                            "{props.member_count}"
                        }
                    }
                }

                // Enter button
                Link {
                    class: "enter-button",
                    to: Route::Messages { room_id: props.room_id },
                    onclick: move |_| {
                        // web_sys::console::log_1(&"Enter button clicked!".into());
                    },
                    "{props.i18n.translate(\"room_dashboard.enter\")}"
                    Icon {
                        name: IconName::ArrowRight,
                        i18n: props.i18n.clone(),
                        class: "enter-icon".to_string()
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MessageProps {
    avatar_color: &'static str,
    username: String,
    room: String,
    message: String,
    time: String,
    is_first: bool,
    i18n: I18nContext,
}

#[component]
fn MessageComponent(props: MessageProps) -> Element {
    rsx! {
        div {
            class: if props.is_first { "message-item first" } else { "message-item" },

            // Avatar
            div {
                class: "message-avatar avatar-{props.avatar_color}"
            }

            // Message content
            div {
                class: "message-content",

                // Header with username, room, and time
                div {
                    class: "message-header",

                    div {
                        class: "message-names",
                        span {
                            class: "username",
                            "{props.username}"
                        }
                        span {
                            class: "separator",
                            "{props.i18n.translate(\"room_dashboard.separator\")}"
                        }
                        span {
                            class: "room-name",
                            "{props.room}"
                        }
                    }

                    span {
                        class: "message-time",
                        "{props.time}"
                    }
                }

                // Message text
                p {
                    class: "message-text",
                    "{props.message}"
                }
            }

            // Menu dots
            button {
                class: "message-menu",

                Icon {
                    name: IconName::DotsVertical,
                    i18n: props.i18n.clone(),
                    class: "menu-icon".to_string()
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CreateRoomCardProps {
    i18n: I18nContext,
    on_room_created: EventHandler<()>,
}

#[component]
fn CreateRoomCard(props: CreateRoomCardProps) -> Element {
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut room_name = use_signal(|| String::new());
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut room_description = use_signal(|| String::new());
    let mut show_form = use_signal(|| false);
    let mut creating = use_signal(|| false);

    rsx! {
        div {
            class: "create-room-card",

            if show_form() {
                // Room creation form
                div {
                    class: "create-room-form",

                    input {
                        r#type: "text",
                        placeholder: "{props.i18n.translate(\"rooms.enter_name\")}",
                        class: "room-name-input",
                        value: "{room_name()}",
                        oninput: move |evt| room_name.set(evt.value()),
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter && !room_name().trim().is_empty() && !creating() {
                                let name = room_name().clone();
                                let description = if room_description().trim().is_empty() {
                                    None
                                } else {
                                    Some(room_description().clone())
                                };
                                creating.set(true);
                                let on_created = props.on_room_created;
                                spawn(async move {
                                    match create_room(name, description).await {
                                        Ok(_) => {
                                            room_name.set(String::new());
                                            room_description.set(String::new());
                                            show_form.set(false);
                                            creating.set(false);
                                            on_created.call(());
                                        }
                                        Err(_) => {
                                            creating.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    }

                    textarea {
                        placeholder: "{props.i18n.translate(\"rooms.enter_description\")}",
                        class: "room-description-input",
                        value: "{room_description()}",
                        oninput: move |evt| room_description.set(evt.value())
                    }

                    div {
                        class: "form-actions",

                        button {
                            class: "create-button",
                            disabled: creating() || room_name().trim().is_empty(),
                            onclick: move |_| {
                                if !room_name().trim().is_empty() && !creating() {
                                    let name = room_name().clone();
                                    let description = if room_description().trim().is_empty() {
                                        None
                                    } else {
                                        Some(room_description().clone())
                                    };
                                    creating.set(true);
                                    let on_created = props.on_room_created;
                                    spawn(async move {
                                        match create_room(name, description).await {
                                            Ok(_) => {
                                                room_name.set(String::new());
                                                room_description.set(String::new());
                                                show_form.set(false);
                                                creating.set(false);
                                                on_created.call(());
                                            }
                                            Err(_) => {
                                                creating.set(false);
                                            }
                                        }
                                    });
                                }
                            },

                            if creating() {
                                "{props.i18n.translate(\"rooms.creating\")}"
                            } else {
                                "{props.i18n.translate(\"rooms.create\")}"
                            }
                        }

                        button {
                            class: "cancel-button",
                            onclick: move |_| {
                                show_form.set(false);
                                room_name.set(String::new());
                                room_description.set(String::new());
                            },

                            "{props.i18n.translate(\"rooms.cancel\")}"
                        }
                    }
                }
            } else {
                // Default create room card
                div {
                    class: "create-icon-container",
                    onclick: move |_| show_form.set(true),

                    Icon {
                        name: IconName::Plus,
                        i18n: props.i18n.clone(),
                        class: "create-icon".to_string()
                    }
                }

                h3 {
                    class: "create-title",
                    onclick: move |_| show_form.set(true),
                    "{props.i18n.translate(\"rooms.create_new\")}"
                }

                p {
                    class: "create-subtitle",
                    onclick: move |_| show_form.set(true),
                    "{props.i18n.translate(\"room_dashboard.create_room_subtitle\")}"
                }
            }
        }
    }
}
