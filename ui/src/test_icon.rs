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
    use crate::icon::*;
    use crate::I18nContext;

    #[test]
    fn test_icon_name_variants_exist() {
        // Test that all icon name variants exist and can be created
        let icons = vec![
            IconName::Dashboard,
            IconName::Users,
            IconName::Settings,
            IconName::Search,
            IconName::Plus,
            IconName::Bell,
            IconName::ArrowRight,
            IconName::DotsVertical,
        ];

        // Test that all icons can be created and have translation keys
        for icon in icons {
            // This should not panic and should return a valid translation key
            let key = icon.default_translation_key();
            assert!(key.starts_with("icons."));
            assert!(!key.is_empty());
        }
    }

    #[test]
    fn test_icon_name_debug_format() {
        // Test that IconName implements Debug correctly
        assert_eq!(format!("{:?}", IconName::Dashboard), "Dashboard");
        assert_eq!(format!("{:?}", IconName::Users), "Users");
        assert_eq!(format!("{:?}", IconName::Settings), "Settings");
    }

    #[test]
    fn test_icon_name_clone_and_partialeq() {
        let icon1 = IconName::Dashboard;
        let icon2 = icon1.clone();
        let icon3 = IconName::Users;

        assert_eq!(icon1, icon2);
        assert_ne!(icon1, icon3);
    }

    #[test]
    fn test_i18n_translations() {
        // Test English translations
        let i18n_en = I18nContext::new("en");
        assert_eq!(i18n_en.translate("icons.dashboard"), "Dashboard");
        assert_eq!(i18n_en.translate("icons.users"), "Users");
        assert_eq!(i18n_en.translate("icons.settings"), "Settings");

        // Test one other language to prove i18n works
        let i18n_es = I18nContext::new("es");
        assert_eq!(i18n_es.translate("icons.dashboard"), "Panel");
        assert_eq!(i18n_es.translate("icons.users"), "Usuarios");

        // Test that default translation keys work with I18nContext
        assert_eq!(
            i18n_en.translate(IconName::Dashboard.default_translation_key()),
            "Dashboard"
        );
        assert_eq!(
            i18n_es.translate(IconName::Dashboard.default_translation_key()),
            "Panel"
        );
    }

    #[test]
    fn test_missing_translation_fallback() {
        let i18n = I18nContext::new("en");

        // Test that missing translations fall back to the key itself
        assert_eq!(i18n.translate("icons.nonexistent"), "icons.nonexistent");
    }

    #[test]
    fn test_icon_component_renders_correctly() {
        use crate::test_utils::test_helpers::*;
        use dioxus::prelude::*;

        let i18n = I18nContext::new("en");

        // Test basic Icon component rendering with default translation
        let icon_element = rsx! {
            Icon {
                name: IconName::Dashboard,
                i18n: i18n.clone(),
                size: "24".to_string(),
                class: "test-icon".to_string(),
                alt_key: None,
                alt_text: None,
            }
        };

        // Test with custom alt text
        let icon_with_custom_alt = rsx! {
            Icon {
                name: IconName::Settings,
                i18n: i18n.clone(),
                size: "16".to_string(),
                class: "".to_string(),
                alt_key: None,
                alt_text: Some("Custom Settings Icon".to_string()),
            }
        };

        // Test rendering doesn't panic and produces valid HTML
        let rendered_default = render_to_string(icon_element);
        let rendered_custom = render_to_string(icon_with_custom_alt);

        // Verify the rendered output contains expected attributes
        assert!(rendered_default.contains("width=\"24\""));
        assert!(rendered_default.contains("height=\"24\""));
        assert!(rendered_default.contains("class=\"test-icon\""));
        assert!(rendered_default.contains("alt=\"Dashboard\""));

        assert!(rendered_custom.contains("width=\"16\""));
        assert!(rendered_custom.contains("height=\"16\""));
        assert!(rendered_custom.contains("alt=\"Custom Settings Icon\""));

        // Ensure different configurations produce different output
        assert_ne!(rendered_default, rendered_custom);
    }

    #[test]
    fn test_icon_props_creation_and_equality() {
        let i18n = I18nContext::new("en");

        // Test creating props with different configurations
        let props1 = IconProps {
            name: IconName::Dashboard,
            i18n: i18n.clone(),
            size: "16".to_string(),
            class: "test".to_string(),
            alt_key: None,
            alt_text: None,
        };

        let props2 = props1.clone();
        let props3 = IconProps {
            name: IconName::Users,
            i18n: i18n.clone(),
            size: "24".to_string(),
            class: "custom-icon".to_string(),
            alt_key: Some("custom.alt".to_string()),
            alt_text: Some("Custom Alt Text".to_string()),
        };

        // Test equality and cloning
        assert!(props1 == props2);
        assert!(props1 != props3);

        // Test specific values
        assert_eq!(props3.name, IconName::Users);
        assert_eq!(props3.size, "24");
        assert_eq!(props3.class, "custom-icon");
        assert_eq!(props3.alt_text, Some("Custom Alt Text".to_string()));
    }
}
