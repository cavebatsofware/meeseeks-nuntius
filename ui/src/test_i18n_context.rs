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

#[cfg(test)]
mod tests {
    use crate::i18n_context::*;

    #[test]
    fn test_load_translations_from_yaml() {
        let yaml_content = r#"
app:
  name: "Test App"
  tagline: "Test Tagline"
nav:
  messages: "Messages"
  settings: "Settings"
auth:
  login: "Login"
  logout: "Logout"
"#;
        let result = load_translations_from_yaml(yaml_content).unwrap();

        assert_eq!(result.get("app.name"), Some(&"Test App".to_string()));
        assert_eq!(result.get("app.tagline"), Some(&"Test Tagline".to_string()));
        assert_eq!(result.get("nav.messages"), Some(&"Messages".to_string()));
        assert_eq!(result.get("nav.settings"), Some(&"Settings".to_string()));
        assert_eq!(result.get("auth.login"), Some(&"Login".to_string()));
        assert_eq!(result.get("auth.logout"), Some(&"Logout".to_string()));
    }

    #[test]
    fn test_flatten_yaml_map_with_nested_structure() {
        let yaml_content = r#"
level1:
  level2:
    level3: "deeply nested"
  simple: "not nested"
top_level: "root level"
"#;
        let result = load_translations_from_yaml(yaml_content).unwrap();

        assert_eq!(
            result.get("level1.level2.level3"),
            Some(&"deeply nested".to_string())
        );
        assert_eq!(result.get("level1.simple"), Some(&"not nested".to_string()));
        assert_eq!(result.get("top_level"), Some(&"root level".to_string()));
    }

    #[test]
    fn test_yaml_with_different_value_types() {
        let yaml_content = r#"
strings:
  text: "Hello World"
numbers:
  count: 42
  price: 19.99
booleans:
  enabled: true
  disabled: false
"#;
        let result = load_translations_from_yaml(yaml_content).unwrap();

        assert_eq!(result.get("strings.text"), Some(&"Hello World".to_string()));
        assert_eq!(result.get("numbers.count"), Some(&"42".to_string()));
        assert_eq!(result.get("numbers.price"), Some(&"19.99".to_string()));
        assert_eq!(result.get("booleans.enabled"), Some(&"true".to_string()));
        assert_eq!(result.get("booleans.disabled"), Some(&"false".to_string()));
    }

    #[test]
    fn test_i18n_context_creation() {
        let context = I18nContext::new("en");

        assert_eq!(context.current_locale, "en");
        assert!(context.available_locales.contains(&"en".to_string()));
        assert!(context.available_locales.len() > 1);
    }

    #[test]
    fn test_translation_lookup() {
        let context = I18nContext::new("en");

        // Test existing translation
        let app_name = context.translate("app.name");
        assert_ne!(app_name, "app.name"); // Should not return the key itself

        // Test non-existing translation (should return key)
        let non_existing = context.translate("non.existing.key");
        assert_eq!(non_existing, "non.existing.key");
    }

    #[test]
    fn test_change_locale() {
        let mut context = I18nContext::new("en");
        assert_eq!(context.current_locale, "en");

        // Change to Spanish
        context.change_locale("es");
        assert_eq!(context.current_locale, "es");

        // Try to change to invalid locale (should remain unchanged)
        context.change_locale("invalid");
        assert_eq!(context.current_locale, "es");
    }

    #[test]
    fn test_locale_specific_translations() {
        let mut context = I18nContext::new("en");

        // Test English translation
        let en_name = context.translate("app.name");

        // Test Spanish translation (if available)
        context.change_locale("es");
        let es_name = context.translate("app.name");

        // Should be different if Spanish translation exists
        // If not, both will be the same (fallback to English)
        assert!(!en_name.is_empty());
        assert!(!es_name.is_empty());
    }

    #[test]
    fn test_invalid_yaml_handling() {
        let invalid_yaml = "invalid: yaml: content: [unclosed";
        let result = load_translations_from_yaml(invalid_yaml);

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_yaml_handling() {
        let empty_yaml = "";
        let result = load_translations_from_yaml(empty_yaml).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_get_current_locale() {
        let context = I18nContext::new("fr");
        assert_eq!(context.get_current_locale(), "fr");

        let mut context2 = I18nContext::new("en");
        context2.change_locale("de");
        assert_eq!(context2.get_current_locale(), "de");
    }

    #[test]
    fn test_get_language_name() {
        assert_eq!(get_language_name("en"), "English");
        assert_eq!(get_language_name("es"), "Español");
        assert_eq!(get_language_name("fr"), "Français");
        assert_eq!(get_language_name("invalid"), "Unknown");
    }

    #[test]
    fn test_get_text_direction() {
        assert_eq!(get_text_direction("ar"), "rtl");
        assert_eq!(get_text_direction("en"), "ltr");
        assert_eq!(get_text_direction("es"), "ltr");
    }
}
