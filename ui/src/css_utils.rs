/// Utility functions for working with CSS variables and colors
use std::collections::HashMap;
use std::sync::OnceLock;

/// Parse CSS variables from variables.css at compile time
fn parse_css_variables() -> HashMap<String, String> {
    let css_content = include_str!("../../mobile/assets/variables.css");
    let mut variables = HashMap::new();

    for line in css_content.lines() {
        let line = line.trim();
        if line.starts_with("--") && line.contains(':') {
            if let Some(colon_pos) = line.find(':') {
                let var_name = line[..colon_pos].trim();
                let var_value = line[colon_pos + 1..].trim().trim_end_matches(';').trim();
                variables.insert(var_name.to_string(), var_value.to_string());
            }
        }
    }

    variables
}

/// Get the static map of CSS variables
fn get_css_variables() -> &'static HashMap<String, String> {
    static CSS_VARIABLES: OnceLock<HashMap<String, String>> = OnceLock::new();
    CSS_VARIABLES.get_or_init(parse_css_variables)
}

/// Converts a CSS variable name to its actual color value
/// Returns the actual hex color value for known CSS variables
pub fn css_var_to_color(var_name: &str) -> Option<String> {
    get_css_variables().get(var_name).cloned()
}

/// Converts a CSS variable reference (like "var(--color-text-primary)") to its actual color value
/// Also handles direct color values (like "#ffffff" or "red")
pub fn resolve_color(color_value: &str) -> String {
    if color_value.starts_with("var(") && color_value.ends_with(")") {
        // Extract the variable name from var(--variable-name)
        let var_name = &color_value[4..color_value.len() - 1];
        css_var_to_color(var_name).unwrap_or_else(|| color_value.to_string())
    } else {
        // Direct color value
        color_value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_var_to_color() {
        // Test that we can parse actual values from variables.css
        assert!(css_var_to_color("--color-text-primary").is_some());
        assert!(css_var_to_color("--color-accent-primary").is_some());
        assert_eq!(css_var_to_color("--unknown-var"), None);
    }

    #[test]
    fn test_resolve_color() {
        // Test that CSS variables get resolved to actual values
        let resolved = resolve_color("var(--color-text-primary)");
        assert!(resolved.starts_with('#') || !resolved.starts_with("var("));

        // Test direct color values pass through unchanged
        assert_eq!(resolve_color("#ff0000"), "#ff0000");
        assert_eq!(resolve_color("red"), "red");
    }
}
