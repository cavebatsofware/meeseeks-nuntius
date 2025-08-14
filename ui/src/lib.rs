//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod i18n_context;
pub use i18n_context::I18nContext;

mod test_i18n_context;

mod user_profile_mini;
pub use user_profile_mini::UserProfileMini;
