/*  This file is part of a secure messaging project codename meeseeks-nuntius
 *  Copyright (C) 2025 Grant DeFayette
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

use std::collections::HashMap;

#[derive(Clone)]
pub struct I18nContext {
    pub current_locale: String,
    pub available_locales: Vec<String>,
    translations: HashMap<String, HashMap<String, String>>,
}

impl PartialEq for I18nContext {
    fn eq(&self, other: &Self) -> bool {
        self.current_locale == other.current_locale
            && self.available_locales == other.available_locales
    }
}

// Include translation files at compile time
const EN_TRANSLATIONS: &str = include_str!("../../locales/en.yml");
const ES_TRANSLATIONS: &str = include_str!("../../locales/es.yml");
const FR_TRANSLATIONS: &str = include_str!("../../locales/fr.yml");
const DE_TRANSLATIONS: &str = include_str!("../../locales/de.yml");
const ZH_TRANSLATIONS: &str = include_str!("../../locales/zh.yml");
const ZH_TW_TRANSLATIONS: &str = include_str!("../../locales/zh-TW.yml");
const AR_TRANSLATIONS: &str = include_str!("../../locales/ar.yml");
const JA_TRANSLATIONS: &str = include_str!("../../locales/ja.yml");

impl I18nContext {
    pub fn new(initial_locale: &str) -> Self {
        let mut translations = HashMap::new();

        // Load all translations from embedded YAML
        let locales = [
            ("en", EN_TRANSLATIONS),
            ("es", ES_TRANSLATIONS),
            ("fr", FR_TRANSLATIONS),
            ("de", DE_TRANSLATIONS),
            ("zh", ZH_TRANSLATIONS),
            ("zh-TW", ZH_TW_TRANSLATIONS),
            ("ar", AR_TRANSLATIONS),
            ("ja", JA_TRANSLATIONS),
        ];

        for (locale_code, yaml_content) in locales {
            match load_translations_from_yaml(yaml_content) {
                Ok(locale_translations) => {
                    translations.insert(locale_code.to_string(), locale_translations);
                }
                Err(e) => {
                    eprintln!("Failed to load translations for {locale_code}: {e}");
                    // Continue with other locales
                }
            }
        }

        Self {
            current_locale: initial_locale.to_string(),
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
            translations,
        }
    }

    pub fn get_current_locale(&self) -> &str {
        &self.current_locale
    }

    pub fn change_locale(&mut self, locale: &str) {
        if self.available_locales.contains(&locale.to_string()) {
            self.current_locale = locale.to_string();
        }
    }

    pub fn translate(&self, key: &str) -> String {
        let current_locale = &self.current_locale;

        // Try current locale first
        if let Some(locale_map) = self.translations.get(current_locale) {
            if let Some(translation) = get_nested_value(locale_map, key) {
                return translation;
            }
        }

        // Fallback to English
        if let Some(en_map) = self.translations.get("en") {
            if let Some(translation) = get_nested_value(en_map, key) {
                return translation;
            }
        }

        // Return the key itself if no translation found
        key.to_string()
    }
}

// Parse YAML and flatten nested keys (e.g., "nav.messages" from nested structure)
pub fn load_translations_from_yaml(
    yaml_content: &str,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let yaml_value: serde_yml::Value = serde_yml::from_str(yaml_content)?;
    let mut flat_map = HashMap::new();

    if let serde_yml::Value::Mapping(map) = yaml_value {
        flatten_yaml_map(&map, "", &mut flat_map);
    }

    Ok(flat_map)
}

// Recursively flatten nested YAML structure
fn flatten_yaml_map(map: &serde_yml::Mapping, prefix: &str, result: &mut HashMap<String, String>) {
    for (key, value) in map {
        if let serde_yml::Value::String(key_str) = key {
            let full_key = if prefix.is_empty() {
                key_str.clone()
            } else {
                format!("{prefix}.{key_str}")
            };

            match value {
                serde_yml::Value::String(val) => {
                    result.insert(full_key, val.clone());
                }
                serde_yml::Value::Mapping(nested_map) => {
                    flatten_yaml_map(nested_map, &full_key, result);
                }
                serde_yml::Value::Number(n) => {
                    result.insert(full_key, n.to_string());
                }
                serde_yml::Value::Bool(b) => {
                    result.insert(full_key, b.to_string());
                }
                _ => {
                    // Skip other types (null, sequence, etc.)
                }
            }
        }
    }
}

// Get value from flattened map (supports dot notation)
fn get_nested_value(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).cloned()
}

// Helper function to get human-readable language names
pub fn get_language_name(locale: &str) -> &'static str {
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

// Helper function to get text direction for RTL languages
pub fn get_text_direction(locale: &str) -> &'static str {
    match locale {
        "ar" => "rtl",
        _ => "ltr",
    }
}
