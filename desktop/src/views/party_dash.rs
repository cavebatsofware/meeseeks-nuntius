use dioxus::prelude::*;
use ui::I18nContext;
use api::{create_room, get_all_rooms};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RoomData {
    id: Option<String>,
    name: String,
    description: Option<String>,
    member_count: Option<u32>,
}

const DASHBOARD_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1H7V7H1V1ZM9 1H15V7H9V1ZM1 9H7V15H1V9ZM9 9H15V15H9V9Z' fill='%23ffffff'/%3E%3C/svg%3E";
const SEARCH_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M7 12A5 5 0 1 0 7 2a5 5 0 0 0 0 10ZM13 13l-3-3' stroke='%23ffffff' stroke-width='1.5' stroke-linecap='round'/%3E%3C/svg%3E";
const PLUS_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M8 3.5V12.5M3.5 8H12.5' stroke='%23ffffff' stroke-width='1.5' stroke-linecap='round'/%3E%3C/svg%3E";
const USERS_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M5.5 7A3 3 0 1 0 5.5 1a3 3 0 0 0 0 6ZM1 12v2h9v-2a3 3 0 0 0-3-3H4a3 3 0 0 0-3 3Zm10-4.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5ZM15 14v-1a2 2 0 0 0-1.18-1.83A3.01 3.01 0 0 1 15 14Z' fill='%23ffffff'/%3E%3C/svg%3E";
const SETTINGS_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z' fill='%23ffffff'/%3E%3Cpath d='M14 8a6 6 0 1 1-12 0 6 6 0 0 1 12 0Z' stroke='%23ffffff' stroke-width='1.5'/%3E%3C/svg%3E";
const BELL_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M8 2a6 6 0 0 0-6 6c0 7.3-3 9-3 9h18s-3-1.7-3-9a6 6 0 0 0-6-6Z' fill='%23ffffff'/%3E%3Cpath d='M9 17a1 1 0 0 1-2 0' fill='%23ffffff'/%3E%3C/svg%3E";
const ARROW_RIGHT_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M6 4L10 8L6 12' stroke='%23ffffff' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E";
const DOTS_VERTICAL_ICON: &str = "data:image/svg+xml,%3Csvg width='16' height='16' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Ccircle cx='8' cy='3' r='1.5' fill='%23ffffff'/%3E%3Ccircle cx='8' cy='8' r='1.5' fill='%23ffffff'/%3E%3Ccircle cx='8' cy='13' r='1.5' fill='%23ffffff'/%3E%3C/svg%3E";

// these need to get turned into icon components before I do other pages
const IMG_GROUP: &str = DASHBOARD_ICON; // needs the real app logo
const IMG_FRAME: &str = DASHBOARD_ICON;
const IMG_FRAME1: &str = PLUS_ICON;
const IMG_FRAME2: &str = USERS_ICON;
const IMG_FRAME3: &str = SETTINGS_ICON;
const IMG_FRAME4: &str = SETTINGS_ICON;
const IMG_FRAME5: &str = SETTINGS_ICON;
const IMG_FRAME6: &str = SEARCH_ICON;
const IMG_FRAME7: &str = BELL_ICON;
const IMG_FRAME8: &str = ARROW_RIGHT_ICON;
const IMG_FRAME9: &str = PLUS_ICON;
const IMG_FRAME10: &str = DOTS_VERTICAL_ICON;
const IMG_IMG: &str = "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face&auto=format"; // Placeholder avatar

#[derive(Props, Clone, PartialEq)]
pub struct PartyDashboardProps {
    #[props(default = "Thom T.".to_string())]
    username: String,
    #[props(default = "View Profile".to_string())]
    user_subtitle: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[component]
pub fn PartyDashboard(props: PartyDashboardProps) -> Element {
    let mut rooms = use_signal(|| Vec::<RoomData>::new());
    let mut loading_rooms = use_signal(|| true);

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
            match get_all_rooms().await {
                Ok(room_jsons) => {
                    let mut parsed_rooms = Vec::new();
                    for room_json in room_jsons {
                        if let Ok(room_data) = serde_json::from_str::<RoomData>(&room_json) {
                            parsed_rooms.push(room_data);
                        }
                    }
                    rooms.set(parsed_rooms);
                }
                Err(_) => {}
            }
        });
    };
    rsx! {
        div {
            class: "dashboard-container",

            // Sidebar
            aside {
                class: "sidebar",

                // Header section
                div {
                    class: "sidebar-header",

                    // Logo
                    div {
                        class: "logo-container",
                        img {
                            src: IMG_GROUP,
                            alt: "Cavebat Logo",
                            class: "logo-image"
                        }
                    }

                    // Brand name
                    h1 {
                        class: "brand-name",
                        "Cavebat"
                    }
                }

                // Navigation
                nav {
                    class: "sidebar-nav",

                    // Active menu item - Party Dashboard
                    div {
                        class: "nav-item active",

                        img {
                            src: IMG_FRAME,
                            alt: "Dashboard Icon",
                            class: "nav-icon"
                        }

                        span {
                            class: "nav-text",
                            "{props.i18n.translate(\"rooms.dashboard\")}"
                        }
                    }

                    // Other menu items
                    MenuItemComponent { icon: IMG_FRAME2, text: props.i18n.translate("rooms.select") }
                    MenuItemComponent { icon: IMG_FRAME3, text: props.i18n.translate("rooms.management") }
                    MenuItemComponent { icon: IMG_FRAME4, text: props.i18n.translate("nav.settings") }
                }

                // User profile section
                div {
                    class: "user-profile",

                    div {
                        class: "user-info",

                        // User avatar
                        div {
                            class: "user-avatar"
                        }

                        // User info
                        div {
                            class: "user-details",

                            div {
                                class: "user-name",
                                "{props.username}"
                            }

                            div {
                                class: "user-subtitle",
                                "{props.user_subtitle}"
                            }
                        }

                        // Settings button
                        button {
                            class: "settings-button",
                            img {
                                src: IMG_FRAME5,
                                alt: "Settings",
                                class: "settings-icon"
                            }
                        }
                    }
                }
            }

            // Main content
            main {
                class: "main-content",

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

                                img {
                                    src: IMG_FRAME6,
                                    alt: "Search",
                                    class: "search-icon"
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

                                img {
                                    src: IMG_FRAME7,
                                    alt: "Notifications",
                                    class: "notification-icon"
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
                                        title: room.name.clone(),
                                        description: room.description.as_ref()
                                            .filter(|desc| !desc.is_empty())
                                            .cloned()
                                            .unwrap_or_else(|| props.i18n.translate("rooms.default_description")),
                                        badge_text: "",
                                        badge_color: "",
                                        member_count: room.member_count
                                            .filter(|&count| count > 0)
                                            .map(|count| format!("+{}", count))
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
                                party: "Project Overlord",
                                message: "The latest schematics are ready for review. Please check the encrypted folder.",
                                time: "2 min ago",
                                is_first: true,
                                i18n: props.i18n.clone()
                            }

                            MessageComponent {
                                avatar_color: "blue",
                                username: "C. Kent",
                                party: "Stealth Ops",
                                message: "Confirmed. Target acquired. Awaiting final command for phase two.",
                                time: "15 min ago",
                                is_first: false,
                                i18n: props.i18n.clone()
                            }

                            MessageComponent {
                                avatar_color: "blue",
                                username: "Barbara G.",
                                party: "Project Overlord",
                                message: "I've bypassed their firewall. The data stream is now open. Be quick.",
                                time: "48 min ago",
                                is_first: false,
                                i18n: props.i18n.clone()
                            }

                            MessageComponent {
                                avatar_color: "blue",
                                username: "Victor S.",
                                party: "R&D Division",
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
}

#[derive(Props, Clone, PartialEq)]
struct MenuItemProps {
    icon: &'static str,
    text: String,
}

#[component]
fn MenuItemComponent(props: MenuItemProps) -> Element {
    rsx! {
        div {
            class: "nav-item",

            img {
                src: props.icon,
                alt: props.text,
                class: "nav-icon"
            }

            span {
                class: "nav-text",
                "{props.text}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PartyCardProps {
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
            class: "party-card",

            // Header with title and badge
            div {
                class: "party-header",

                h3 {
                    class: "party-title",
                    "{props.title}"
                }

                if !props.badge_text.is_empty() {
                    span {
                        class: "party-badge badge-{props.badge_color}",
                        "{props.badge_text}"
                    }
                }
            }

            // Description
            p {
                class: "party-description",
                "{props.description}"
            }

            // Footer with avatars and enter button
            div {
                class: "party-footer",

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
                button {
                    class: "enter-button",

                    "{props.i18n.translate(\"room_dashboard.enter\")}"

                    img {
                        src: IMG_FRAME8,
                        alt: "Enter",
                        class: "enter-icon"
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
    party: String,
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

                // Header with username, party, and time
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
                            class: "party-name",
                            "{props.party}"
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

                img {
                    src: IMG_FRAME10,
                    alt: "Menu",
                    class: "menu-icon"
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
    let mut room_name = use_signal(|| String::new());
    let mut room_description = use_signal(|| String::new());
    let mut show_form = use_signal(|| false);
    let mut creating = use_signal(|| false);

    rsx! {
        div {
            class: "create-party-card",

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
                                let on_created = props.on_room_created.clone();
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
                                    let on_created = props.on_room_created.clone();
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

                    img {
                        src: IMG_FRAME9,
                        alt: "Add",
                        class: "create-icon"
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
