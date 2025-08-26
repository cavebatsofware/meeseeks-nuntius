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

use crate::Route;
use dioxus::prelude::*;
use shared::local::get_room;
use ui::{I18nContext, RoomData, UserProfileMini};

const MESSAGES_CSS: Asset = asset!("/assets/messages.css");

#[derive(Props, Clone, PartialEq)]
pub struct MessagesProps {
    room_id: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[component]
pub fn Messages(props: MessagesProps) -> Element {
    let mut room_data = use_signal(|| Option::<RoomData>::None);
    let mut loading = use_signal(|| true);
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut message_input = use_signal(|| String::new());

    // Load room data on component initialization
    use_effect(move || {
        let room_id = props.room_id.clone();
        spawn(async move {
            match get_room(room_id).await {
                Ok(Some(room_json)) => {
                    if let Ok(room) = serde_json::from_str::<RoomData>(&room_json) {
                        room_data.set(Some(room));
                    }
                    loading.set(false);
                }
                Ok(None) => {
                    loading.set(false);
                }
                Err(_) => {
                    loading.set(false);
                }
            }
        });
    });

    if loading() {
        return rsx! {
            div {
                class: "messages-container loading",
                "Loading room..."
            }
        };
    }

    let Some(room) = room_data() else {
        return rsx! {
            div {
                class: "messages-container error",
                "Room not found"
            }
        };
    };

    rsx! {
        document::Stylesheet { href: MESSAGES_CSS }

        div {
            class: "messages-view-container",

            // Left sidebar (aside) - Room list and navigation
            aside {
                class: "messages-sidebar",

                // Sidebar header with back button
                header {
                    class: "messages-sidebar-header",

                    Link {
                        class: "back-to-dashboard",
                        to: Route::RoomDashboard {},
                        "← Back to Dashboard"
                    }
                }

                // Room list in sidebar
                div {
                    class: "sidebar-rooms",
                    h3 { class: "sidebar-section-title", "Recent Rooms" }

                    // Sample rooms in sidebar - these would be dynamic
                    div { class: "sidebar-room-item active", "🏠 General" }
                    div { class: "sidebar-room-item", "💼 Project Team" }
                    div { class: "sidebar-room-item", "🔐 Security Discuss" }
                    div { class: "sidebar-room-item", "🚀 Development" }
                }

                // Quick actions
                div {
                    class: "sidebar-actions",
                    button { class: "sidebar-button", "➕ New Room" }
                    button { class: "sidebar-button", "👥 Contacts" }
                    button { class: "sidebar-button", "⚙️ Settings" }
                }

                // User profile section at bottom
                UserProfileMini {
                    username: "User".to_string(),
                    status: "Online".to_string(),
                    avatar_initial: "U".to_string(),
                    i18n: props.i18n.clone()
                }
            }

            // Main messaging area
            main {
                class: "messages-main-area",

                // Message header
                header {
                class: "messages-header",

                div {
                    class: "room-info",

                    div {
                        class: "room-avatar",
                        // Room avatar placeholder
                    }

                    div {
                        class: "room-details",
                        h1 {
                            class: "room-name",
                            "{room.name}"
                        }
                        p {
                            class: "room-status",
                            if let Some(count) = room.member_count {
                                if count > 0 {
                                    "{count} members • Active now"
                                } else {
                                    "Active now"
                                }
                            } else {
                                "Active now"
                            }
                        }
                    }
                }

                div {
                    class: "header-actions",

                    button {
                        class: "header-button",
                        "title": "{props.i18n.translate(\"messages.search\")}",
                        // Search icon
                        "🔍"
                    }

                    button {
                        class: "header-button",
                        "title": "{props.i18n.translate(\"messages.call\")}",
                        // Call icon
                        "📞"
                    }

                    button {
                        class: "header-button",
                        "title": "{props.i18n.translate(\"messages.more\")}",
                        // More options icon
                        "⋯"
                    }
                }
            }

            // Messages area
            main {
                class: "messages-main",

                div {
                    class: "messages-list",

                    // Sample messages - these will be dynamic later
                    MessageComponent {
                        sender_name: "Alice".to_string(),
                        message: "Hey everyone! How's the project going?".to_string(),
                        timestamp: "10:30 AM".to_string(),
                        is_own: false,
                        i18n: props.i18n.clone()
                    }

                    MessageComponent {
                        sender_name: "You".to_string(),
                        message: "Great! Just finished the authentication module. The encryption is working perfectly.".to_string(),
                        timestamp: "10:32 AM".to_string(),
                        is_own: true,
                        i18n: props.i18n.clone()
                    }

                    MessageComponent {
                        sender_name: "Bob".to_string(),
                        message: "Awesome work! I've been working on the UI components. Should have the messaging interface ready soon.".to_string(),
                        timestamp: "10:35 AM".to_string(),
                        is_own: false,
                        i18n: props.i18n.clone()
                    }

                    MessageComponent {
                        sender_name: "You".to_string(),
                        message: "Looking forward to seeing it! The backend API is almost complete too.".to_string(),
                        timestamp: "10:37 AM".to_string(),
                        is_own: true,
                        i18n: props.i18n.clone()
                    }
                }
            }

                // Input area
                footer {
                    class: "messages-input-area",

                    div {
                        class: "input-container",

                        button {
                            class: "attachment-button",
                            "title": "{props.i18n.translate(\"messages.attach\")}",
                            "📎"
                        }

                        input {
                            r#type: "text",
                            class: "message-input",
                            placeholder: "{props.i18n.translate(\"messages.placeholder\")}",
                            value: "{message_input()}",
                            oninput: move |evt| message_input.set(evt.value()),
                            onkeypress: move |evt| {
                                if evt.key() == Key::Enter && !message_input().trim().is_empty() {
                                    // TODO: Send message logic here
                                    message_input.set(String::new());
                                }
                            }
                        }

                        button {
                            class: "emoji-button",
                            "title": "{props.i18n.translate(\"messages.emoji\")}",
                            "😊"
                        }

                        button {
                            class: "send-button",
                            disabled: message_input().trim().is_empty(),
                            onclick: move |_| {
                                if !message_input().trim().is_empty() {
                                    // TODO: Send message logic here
                                    message_input.set(String::new());
                                }
                            },
                            "Send"
                        }
                    }
                }
            }

            // Right section (details panel) - Room info and members
            section {
                class: "messages-details-panel",

                // Panel header
                header {
                    class: "details-header",
                    h3 { "Room Details" }
                    button {
                        class: "panel-toggle",
                        "✕"
                    }
                }

                // Room information
                div {
                    class: "room-details-section",

                    div {
                        class: "room-info-detailed",
                        div {
                            class: "room-avatar-large",
                            "{room.name.chars().next().unwrap_or('R')}"
                        }
                        h3 { class: "room-name-large", "{room.name}" }
                        p {
                            class: "room-description-detailed",
                            if let Some(desc) = &room.description {
                                "{desc}"
                            } else {
                                "No description available"
                            }
                        }
                        p {
                            class: "room-member-count",
                            if let Some(count) = room.member_count {
                                "{count} members"
                            } else {
                                "No members"
                            }
                        }
                    }
                }

                // Members list
                div {
                    class: "members-section",
                    h4 { "Members" }

                    div { class: "member-item online", "👤 Alice Johnson • Online" }
                    div { class: "member-item online", "👤 Bob Smith • Online" }
                    div { class: "member-item away", "👤 Charlie Davis • Away" }
                    div { class: "member-item offline", "👤 Diana Prince • 2h ago" }
                }

                // Shared files/media
                div {
                    class: "shared-content-section",
                    h4 { "Shared Files" }

                    div { class: "shared-item", "📄 Project_Plan.pdf" }
                    div { class: "shared-item", "🖼️ screenshot_001.png" }
                    div { class: "shared-item", "📊 analytics_report.xlsx" }
                }

                // Room actions
                div {
                    class: "room-actions-section",
                    button { class: "detail-action-btn primary", "📞 Start Call" }
                    button { class: "detail-action-btn secondary", "👥 Add Members" }
                    button { class: "detail-action-btn secondary", "⚙️ Room Settings" }
                    button { class: "detail-action-btn danger", "🚪 Leave Room" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MessageProps {
    sender_name: String,
    message: String,
    timestamp: String,
    is_own: bool,
    i18n: I18nContext,
}

#[component]
fn MessageComponent(props: MessageProps) -> Element {
    rsx! {
        div {
            class: if props.is_own { "message own-message" } else { "message other-message" },

            if !props.is_own {
                div {
                    class: "message-avatar",
                    // Avatar placeholder
                    "{props.sender_name.chars().next().unwrap_or('?')}"
                }
            }

            div {
                class: "message-content",

                if !props.is_own {
                    div {
                        class: "message-sender",
                        "{props.sender_name}"
                    }
                }

                div {
                    class: "message-bubble",
                    p {
                        class: "message-text",
                        "{props.message}"
                    }
                }

                div {
                    class: "message-timestamp",
                    "{props.timestamp}"
                }
            }
        }
    }
}
