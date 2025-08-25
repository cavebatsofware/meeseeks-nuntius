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

//! This crate contains all shared UI for the workspace.

mod i18n_context;
pub use i18n_context::*;
mod test_i18n_context;

pub mod types;
pub use types::*;

mod icon;
pub use icon::{Icon, IconName};

mod css_utils;
pub use css_utils::{css_var_to_color, resolve_color};

// non-web modules
#[cfg(all(not(target_arch = "wasm32"), any(feature = "desktop", feature = "mobile")))]
mod user_profile_mini;
#[cfg(all(not(target_arch = "wasm32"), any(feature = "desktop", feature = "mobile")))]
pub use user_profile_mini::UserProfileMini;

#[cfg(all(not(target_arch = "wasm32"), any(feature = "desktop", feature = "mobile")))]
mod user_profile_edit;
#[cfg(all(not(target_arch = "wasm32"), any(feature = "desktop", feature = "mobile")))]
pub use user_profile_edit::*;

mod test_icon;
mod test_utils;