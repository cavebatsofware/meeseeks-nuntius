use crate::components::messages_side_panel::{Member, SharedFile};
use crate::components::{MessagesSidePanel, MobileLayout};
use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

const MOBILE_MESSAGES_CSS: Asset = asset!("/assets/mobile_messages.css");

#[derive(Props, Clone, PartialEq)]
pub struct MobileMessagesProps {
    #[props(default = "1".to_string())]
    pub room_id: String,
    #[props(default = "First Mac Room".to_string())]
    pub room_name: String,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = false)]
    pub show_side_panel: bool,
}

#[derive(Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: String,
    pub is_sent: bool,
}

#[component]
pub fn MobileMessages(props: MobileMessagesProps) -> Element {
    let mut message_input = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "chat".to_string());
    let mut show_side_panel = use_signal(|| props.show_side_panel);

    // Sample messages data
    let mut messages = use_signal(|| vec![
        Message {
            id: "1".to_string(),
            sender_id: "user1".to_string(),
            sender_name: "A".to_string(),
            content: "Hey everyone! How's the project going?".to_string(),
            timestamp: "10:30 AM".to_string(),
            is_sent: false,
        },
        Message {
            id: "2".to_string(),
            sender_id: "current_user".to_string(),
            sender_name: "You".to_string(),
            content: "Great! Just finished the authentication module. The encryption is working perfectly.".to_string(),
            timestamp: "10:32 AM".to_string(),
            is_sent: true,
        },
        Message {
            id: "3".to_string(),
            sender_id: "user2".to_string(),
            sender_name: "B".to_string(),
            content: "Awesome work! I've been working on the UI components. Should have the messaging interface ready soon.".to_string(),
            timestamp: "10:35 AM".to_string(),
            is_sent: false,
        },
        Message {
            id: "4".to_string(),
            sender_id: "current_user".to_string(),
            sender_name: "You".to_string(),
            content: "Looking forward to seeing it! The backend API is almost complete too.".to_string(),
            timestamp: "10:37 AM".to_string(),
            is_sent: true,
        },
    ]);

    // Sample members data
    let members = use_memo(move || {
        vec![
            Member {
                id: "1".to_string(),
                name: "Alice Johnson".to_string(),
                status: "online".to_string(),
                avatar_url: None,
            },
            Member {
                id: "2".to_string(),
                name: "Bob Smith".to_string(),
                status: "online".to_string(),
                avatar_url: None,
            },
            Member {
                id: "3".to_string(),
                name: "Charlie Davis".to_string(),
                status: "away".to_string(),
                avatar_url: None,
            },
            Member {
                id: "4".to_string(),
                name: "Diana Prince".to_string(),
                status: "offline".to_string(),
                avatar_url: None,
            },
        ]
    });

    // Sample shared files
    let shared_files = use_memo(move || {
        vec![
            SharedFile {
                id: "1".to_string(),
                name: "Project_Plan.pdf".to_string(),
                file_type: "pdf".to_string(),
            },
            SharedFile {
                id: "2".to_string(),
                name: "screenshot_001.png".to_string(),
                file_type: "image".to_string(),
            },
            SharedFile {
                id: "3".to_string(),
                name: "analytics_report.xlsx".to_string(),
                file_type: "spreadsheet".to_string(),
            },
        ]
    });

    rsx! {
        document::Stylesheet { href: MOBILE_MESSAGES_CSS }

        MobileLayout {
            i18n: props.i18n.clone(),
            brand_name: props.room_name.clone(),
            active_tab: active_tab(),
            has_notifications: false,
            notification_count: 0,
            on_tab_change: move |tab: String| {
                active_tab.set(tab);
            },
            custom_header: Some(rsx! {
                div {
                    class: "mm-header-custom",
                    div {
                        class: "mm-header-left",
                        button {
                            class: "mm-back-btn",
                            onclick: move |_| {
                                let nav = navigator();
                                nav.push(crate::Route::Home {});
                            },
                            Icon {
                                name: IconName::ArrowLeft,
                                i18n: props.i18n.clone(),
                                class: "mm-back-icon".to_string()
                            }
                        }

                        div {
                            class: "mm-room-info",
                            div {
                                class: "mm-room-avatar",
                                "{props.room_name.chars().next().unwrap_or('R')}"
                            }
                            div {
                                class: "mm-room-details",
                                p {
                                    class: "mm-room-status",
                                    "Active now"
                                }
                            }
                        }
                    }

                    div {
                        class: "mm-header-actions",
                        button {
                            class: "mm-action-btn",
                            onclick: move |_| {
                                println!("Start call");
                            },
                            Icon {
                                name: IconName::Phone,
                                i18n: props.i18n.clone(),
                                class: "mm-action-icon".to_string()
                            }
                        }
                        button {
                            class: "mm-action-btn",
                            onclick: move |_| {
                                println!("Start video call");
                            },
                            Icon {
                                name: IconName::Video,
                                i18n: props.i18n.clone(),
                                class: "mm-action-icon".to_string()
                            }
                        }
                        button {
                            class: "mm-action-btn",
                            onclick: move |_| {
                                show_side_panel.set(!show_side_panel());
                            },
                            Icon {
                                name: IconName::DotsVertical,
                                i18n: props.i18n.clone(),
                                class: "mm-action-icon".to_string()
                            }
                        }
                    }
                }
            }),

            // Messages content
            div {
                class: "mm-messages-content",

                // Messages container
                div {
                    class: if show_side_panel() { "mm-with-side-panel" } else { "" },

                    // Main messages panel
                    div {
                        class: "mm-main-panel",

                        // Messages area
                        main {
                            class: "mm-messages-container",
                            for message in messages() {
                                if message.is_sent {
                                    MessageSent {
                                        key: "{message.id}",
                                        message: message.clone(),
                                        i18n: props.i18n.clone()
                                    }
                                } else {
                                    MessageReceived {
                                        key: "{message.id}",
                                        message: message.clone(),
                                        i18n: props.i18n.clone()
                                    }
                                }
                            }
                        }

                        // Message input
                        footer {
                            class: "mm-input-container",
                            div {
                                class: "mm-input-wrapper",
                                button {
                                    class: "mm-attach-btn",
                                    onclick: move |_| {
                                        println!("Attach file");
                                    },
                                    Icon {
                                        name: IconName::Paperclip,
                                        i18n: props.i18n.clone(),
                                        class: "mm-attach-icon".to_string()
                                    }
                                }

                                input {
                                    class: "mm-message-input",
                                    r#type: "text",
                                    placeholder: "Type your secure message...",
                                    value: "{message_input()}",
                                    oninput: move |evt| message_input.set(evt.value()),
                                    onkeypress: move |evt| {
                                        if evt.key() == Key::Enter {
                                            let content = message_input();
                                            if !content.trim().is_empty() {
                                                let mut current_messages = messages();
                                                let new_message = Message {
                                                    id: (current_messages.len() + 1).to_string(),
                                                    sender_id: "current_user".to_string(),
                                                    sender_name: "You".to_string(),
                                                    content: content.trim().to_string(),
                                                    timestamp: "now".to_string(),
                                                    is_sent: true,
                                                };
                                                current_messages.push(new_message);
                                                messages.set(current_messages);
                                                message_input.set(String::new());
                                            }
                                        }
                                    }
                                }

                                button {
                                    class: "mm-emoji-btn",
                                    onclick: move |_| {
                                        println!("Open emoji picker");
                                    },
                                    Icon {
                                        name: IconName::Smiley,
                                        i18n: props.i18n.clone(),
                                        class: "mm-emoji-icon".to_string()
                                    }
                                }

                                button {
                                    class: "mm-send-btn",
                                    disabled: message_input().trim().is_empty(),
                                    onclick: move |_| {
                                        let content = message_input();
                                        if !content.trim().is_empty() {
                                            let mut current_messages = messages();
                                            let new_message = Message {
                                                id: (current_messages.len() + 1).to_string(),
                                                sender_id: "current_user".to_string(),
                                                sender_name: "You".to_string(),
                                                content: content.trim().to_string(),
                                                timestamp: "now".to_string(),
                                                is_sent: true,
                                            };
                                            current_messages.push(new_message);
                                            messages.set(current_messages);
                                            message_input.set(String::new());
                                        }
                                    },
                                    "Send"
                                }
                            }
                        }
                    }

                    // Side panel
                    if show_side_panel() {
                        MessagesSidePanel {
                            room_name: props.room_name.clone(),
                            members: members(),
                            shared_files: shared_files(),
                            i18n: props.i18n.clone(),
                            on_close: move |_| show_side_panel.set(false),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MessageReceivedProps {
    message: Message,
    i18n: I18nContext,
}

#[component]
fn MessageReceived(props: MessageReceivedProps) -> Element {
    rsx! {
        div {
            class: "mm-message-group",
            div {
                class: "mm-message-received",
                div {
                    class: "mm-sender-avatar",
                    "{props.message.sender_name}"
                }
                div {
                    class: "mm-message-content-received",
                    div {
                        class: "mm-message-bubble-received",
                        p {
                            class: "mm-message-text",
                            "{props.message.content}"
                        }
                    }
                    p {
                        class: "mm-message-time",
                        "{props.message.timestamp}"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MessageSentProps {
    message: Message,
    i18n: I18nContext,
}

#[component]
fn MessageSent(props: MessageSentProps) -> Element {
    rsx! {
        div {
            class: "mm-message-group",
            div {
                class: "mm-message-sent",
                div {
                    class: "mm-message-content-sent",
                    div {
                        class: "mm-message-bubble-sent",
                        p {
                            class: "mm-message-text",
                            "{props.message.content}"
                        }
                    }
                    p {
                        class: "mm-message-time mm-message-time-sent",
                        "{props.message.timestamp}"
                    }
                }
            }
        }
    }
}
