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
pub mod test_helpers {
    use dioxus::prelude::*;

    /// Helper function to compare RSX rendering output
    /// Based on official Dioxus testing documentation: https://dioxuslabs.com/learn/0.6/cookbook/testing
    pub fn assert_rsx_eq(first: Element, second: Element) {
        let first = dioxus_ssr::render_element(first);
        let second = dioxus_ssr::render_element(second);
        pretty_assertions::assert_str_eq!(first, second);
    }

    /// Helper function to render an RSX element to HTML string for inspection
    pub fn render_to_string(element: Element) -> String {
        dioxus_ssr::render_element(element)
    }
}
