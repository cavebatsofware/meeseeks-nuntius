// src/i18n/mod.rs - I18n module for meeseeks-nuntius (Dioxus 0.6.x)

use dioxus::prelude::*;
use rust_i18n::{i18n, t};
use std::rc::Rc;

// Initialize i18n with locales directory
i18n!("locales", fallback = "en");

#[derive(Clone)]
pub struct I18nContext {
    pub current_locale: Signal<String>,
    pub available_locales: Vec<String>,
}

impl PartialEq for I18nContext {
    fn eq(&self, other: &Self) -> bool {
        self.current_locale.read().as_str() == other.current_locale.read().as_str()
            && self.available_locales == other.available_locales
    }
}

impl I18nContext {
    pub fn new(initial_locale: &str) -> Self {
        rust_i18n::set_locale(initial_locale);
        
        Self {
            current_locale: Signal::new(initial_locale.to_string()),
            available_locales: vec![
                "en".to_string(),
                "es".to_string(),
                "fr".to_string(),
                "de".to_string(),
                "zh".to_string(),
                "zh-TW".to_string(),
                "ar".to_string(),
                "ja".to_string(),
            ],
        }
    }
    
    pub fn get_current_locale(&self) -> String {
        self.current_locale.read().clone()
    }
    
    pub fn change_locale(&mut self, locale: &str) {
        if self.available_locales.contains(&locale.to_string()) {
            rust_i18n::set_locale(locale);
            *self.current_locale.write() = locale.to_string();
        }
    }
}

// Hook for easy access to translations in components
pub fn use_i18n() -> Signal<I18nContext> {
    use_context::<Signal<I18nContext>>()
}

// Provider component to wrap your app
#[component]
pub fn I18nProvider(children: Element) -> Element {
    let i18n_context = use_signal(|| I18nContext::new(&get_initial_locale()));
    
    use_context_provider(|| i18n_context);

    rsx! { {children} }
}

// Get initial locale from browser/system preferences
fn get_initial_locale() -> String {
    // This would need platform-specific implementation
    // For now, default to English
    "en".to_string()
}

// Save locale preference (platform-specific)
fn save_locale_preference(_locale: &str) {
    // Implementation would depend on platform:
    // - Web: localStorage
    // - Desktop: config file
    // - Mobile: platform preferences
}

// Convenience macro for translations
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        t!($key)
    };
    ($key:expr, $($args:tt)*) => {
        rust_i18n::t!($key, $($args)*)
    };
}

// Language switcher component for settings
#[component]
pub fn LanguageSwitcher() -> Element {
    let mut i18n = use_i18n();
    
    rsx! {
        div { class: "language-switcher",
            label { 
                r#for: "language-select",
                class: "language-label",
                {tr!("settings.language")}
            }
            select {
                id: "language-select",
                class: "language-select",
                value: "{i18n.read().get_current_locale()}",
                onchange: move |evt: FormEvent| {
                    i18n.write().change_locale(&evt.value());
                },
                {i18n.read().available_locales.iter().map(|locale| rsx! {
                    option {
                        value: "{locale}",
                        selected: i18n.read().get_current_locale() == *locale,
                        {get_language_name(locale)}
                    }
                })}
            }
        }
    }
}

// Helper function to get human-readable language names
fn get_language_name(locale: &str) -> &'static str {
    match locale {
        "en" => "English",
        "es" => "Español", 
        "fr" => "Français",
        "de" => "Deutsch",
        "zh" => "中文 (简体)",
        "zh-TW" => "中文 (繁體)",
        "ar" => "العربية",
        "ja" => "日本語",
        _ => "Unknown",
    }
}

// Utility component for displaying translated text with fallback
#[derive(Props, Clone, PartialEq)]
pub struct TranslatedTextProps {
    key: String,
    #[props(optional)]
    fallback: Option<String>,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    interpolations: Option<std::collections::HashMap<String, String>>,
}

#[component]
pub fn TranslatedText(props: TranslatedTextProps) -> Element {
    let _i18n = use_i18n(); // Ensure we re-render when locale changes
    
    let text = t!(&props.key);
    let display_text = if text == props.key && props.fallback.is_some() {
        // Translation key not found, use fallback
        props.fallback.as_ref().unwrap()
    } else {
        &text
    };
    
    // Apply interpolations if provided
    let final_text = if let Some(interpolations) = &props.interpolations {
        let mut result = display_text.to_string();
        for (key, value) in interpolations {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    } else {
        display_text.to_string()
    };
    
    rsx! {
        span { 
            class: props.class.as_deref().unwrap_or(""),
            {final_text}
        }
    }
}

// Specialized component for error messages with automatic error prefix
#[derive(Props, Clone, PartialEq)]
pub struct ErrorTextProps {
    error_key: String,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    fallback: Option<String>,
}

#[component]
pub fn ErrorText(props: ErrorTextProps) -> Element {
    let _i18n = use_i18n();
    
    let error_translation_key = format!("errors.{}", props.error_key);
    let text = t!(&error_translation_key);
    
    let display_text = if text == error_translation_key {
        // Specific error not found, try generic or fallback
        props.fallback.as_deref().unwrap_or(&t!("errors.generic"))
    } else {
        &text
    };
    
    rsx! {
        span { 
            class: format!("error-text {}", props.class.as_deref().unwrap_or("")),
            {display_text}
        }
    }
}

// Component for displaying status with color indicators
#[derive(Props, Clone, PartialEq)]
pub struct StatusTextProps {
    status_key: String,
    #[props(optional)]
    show_indicator: Option<bool>,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn StatusText(props: StatusTextProps) -> Element {
    let _i18n = use_i18n();
    
    let status_translation_key = format!("status.{}", props.status_key);
    let text = t!(&status_translation_key);
    
    let status_class = format!("status-{}", props.status_key);
    let show_indicator = props.show_indicator.unwrap_or(false);
    
    rsx! {
        span { 
            class: format!("status-text {} {}", status_class, props.class.as_deref().unwrap_or("")),
            if show_indicator {
                span { class: "status-indicator", "●" }
            }
            {text}
        }
    }
}

// Hook for translating error messages
pub fn use_error_translation() -> impl Fn(&str) -> String {
    let _i18n = use_i18n();
    
    move |error_key: &str| {
        let key = format!("errors.{}", error_key);
        let translated = t!(&key);
        if translated == key {
            // Fallback to generic error message
            t!("errors.generic")
        } else {
            translated
        }
    }
}

// Helper function to get text direction for RTL languages
pub fn get_text_direction(locale: &str) -> &'static str {
    match locale {
        "ar" => "rtl",
        _ => "ltr",
    }
}
