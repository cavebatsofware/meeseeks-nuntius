use crate::components::MobileLayout;
use api::local::{create_room, get_all_rooms};
use dioxus::prelude::*;
use ui::{get_language_name, get_text_direction, I18nContext, Icon, IconName, RoomData};

const MOBILE_ROOM_DASH_CSS: Asset = asset!("/assets/mobile_room_dash.css");

#[derive(Props, Clone, PartialEq)]
pub struct MobileRoomDashboardProps {
    #[props(default = "Thom T.".to_string())]
    username: String,
    #[props(default = "View Profile".to_string())]
    user_subtitle: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[component]
pub fn MobileRoomDashboard(props: MobileRoomDashboardProps) -> Element {
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut rooms = use_signal(|| Vec::<RoomData>::new());
    let mut loading_rooms = use_signal(|| true);
    let mut active_tab = use_signal(|| "rooms".to_string());
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
        document::Stylesheet { href: MOBILE_ROOM_DASH_CSS }

        MobileLayout {
            i18n: props.i18n.clone(),
            brand_name: "Cavebat".to_string(),
            active_tab: active_tab(),
            has_notifications: true,
            notification_count: 3,
            on_tab_change: move |tab: String| {
                active_tab.set(tab);
            },

            // Header section content
            div {
                class: "mrd-header-content",

                div {
                    class: "mrd-greeting-section",
                    h1 {
                        class: "mrd-greeting-title",
                        "{props.i18n.translate(\"room_dashboard.title\")}"
                    }
                    p {
                        class: "mrd-greeting-subtitle",
                        "{props.i18n.translate(\"room_dashboard.subtitle\")}"
                    }
                }
            }

            // Top Rooms Section
            section {
                class: "mrd-rooms-section",

                div {
                    class: "mrd-section-header",
                    h2 {
                        class: "mrd-section-title",
                        "{props.i18n.translate(\"room_dashboard.sections.top_rooms\")}"
                    }
                }

                div {
                    class: "mrd-rooms-scroll",
                    if loading_rooms() {
                        div {
                            class: "mrd-loading-rooms",
                            "{props.i18n.translate(\"rooms.loading\")}"
                        }
                    } else {
                        // Dynamic rooms from database
                        for room in rooms() {
                            MobileRoomCard {
                                key: "{room.id.as_deref().unwrap_or(&room.name)}",
                                room_id: room.id.as_deref().unwrap_or(&room.name).to_string(),
                                title: room.name.clone(),
                                description: room.description.as_ref()
                                    .filter(|desc| !desc.is_empty())
                                    .cloned()
                                    .unwrap_or_else(|| props.i18n.translate("rooms.default_description")),
                                member_count: room.member_count.unwrap_or(0) as i32,
                                i18n: props.i18n.clone()
                            }
                        }

                        // Create New Room Card
                        MobileCreateRoomCard {
                            i18n: props.i18n.clone(),
                            on_room_created: refresh_rooms
                        }
                    }
                }
            }

            // Recent Messages Section
            section {
                class: "mrd-messages-section",

                h2 {
                    class: "mrd-section-title",
                    "{props.i18n.translate(\"room_dashboard.sections.latest_messages\")}"
                }

                div {
                    class: "mrd-messages-list",
                    MobileMessageCard {
                        avatar_color: "purple",
                        username: "Diana P.",
                        room: "Project Overlord",
                        message: "The latest schematics are ready for review. Please check the encrypted folder.",
                        time: "2 min ago",
                        unread_count: 3,
                        i18n: props.i18n.clone()
                    }

                    MobileMessageCard {
                        avatar_color: "blue",
                        username: "C. Kent",
                        room: "Stealth Ops",
                        message: "Confirmed. Target acquired. Awaiting final command for phase two.",
                        time: "15 min ago",
                        unread_count: 1,
                        i18n: props.i18n.clone()
                    }

                    MobileMessageCard {
                        avatar_color: "green",
                        username: "Barbara G.",
                        room: "Project Overlord",
                        message: "I've bypassed their firewall. The data stream is now open. Be quick.",
                        time: "48 min ago",
                        unread_count: 0,
                        i18n: props.i18n.clone()
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MobileRoomCardProps {
    room_id: String,
    title: String,
    description: String,
    member_count: i32,
    i18n: I18nContext,
}

#[component]
fn MobileRoomCard(props: MobileRoomCardProps) -> Element {
    rsx! {
        div {
            class: "mrd-room-card",

            div {
                class: "mrd-room-header",
                h3 {
                    class: "mrd-room-title",
                    "{props.title}"
                }
                div {
                    class: "mrd-member-count",
                    Icon {
                        name: IconName::Users,
                        i18n: props.i18n.clone(),
                        class: "mrd-member-icon".to_string()
                    }
                    span {
                        class: "mrd-member-text",
                        "{props.member_count}"
                    }
                }
            }

            p {
                class: "mrd-room-description",
                "{props.description}"
            }

            div {
                class: "mrd-room-footer",
                div {
                    class: "mrd-member-avatars",
                    div { class: "mrd-avatar mrd-avatar-1" }
                    div { class: "mrd-avatar mrd-avatar-2" }
                    div { class: "mrd-avatar mrd-avatar-3" }
                    if props.member_count > 3 {
                        div {
                            class: "mrd-avatar mrd-avatar-more",
                            "+{props.member_count - 3}"
                        }
                    }
                }

                button {
                    class: "mrd-enter-btn",
                    "{props.i18n.translate(\"room_dashboard.enter\")}"
                    Icon {
                        name: IconName::ArrowRight,
                        i18n: props.i18n.clone(),
                        class: "mrd-enter-icon".to_string()
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MobileCreateRoomCardProps {
    i18n: I18nContext,
    on_room_created: EventHandler<()>,
}

#[component]
fn MobileCreateRoomCard(props: MobileCreateRoomCardProps) -> Element {
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut room_name = use_signal(|| String::new());
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut room_description = use_signal(|| String::new());
    let mut show_form = use_signal(|| false);
    let mut creating = use_signal(|| false);

    rsx! {
        div {
            class: "mrd-create-room-card",

            if show_form() {
                // Mobile room creation form (simplified)
                div {
                    class: "mrd-create-form",

                    input {
                        r#type: "text",
                        placeholder: "{props.i18n.translate(\"rooms.enter_name\")}",
                        class: "mrd-room-name-input",
                        value: "{room_name()}",
                        oninput: move |evt| room_name.set(evt.value())
                    }

                    textarea {
                        placeholder: "{props.i18n.translate(\"rooms.enter_description\")}",
                        class: "mrd-room-description-input",
                        value: "{room_description()}",
                        oninput: move |evt| room_description.set(evt.value())
                    }

                    div {
                        class: "mrd-form-actions",

                        button {
                            class: "mrd-create-btn",
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
                            class: "mrd-cancel-btn",
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
                    class: "mrd-create-icon-container",
                    onclick: move |_| show_form.set(true),
                    Icon {
                        name: IconName::Plus,
                        i18n: props.i18n.clone(),
                        class: "mrd-create-icon".to_string()
                    }
                }

                h3 {
                    class: "mrd-create-title",
                    onclick: move |_| show_form.set(true),
                    "{props.i18n.translate(\"rooms.create_new\")}"
                }

                p {
                    class: "mrd-create-subtitle",
                    onclick: move |_| show_form.set(true),
                    "{props.i18n.translate(\"room_dashboard.create_room_subtitle\")}"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MobileMessageCardProps {
    avatar_color: &'static str,
    username: String,
    room: String,
    message: String,
    time: String,
    unread_count: i32,
    i18n: I18nContext,
}

#[component]
fn MobileMessageCard(props: MobileMessageCardProps) -> Element {
    rsx! {
        div {
            class: "mrd-message-card",

            div {
                class: "mrd-message-header",
                div {
                    class: "mrd-message-avatar mrd-avatar-{props.avatar_color}",
                    "{props.username.chars().next().unwrap_or('U')}"
                }

                div {
                    class: "mrd-message-info",
                    div {
                        class: "mrd-message-meta",
                        span {
                            class: "mrd-username",
                            "{props.username}"
                        }
                        span {
                            class: "mrd-separator",
                            "{props.i18n.translate(\"room_dashboard.separator\")}"
                        }
                        span {
                            class: "mrd-room-name",
                            "{props.room}"
                        }
                    }
                    span {
                        class: "mrd-message-time",
                        "{props.time}"
                    }
                }

                if props.unread_count > 0 {
                    div {
                        class: "mrd-unread-badge",
                        "{props.unread_count}"
                    }
                }
            }

            p {
                class: "mrd-message-text",
                "{props.message}"
            }
        }
    }
}
