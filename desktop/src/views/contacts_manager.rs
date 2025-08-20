use crate::DesktopLayout;
use dioxus::prelude::*;
use ui::{I18nContext, Icon, IconName};

const CONTACTS_CSS: Asset = asset!("/assets/contacts_manager.css");

#[derive(Props, Clone, PartialEq)]
pub struct ContactsManagerProps {
    #[props(default = "User".to_string())]
    username: String,
    #[props(default = "Online".to_string())]
    user_status: String,
    #[props(default = I18nContext::new("en"))]
    i18n: I18nContext,
}

#[derive(Clone, PartialEq)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub username: String,
    pub status: ContactStatus,
    pub avatar_initial: String,
    pub last_seen: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum ContactStatus {
    Online,
    Away,
    Offline,
    DoNotDisturb,
}

impl ContactStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactStatus::Online => "online",
            ContactStatus::Away => "away", 
            ContactStatus::Offline => "offline",
            ContactStatus::DoNotDisturb => "dnd",
        }
    }
}

#[component]
pub fn ContactsManager(props: ContactsManagerProps) -> Element {
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let contacts = use_signal(|| get_mock_contacts());
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut search_query = use_signal(|| String::new());
    let mut selected_contact = use_signal(|| Option::<String>::None);

    // Filter contacts based on search query
    let filtered_contacts = use_memo(move || {
        let query = search_query().to_lowercase();
        if query.is_empty() {
            contacts()
        } else {
            contacts()
                .into_iter()
                .filter(|contact| {
                    contact.name.to_lowercase().contains(&query) ||
                    contact.username.to_lowercase().contains(&query)
                })
                .collect()
        }
    });

    rsx! {
        document::Stylesheet { href: CONTACTS_CSS }

        DesktopLayout {
            i18n: props.i18n.clone(),
            username: props.username.clone(),
            user_status: props.user_status.clone(),
            user_avatar_initial: props.username.chars().next().unwrap_or('U').to_string(),
            brand_name: "Cavebat".to_string(),

            div {
                class: "cm-contacts-container",

                // Header
                header {
                    class: "cm-contacts-header",

                    div {
                        class: "cm-header-title",
                        h1 {
                            class: "cm-page-title",
                            "{props.i18n.translate(\"contacts.title\")}"
                        }
                        p {
                            class: "cm-page-subtitle",
                            "{props.i18n.translate(\"contacts.subtitle\")}"
                        }
                    }

                    // Search and actions section
                    div {
                        class: "cm-header-actions",

                        // Search bar
                        div {
                            class: "cm-search-container",
                            Icon {
                                name: IconName::Search,
                                i18n: props.i18n.clone(),
                                class: "cm-search-icon".to_string()
                            }
                            input {
                                r#type: "text",
                                placeholder: "{props.i18n.translate(\"contacts.search_placeholder\")}",
                                class: "cm-search-input",
                                value: "{search_query()}",
                                oninput: move |evt| search_query.set(evt.value())
                            }
                        }
                    }
                }

                // Add Contact Section
                div {
                    class: "cm-add-contact-section",
                    button {
                        class: "cm-add-contact-btn",
                        Icon {
                            name: IconName::Plus,
                            i18n: props.i18n.clone(),
                            class: "cm-add-icon".to_string()
                        }
                        "{props.i18n.translate(\"contacts.add_contact\")}"
                    }
                    p {
                        class: "cm-add-contact-subtitle",
                        "{props.i18n.translate(\"contacts.add_contact_subtitle\")}"
                    }
                }

                // Main content area
                div {
                    class: "cm-contacts-main",

                    // Contacts list
                    div {
                        class: "cm-contacts-list",
                        
                        // Contacts list header
                        div {
                            class: "cm-contacts-list-header",
                            
                            h2 {
                                class: "cm-contacts-list-title",
                                "{props.i18n.translate(\"contacts.contacts_list\")}"
                            }
                        }

                        // Status filter tabs
                        div {
                            class: "cm-status-filters",
                            StatusFilterTab { 
                                status: "all".to_string(),
                                label: props.i18n.translate("contacts.all"),
                                is_active: true,
                                i18n: props.i18n.clone()
                            }
                            StatusFilterTab { 
                                status: "online".to_string(),
                                label: props.i18n.translate("status.online"),
                                is_active: false,
                                i18n: props.i18n.clone()
                            }
                            StatusFilterTab { 
                                status: "offline".to_string(),
                                label: props.i18n.translate("status.offline"),
                                is_active: false,
                                i18n: props.i18n.clone()
                            }
                        }

                        // Contacts list (not grid)
                        div {
                            class: "cm-contacts-list-container",
                            for contact in filtered_contacts() {
                                ContactListItem {
                                    key: "{contact.id}",
                                    contact: contact.clone(),
                                    is_selected: selected_contact().as_ref() == Some(&contact.id),
                                    onclick: move |contact_id| selected_contact.set(Some(contact_id)),
                                    i18n: props.i18n.clone()
                                }
                            }
                        }

                        // Pagination
                        div {
                            class: "cm-contacts-pagination",
                            ContactsPagination {
                                current_page: 1,
                                total_pages: 3,
                                i18n: props.i18n.clone()
                            }
                        }
                    }

                    // Contact details panel
                    div {
                        class: "cm-contact-details",
                        if let Some(contact_id) = selected_contact() {
                            if let Some(contact) = contacts().iter().find(|c| c.id == contact_id) {
                                ContactDetailsPanel {
                                    contact: contact.clone(),
                                    i18n: props.i18n.clone()
                                }
                            } else {
                                ContactPlaceholder { i18n: props.i18n.clone() }
                            }
                        } else {
                            ContactPlaceholder { i18n: props.i18n.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StatusFilterTabProps {
    status: String,
    label: String,
    is_active: bool,
    i18n: I18nContext,
}

#[component]
fn StatusFilterTab(props: StatusFilterTabProps) -> Element {
    let class = if props.is_active {
        "cm-status-tab active"
    } else {
        "cm-status-tab"
    };

    rsx! {
        button {
            class: class,
            "{props.label}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactListItemProps {
    contact: Contact,
    is_selected: bool,
    onclick: EventHandler<String>,
    i18n: I18nContext,
}

#[component]
fn ContactListItem(props: ContactListItemProps) -> Element {
    let item_class = if props.is_selected {
        "cm-contact-list-item selected"
    } else {
        "cm-contact-list-item"
    };

    rsx! {
        div {
            class: item_class,
            onclick: move |_| props.onclick.call(props.contact.id.clone()),

            // Avatar section
            div {
                class: "cm-contact-avatar-section",
                div {
                    class: "cm-contact-avatar-container",
                    div {
                        class: "cm-contact-avatar",
                        "{props.contact.avatar_initial}"
                    }
                    div {
                        class: "cm-status-indicator status-{props.contact.status.as_str()}"
                    }
                }
            }

            // Main info section
            div {
                class: "cm-contact-main-info",
                div {
                    class: "cm-contact-name-section",
                    h3 {
                        class: "cm-contact-name",
                        "{props.contact.name}"
                    }
                    p {
                        class: "cm-contact-username",
                        "@{props.contact.username}"
                    }
                }
                
                div {
                    class: "cm-contact-status-section",
                    p {
                        class: "cm-contact-status-text",
                        match props.contact.status {
                            ContactStatus::Online => props.i18n.translate("status.online"),
                            ContactStatus::Away => props.i18n.translate("status.away"),
                            ContactStatus::Offline => if let Some(last_seen) = &props.contact.last_seen {
                                format!("{} {}", props.i18n.translate("contacts.last_seen"), last_seen)
                            } else {
                                props.i18n.translate("status.offline")
                            },
                            ContactStatus::DoNotDisturb => props.i18n.translate("status.busy"),
                        }
                    }
                }
            }

            // Action buttons section
            div {
                class: "cm-contact-actions-section",
                button {
                    class: "cm-contact-action-btn message-btn",
                    Icon {
                        name: IconName::Users,
                        i18n: props.i18n.clone(),
                        class: "cm-action-btn-icon".to_string()
                    }
                }
                button {
                    class: "cm-contact-action-btn more-btn",
                    Icon {
                        name: IconName::DotsVertical,
                        i18n: props.i18n.clone(),
                        class: "cm-action-btn-icon".to_string()
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactDetailsPanelProps {
    contact: Contact,
    i18n: I18nContext,
}

#[component]
fn ContactDetailsPanel(props: ContactDetailsPanelProps) -> Element {
    rsx! {
        div {
            class: "cm-contact-details-panel",

            // Contact header
            div {
                class: "cm-contact-details-header",
                div {
                    class: "cm-contact-details-avatar-container",
                    div {
                        class: "cm-contact-details-avatar",
                        "{props.contact.avatar_initial}"
                    }
                    div {
                        class: "cm-status-indicator-large status-{props.contact.status.as_str()}"
                    }
                }

                div {
                    class: "cm-contact-details-info",
                    h2 {
                        class: "cm-contact-details-name",
                        "{props.contact.name}"
                    }
                    p {
                        class: "cm-contact-details-username",
                        "@{props.contact.username}"
                    }
                    p {
                        class: "cm-contact-details-status",
                        match props.contact.status {
                            ContactStatus::Online => props.i18n.translate("status.online"),
                            ContactStatus::Away => props.i18n.translate("status.away"),
                            ContactStatus::Offline => props.i18n.translate("status.offline"),
                            ContactStatus::DoNotDisturb => props.i18n.translate("status.busy"),
                        }
                    }
                }
            }

            // Action buttons
            div {
                class: "cm-contact-actions",
                button {
                    class: "cm-action-btn primary",
                    Icon {
                        name: IconName::Users,
                        i18n: props.i18n.clone(),
                        class: "cm-action-icon".to_string()
                    }
                    "{props.i18n.translate(\"contacts.send_message\")}"
                }

                button {
                    class: "cm-action-btn secondary",
                    Icon {
                        name: IconName::Plus,
                        i18n: props.i18n.clone(),
                        class: "cm-action-icon".to_string()
                    }
                    "{props.i18n.translate(\"contacts.add_to_room\")}"
                }

                button {
                    class: "cm-action-btn secondary",
                    Icon {
                        name: IconName::Settings,
                        i18n: props.i18n.clone(),
                        class: "cm-action-icon".to_string()
                    }
                    "{props.i18n.translate(\"contacts.verify_keys\")}"
                }
            }

            // Contact details sections
            div {
                class: "cm-contact-details-sections",
                
                ContactInfoSection {
                    title: props.i18n.translate("contacts.details.info"),
                    i18n: props.i18n.clone(),
                    InfoItem {
                        label: props.i18n.translate("contacts.details.username"),
                        value: props.contact.username.clone()
                    }
                    InfoItem {
                        label: "Status".to_string(),
                        value: match props.contact.status {
                            ContactStatus::Online => props.i18n.translate("status.online"),
                            ContactStatus::Away => props.i18n.translate("status.away"), 
                            ContactStatus::Offline => props.i18n.translate("status.offline"),
                            ContactStatus::DoNotDisturb => props.i18n.translate("status.busy"),
                        }
                    }
                }

                ContactInfoSection {
                    title: props.i18n.translate("contacts.details.security"),
                    i18n: props.i18n.clone(),
                    InfoItem {
                        label: props.i18n.translate("contacts.details.encryption"),
                        value: props.i18n.translate("security.end_to_end_encryption")
                    }
                    InfoItem {
                        label: "Key Verified".to_string(),
                        value: props.i18n.translate("contacts.details.verified")
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactPlaceholderProps {
    i18n: I18nContext,
}

#[component]
fn ContactPlaceholder(props: ContactPlaceholderProps) -> Element {
    rsx! {
        div {
            class: "cm-contact-placeholder",
            div {
                class: "cm-placeholder-icon",
                Icon {
                    name: IconName::Users,
                    i18n: props.i18n.clone(),
                    class: "cm-placeholder-icon-svg".to_string()
                }
            }
            h3 {
                class: "cm-placeholder-title",
                "{props.i18n.translate(\"contacts.placeholder.title\")}"
            }
            p {
                class: "cm-placeholder-subtitle",
                "{props.i18n.translate(\"contacts.placeholder.subtitle\")}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactInfoSectionProps {
    title: String,
    i18n: I18nContext,
    children: Element,
}

#[component]
fn ContactInfoSection(props: ContactInfoSectionProps) -> Element {
    rsx! {
        div {
            class: "cm-info-section",
            h4 {
                class: "cm-info-section-title",
                "{props.title}"
            }
            div {
                class: "cm-info-items",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct InfoItemProps {
    label: String,
    value: String,
}

#[component]
fn InfoItem(props: InfoItemProps) -> Element {
    rsx! {
        div {
            class: "cm-info-item",
            span {
                class: "cm-info-label",
                "{props.label}:"
            }
            span {
                class: "cm-info-value",
                "{props.value}"
            }
        }
    }
}

// Mock data for development
fn get_mock_contacts() -> Vec<Contact> {
    vec![
        Contact {
            id: "1".to_string(),
            name: "Alice Johnson".to_string(),
            username: "alice_j".to_string(),
            status: ContactStatus::Online,
            avatar_initial: "A".to_string(),
            last_seen: None,
        },
        Contact {
            id: "2".to_string(),
            name: "Bob Smith".to_string(),
            username: "bob_smith".to_string(),
            status: ContactStatus::Away,
            avatar_initial: "B".to_string(),
            last_seen: Some("2 hours ago".to_string()),
        },
        Contact {
            id: "3".to_string(),
            name: "Carol Davis".to_string(),
            username: "carol_d".to_string(),
            status: ContactStatus::Offline,
            avatar_initial: "C".to_string(),
            last_seen: Some("Yesterday".to_string()),
        },
        Contact {
            id: "4".to_string(),
            name: "David Wilson".to_string(),
            username: "d_wilson".to_string(),
            status: ContactStatus::DoNotDisturb,
            avatar_initial: "D".to_string(),
            last_seen: None,
        },
        Contact {
            id: "5".to_string(),
            name: "Eve Brown".to_string(),
            username: "eve_b".to_string(),
            status: ContactStatus::Online,
            avatar_initial: "E".to_string(),
            last_seen: None,
        },
    ]
}

#[derive(Props, Clone, PartialEq)]
struct ContactsPaginationProps {
    current_page: usize,
    total_pages: usize,
    i18n: I18nContext,
}

#[component]
fn ContactsPagination(props: ContactsPaginationProps) -> Element {
    rsx! {
        div {
            class: "cm-pagination-container",
            
            div {
                class: "cm-pagination-info",
                span {
                    class: "cm-pagination-text",
                    "{props.i18n.translate(\"contacts.showing\")} {props.current_page * 10 - 9}-{(props.current_page * 10).min(50)} {props.i18n.translate(\"contacts.of\")} 50 {props.i18n.translate(\"nav.contacts\")}"
                }
            }

            div {
                class: "cm-pagination-controls",
                button {
                    class: "cm-pagination-btn",
                    disabled: props.current_page == 1,
                    Icon {
                        name: IconName::ArrowRight,
                        i18n: props.i18n.clone(),
                        class: "cm-pagination-icon prev".to_string()
                    }
                }
                
                for page in 1..=props.total_pages {
                    button {
                        key: "{page}",
                        class: if page == props.current_page { "cm-pagination-btn active" } else { "cm-pagination-btn" },
                        "{page}"
                    }
                }

                button {
                    class: "cm-pagination-btn",
                    disabled: props.current_page == props.total_pages,
                    Icon {
                        name: IconName::ArrowRight,
                        i18n: props.i18n.clone(),
                        class: "cm-pagination-icon next".to_string()
                    }
                }
            }
        }
    }
}