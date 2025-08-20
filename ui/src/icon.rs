use crate::I18nContext;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IconName {
    Dashboard,
    Users,
    Settings,
    Search,
    Plus,
    Bell,
    ArrowRight,
    ArrowLeft,
    DotsVertical,
    MoreVertical,
    MessageSquare,
    Home,
    User,
}

impl IconName {
    fn asset_path(&self) -> Asset {
        match self {
            IconName::Dashboard => asset!("/assets/icons/dashboard.svg"),
            IconName::Users => asset!("/assets/icons/users.svg"),
            IconName::Settings => asset!("/assets/icons/settings.svg"),
            IconName::Search => asset!("/assets/icons/search.svg"),
            IconName::Plus => asset!("/assets/icons/plus.svg"),
            IconName::Bell => asset!("/assets/icons/bell.svg"),
            IconName::ArrowRight => asset!("/assets/icons/arrow-right.svg"),
            IconName::ArrowLeft => asset!("/assets/icons/arrow-left.svg"),
            IconName::DotsVertical => asset!("/assets/icons/dots-vertical.svg"),
            IconName::MoreVertical => asset!("/assets/icons/more-vertical.svg"),
            IconName::MessageSquare => asset!("/assets/icons/message-square.svg"),
            IconName::Home => asset!("/assets/icons/home.svg"),
            IconName::User => asset!("/assets/icons/user.svg"),
        }
    }

    pub fn default_translation_key(&self) -> &'static str {
        match self {
            IconName::Dashboard => "icons.dashboard",
            IconName::Users => "icons.users",
            IconName::Settings => "icons.settings",
            IconName::Search => "icons.search",
            IconName::Plus => "icons.plus",
            IconName::Bell => "icons.bell",
            IconName::ArrowRight => "icons.arrow_right",
            IconName::ArrowLeft => "icons.arrow_left",
            IconName::DotsVertical => "icons.dots_vertical",
            IconName::MoreVertical => "icons.more_vertical",
            IconName::MessageSquare => "icons.message_square",
            IconName::Home => "icons.home",
            IconName::User => "icons.user",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub name: IconName,
    #[props(default = I18nContext::new("en"))]
    pub i18n: I18nContext,
    #[props(default = "16".to_string())]
    pub size: String,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = None)]
    pub alt_key: Option<String>, // Custom translation key
    #[props(default = None)]
    pub alt_text: Option<String>, // Direct alt text (overrides translation)
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let alt_text = if let Some(direct_alt) = props.alt_text {
        // Use direct alt text if provided
        direct_alt
    } else if let Some(custom_key) = props.alt_key {
        // Use custom translation key if provided
        props.i18n.translate(&custom_key)
    } else {
        // Use default translation key for the icon
        props.i18n.translate(props.name.default_translation_key())
    };

    rsx! {
        img {
            src: props.name.asset_path(),
            width: "{props.size}",
            height: "{props.size}",
            alt: "{alt_text}",
            class: "{props.class}",
        }
    }
}
