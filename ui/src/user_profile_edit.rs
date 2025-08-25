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

use crate::I18nContext;
use dioxus::prelude::*;
use shared::user_data::UserData;

const USER_PROFILE_EDIT_CSS: Asset = asset!("/assets/styling/user_profile_edit.css");

#[derive(Props, Clone, PartialEq)]
pub struct UserProfileEditProps {
    pub initial_data: UserData,
    pub i18n: I18nContext,
    pub on_save: EventHandler<UserData>,
    pub on_cancel: EventHandler<()>,
    #[props(default = false)]
    pub is_saving: bool,
}

#[component]
pub fn UserProfileEdit(props: UserProfileEditProps) -> Element {
    let mut profile_data = use_signal(|| props.initial_data.clone());
    let mut show_avatar_upload = use_signal(|| false);
    #[allow(clippy::redundant_closure)] // use_signal requires closures, not function pointers
    let mut validation_errors = use_signal(|| Vec::<String>::new());

    // Available theme options
    let theme_options = [
        ("dark", "user_profile.themes.dark"),
        ("light", "user_profile.themes.light"),
        ("auto", "user_profile.themes.auto"),
    ];

    // Available language options - always display in native text
    let language_options = [
        ("en", "English"),
        ("es", "Español"),
        ("fr", "Français"),
        ("de", "Deutsch"),
        ("ja", "日本語"),
        ("zh", "中文"),
    ];

    // Auto-away minute options
    let auto_away_options = [
        (5, "5 minutes"),
        (10, "10 minutes"),
        (15, "15 minutes"),
        (30, "30 minutes"),
        (60, "1 hour"),
        (120, "2 hours"),
        (0, "user_profile.auto_away_options.never"),
    ];

    // Clone i18n for use in closures
    let i18n = props.i18n.clone();
    let i18n_validate = i18n.clone();

    // Validation function
    let mut validate_form = move || {
        let mut errors = Vec::new();
        let data = profile_data();

        if data.username.trim().is_empty() {
            errors.push(i18n_validate.translate("user_profile.errors.username_required"));
        }

        if data.display_name.trim().is_empty() {
            errors.push(i18n_validate.translate("user_profile.errors.display_name_required"));
        }

        validation_errors.set(errors.clone());
        errors.is_empty()
    };

    // Handle save
    let mut handle_save = move |_| {
        if validate_form() {
            props.on_save.call(profile_data());
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: USER_PROFILE_EDIT_CSS }

        div {
            class: "user-profile-edit",

            // Header
            header {
                class: "profile-header",

                h1 {
                    class: "profile-title",
                    "{i18n.translate(\"user_profile.title\")}"
                }

                p {
                    class: "profile-subtitle",
                    "{i18n.translate(\"user_profile.subtitle\")}"
                }
            }

            // Validation errors
            if !validation_errors().is_empty() {
                div {
                    class: "validation-errors",
                    for error in validation_errors() {
                        div {
                            class: "error-message",
                            "{error}"
                        }
                    }
                }
            }

            // Main form
            form {
                class: "profile-form",
                onsubmit: move |e| {
                    e.prevent_default();
                    handle_save(());
                },

                // Avatar section
                section {
                    class: "form-section avatar-section",

                    h2 {
                        class: "section-title",
                        "{i18n.translate(\"user_profile.avatar.title\")}"
                    }

                    div {
                        class: "avatar-container",

                        div {
                            class: "current-avatar",
                            onclick: move |_| show_avatar_upload.set(!show_avatar_upload()),

                            if let Some(ref avatar_url) = profile_data().avatar_url {
                                img {
                                    src: "{avatar_url}",
                                    alt: "User Avatar",
                                    class: "avatar-image"
                                }
                            } else {
                                div {
                                    class: "avatar-placeholder",
                                    "{profile_data().display_name.chars().next().unwrap_or('U').to_uppercase()}"
                                }
                            }

                            div {
                                class: "avatar-overlay",
                                "📷"
                            }
                        }

                        if show_avatar_upload() {
                            div {
                                class: "avatar-upload",

                                input {
                                    r#type: "url",
                                    placeholder: i18n.translate("user_profile.avatar.url_placeholder"),
                                    class: "avatar-input",
                                    value: profile_data().avatar_url.as_deref().unwrap_or(""),
                                    oninput: move |evt| {
                                        let mut data = profile_data();
                                        data.set_avatar_url(if evt.value().is_empty() {
                                            None
                                        } else {
                                            Some(evt.value())
                                        });
                                        profile_data.set(data);
                                    }
                                }

                                button {
                                    r#type: "button",
                                    class: "remove-avatar-btn",
                                    onclick: move |_| {
                                        let mut data = profile_data();
                                        data.set_avatar_url(None);
                                        profile_data.set(data);
                                        show_avatar_upload.set(false);
                                    },
                                    "{i18n.translate(\"user_profile.avatar.remove\")}"
                                }
                            }
                        }
                    }
                }

                // Basic information section
                section {
                    class: "form-section",

                    h2 {
                        class: "section-title",
                        "{i18n.translate(\"user_profile.basic_info.title\")}"
                    }

                    div {
                        class: "form-row",

                        div {
                            class: "form-group",
                            label {
                                class: "form-label",
                                r#for: "username",
                                "{i18n.translate(\"user_profile.username\")}"
                            }
                            input {
                                id: "username",
                                r#type: "text",
                                class: "form-input",
                                value: "{profile_data().username}",
                                placeholder: i18n.translate("user_profile.username_placeholder"),
                                oninput: move |evt| {
                                    let mut data = profile_data();
                                    data.username = evt.value();
                                    profile_data.set(data);
                                }
                            }
                        }

                        div {
                            class: "form-group",
                            label {
                                class: "form-label",
                                r#for: "display_name",
                                "{i18n.translate(\"user_profile.display_name\")}"
                            }
                            input {
                                id: "display_name",
                                r#type: "text",
                                class: "form-input",
                                value: "{profile_data().display_name}",
                                placeholder: i18n.translate("user_profile.display_name_placeholder"),
                                oninput: move |evt| {
                                    let mut data = profile_data();
                                    data.set_display_name(evt.value());
                                    profile_data.set(data);
                                }
                            }
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            r#for: "status_message",
                            "{i18n.translate(\"user_profile.status_message\")}"
                        }
                        textarea {
                            id: "status_message",
                            class: "form-textarea",
                            value: profile_data().status_message.as_deref().unwrap_or(""),
                            placeholder: i18n.translate("user_profile.status_message_placeholder"),
                            rows: "3",
                            oninput: move |evt| {
                                let mut data = profile_data();
                                data.set_status_message(if evt.value().is_empty() {
                                    None
                                } else {
                                    Some(evt.value())
                                });
                                profile_data.set(data);
                            }
                        }
                    }
                }

                // Preferences section
                section {
                    class: "form-section",

                    h2 {
                        class: "section-title",
                        "{props.i18n.translate(\"user_profile.preferences.title\")}"
                    }

                    div {
                        class: "form-row",

                        div {
                            class: "form-group",
                            label {
                                class: "form-label",
                                r#for: "theme",
                                "{props.i18n.translate(\"user_profile.theme\")}"
                            }
                            select {
                                id: "theme",
                                class: "form-select",
                                value: "{profile_data().theme}",
                                onchange: move |evt| {
                                    let mut data = profile_data();
                                    data.set_theme(evt.value());
                                    profile_data.set(data);
                                },

                                for (value, text_key) in theme_options.iter() {
                                    option {
                                        value: "{value}",
                                        selected: profile_data().theme == *value,
                                        "{props.i18n.translate(text_key)}"
                                    }
                                }
                            }
                        }

                        div {
                            class: "form-group",
                            label {
                                class: "form-label",
                                r#for: "language",
                                "{props.i18n.translate(\"user_profile.language\")}"
                            }
                            select {
                                id: "language",
                                class: "form-select",
                                value: "{profile_data().language}",
                                onchange: move |evt| {
                                    let mut data = profile_data();
                                    data.set_language(evt.value());
                                    profile_data.set(data);
                                },

                                for (value, native_name) in language_options.iter() {
                                    option {
                                        value: "{value}",
                                        selected: profile_data().language == *value,
                                        "{native_name}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            r#for: "auto_away",
                            "{props.i18n.translate(\"user_profile.auto_away\")}"
                        }
                        select {
                            id: "auto_away",
                            class: "form-select",
                            value: "{profile_data().auto_away_minutes}",
                            onchange: move |evt| {
                                let mut data = profile_data();
                                data.set_auto_away_minutes(evt.value().parse().unwrap_or(15));
                                profile_data.set(data);
                            },

                            for (value, label) in auto_away_options.iter() {
                                option {
                                    value: "{value}",
                                    selected: profile_data().auto_away_minutes == *value,
                                    if label.starts_with("user_profile.") {
                                        "{props.i18n.translate(label)}"
                                    } else {
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Notification settings section
                section {
                    class: "form-section",

                    h2 {
                        class: "section-title",
                        "{props.i18n.translate(\"user_profile.notifications.title\")}"
                    }

                    div {
                        class: "form-group checkbox-group",
                        label {
                            class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                class: "form-checkbox",
                                checked: profile_data().notifications_enabled,
                                onchange: move |evt| {
                                    let mut data = profile_data();
                                    data.set_notifications_enabled(evt.checked());
                                    profile_data.set(data);
                                }
                            }
                            span {
                                class: "checkbox-text",
                                "{props.i18n.translate(\"user_profile.enable_notifications\")}"
                            }
                        }
                    }

                    div {
                        class: "form-group checkbox-group",
                        label {
                            class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                class: "form-checkbox",
                                checked: profile_data().sound_enabled,
                                onchange: move |evt| {
                                    let mut data = profile_data();
                                    data.set_sound_enabled(evt.checked());
                                    profile_data.set(data);
                                }
                            }
                            span {
                                class: "checkbox-text",
                                "{props.i18n.translate(\"user_profile.enable_sound\")}"
                            }
                        }
                    }
                }

                // Form actions
                footer {
                    class: "form-actions",

                    button {
                        r#type: "button",
                        class: "btn btn-secondary",
                        onclick: move |_| props.on_cancel.call(()),
                        disabled: props.is_saving,
                        "{props.i18n.translate(\"actions.cancel\")}"
                    }

                    button {
                        r#type: "submit",
                        class: "btn btn-primary",
                        disabled: props.is_saving || !validation_errors().is_empty(),
                        if props.is_saving {
                            "{props.i18n.translate(\"user_profile.saving\")}"
                        } else {
                            "{props.i18n.translate(\"actions.save\")}"
                        }
                    }
                }
            }
        }
    }
}
