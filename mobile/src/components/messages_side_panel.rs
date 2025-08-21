use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

#[derive(Clone, PartialEq)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub status: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct SharedFile {
    pub id: String,
    pub name: String,
    pub file_type: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct MessagesSidePanelProps {
    pub room_name: String,
    pub members: Vec<Member>,
    pub shared_files: Vec<SharedFile>,
    pub i18n: I18nContext,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn MessagesSidePanel(props: MessagesSidePanelProps) -> Element {
    rsx! {
        aside {
            class: "mm-side-panel",
            div {
                class: "mm-side-content",

                // Header
                div {
                    class: "mm-side-header",
                    h2 {
                        class: "mm-side-title",
                        "Room Details"
                    }
                    button {
                        class: "mm-close-side-btn",
                        onclick: move |_| props.on_close.call(()),
                        Icon {
                            name: IconName::X,
                            i18n: props.i18n.clone(),
                            class: "mm-close-side-icon".to_string()
                        }
                    }
                }

                // Room info
                div {
                    class: "mm-side-room-info",
                    div {
                        class: "mm-side-room-avatar",
                        "{props.room_name.chars().next().unwrap_or('R')}"
                    }
                    h3 {
                        class: "mm-side-room-name",
                        "{props.room_name}"
                    }
                    p {
                        class: "mm-side-room-desc",
                        "New Room on the Mini"
                    }
                    p {
                        class: "mm-side-room-members",
                        "{props.members.len()} members"
                    }
                }

                // Members section
                div {
                    class: "mm-members-section",
                    h4 {
                        class: "mm-section-header",
                        "Members"
                    }
                    div {
                        class: "mm-members-list",
                        for member in props.members {
                            MemberItem {
                                key: "{member.id}",
                                member: member.clone(),
                                i18n: props.i18n.clone()
                            }
                        }
                    }
                }

                // Shared files section
                div {
                    class: "mm-files-section",
                    h4 {
                        class: "mm-section-header",
                        "Shared Files"
                    }
                    div {
                        class: "mm-files-list",
                        for file in props.shared_files {
                            FileItem {
                                key: "{file.id}",
                                file: file.clone(),
                                i18n: props.i18n.clone()
                            }
                        }
                    }
                }

                // Action buttons
                div {
                    class: "mm-actions-section",
                    button {
                        class: "mm-action-button primary",
                        onclick: move |_| {
                            println!("Start call");
                        },
                        Icon {
                            name: IconName::Phone,
                            i18n: props.i18n.clone(),
                            class: "mm-action-button-icon".to_string()
                        }
                        "Start Call"
                    }
                    button {
                        class: "mm-action-button secondary",
                        onclick: move |_| {
                            println!("Add members");
                        },
                        Icon {
                            name: IconName::UserPlus,
                            i18n: props.i18n.clone(),
                            class: "mm-action-button-icon".to_string()
                        }
                        "Add Members"
                    }
                    button {
                        class: "mm-action-button secondary",
                        onclick: move |_| {
                            println!("Room settings");
                        },
                        Icon {
                            name: IconName::Settings,
                            i18n: props.i18n.clone(),
                            class: "mm-action-button-icon".to_string()
                        }
                        "Room Settings"
                    }
                    button {
                        class: "mm-action-button danger",
                        onclick: move |_| {
                            println!("Leave room");
                        },
                        Icon {
                            name: IconName::LogOut,
                            i18n: props.i18n.clone(),
                            class: "mm-action-button-icon".to_string()
                        }
                        "Leave Room"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MemberItemProps {
    member: Member,
    i18n: I18nContext,
}

#[component]
fn MemberItem(props: MemberItemProps) -> Element {
    rsx! {
        div {
            class: format!("mm-member-item {}", if props.member.status == "offline" { "offline" } else { "" }),
            div {
                class: "mm-member-avatar",
                style: if let Some(avatar_url) = &props.member.avatar_url {
                    format!("background-image: url('{}')", avatar_url)
                } else {
                    format!("background-color: var(--color-bg-quaternary)")
                }
            }
            div {
                class: "mm-member-info",
                p {
                    class: "mm-member-name",
                    "{props.member.name}"
                }
            }
            p {
                class: format!("mm-member-status {}", props.member.status),
                match props.member.status.as_str() {
                    "online" => "Online",
                    "away" => "Away",
                    "offline" => "2h ago",
                    _ => &props.member.status
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FileItemProps {
    file: SharedFile,
    i18n: I18nContext,
}

#[component]
fn FileItem(props: FileItemProps) -> Element {
    let icon_name = match props.file.file_type.as_str() {
        "pdf" => IconName::FileText,
        "image" => IconName::Image,
        "spreadsheet" => IconName::FileSpreadsheet,
        _ => IconName::File,
    };

    rsx! {
        div {
            class: "mm-file-item",
            onclick: move |_| {
                println!("Open file: {}", props.file.name);
            },
            Icon {
                name: icon_name,
                i18n: props.i18n.clone(),
                class: "mm-file-icon".to_string()
            }
            p {
                class: "mm-file-name",
                "{props.file.name}"
            }
        }
    }
}
