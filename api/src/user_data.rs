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

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::persistence::database::Entity;

// Type alias for convenience
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Represents user profile and preference data
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub username: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    pub recent_rooms: VecDeque<String>,
    pub max_recent_rooms: usize,
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub auto_away_minutes: u32,
    pub created_at: u64,
    pub last_updated: u64,
}

impl Default for UserData {
    fn default() -> Self {
        let now = current_timestamp();
        Self {
            id: None,
            username: String::new(),
            display_name: String::new(),
            avatar_url: None,
            status_message: None,
            recent_rooms: VecDeque::new(),
            max_recent_rooms: 10,
            theme: "dark".to_string(),
            language: "en".to_string(),
            notifications_enabled: true,
            sound_enabled: true,
            auto_away_minutes: 15,
            created_at: now,
            last_updated: now,
        }
    }
}

impl UserData {
    /// Create a new user data instance with required fields
    pub fn new(username: &str, display_name: &str) -> Self {
        Self {
            username: username.to_string(),
            display_name: display_name.to_string(),
            ..Default::default()
        }
    }

    /// Create a default user data instance for a given username
    pub fn default_for_user(username: &str) -> Self {
        Self::new(username, username)
    }

    /// Update the display name
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
        self.update_timestamp();
    }

    /// Update the avatar URL
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.update_timestamp();
    }

    /// Update the status message
    pub fn set_status_message(&mut self, status_message: Option<String>) {
        self.status_message = status_message;
        self.update_timestamp();
    }

    /// Add a room to recent rooms (maintaining max limit)
    pub fn add_recent_room(&mut self, room_id: String) {
        // Remove if already exists to avoid duplicates
        if let Some(pos) = self.recent_rooms.iter().position(|r| r == &room_id) {
            self.recent_rooms.remove(pos);
        }

        // Add to front
        self.recent_rooms.push_front(room_id);

        // Maintain max limit
        while self.recent_rooms.len() > self.max_recent_rooms {
            self.recent_rooms.pop_back();
        }

        self.update_timestamp();
    }

    /// Remove a room from recent rooms
    pub fn remove_recent_room(&mut self, room_id: &str) {
        if let Some(pos) = self.recent_rooms.iter().position(|r| r == room_id) {
            self.recent_rooms.remove(pos);
            self.update_timestamp();
        }
    }

    /// Clear all recent rooms
    pub fn clear_recent_rooms(&mut self) {
        self.recent_rooms.clear();
        self.update_timestamp();
    }

    /// Get recent rooms as a vector
    pub fn get_recent_rooms(&self) -> Vec<String> {
        self.recent_rooms.iter().cloned().collect()
    }

    /// Set the maximum number of recent rooms to keep
    pub fn set_max_recent_rooms(&mut self, max_recent_rooms: usize) {
        self.max_recent_rooms = max_recent_rooms;

        // Trim existing recent rooms if necessary
        while self.recent_rooms.len() > max_recent_rooms {
            self.recent_rooms.pop_back();
        }

        self.update_timestamp();
    }

    /// Update the theme
    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
        self.update_timestamp();
    }

    /// Update the language
    pub fn set_language(&mut self, language: String) {
        self.language = language;
        self.update_timestamp();
    }

    /// Enable/disable notifications
    pub fn set_notifications_enabled(&mut self, enabled: bool) {
        self.notifications_enabled = enabled;
        self.update_timestamp();
    }

    /// Enable/disable sound
    pub fn set_sound_enabled(&mut self, enabled: bool) {
        self.sound_enabled = enabled;
        self.update_timestamp();
    }

    /// Set auto-away timeout in minutes
    pub fn set_auto_away_minutes(&mut self, minutes: u32) {
        self.auto_away_minutes = minutes;
        self.update_timestamp();
    }

    /// Update the last_updated timestamp
    fn update_timestamp(&mut self) {
        self.last_updated = current_timestamp();
    }

    /// Check if a room is in recent rooms
    pub fn is_recent_room(&self, room_id: &str) -> bool {
        self.recent_rooms.iter().any(|r| r == room_id)
    }

    /// Get the most recent room (if any)
    pub fn get_most_recent_room(&self) -> Option<&String> {
        self.recent_rooms.front()
    }

    /// Get display name or fallback to username
    pub fn effective_display_name(&self) -> &str {
        if self.display_name.is_empty() {
            &self.username
        } else {
            &self.display_name
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Check if this user data has been modified recently (within last hour)
    pub fn is_recently_modified(&self) -> bool {
        let now = current_timestamp();
        now - self.last_updated < 3600 // 1 hour
    }

    /// Get the age of this user data in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = current_timestamp();
        now.saturating_sub(self.created_at)
    }
}

impl Entity for UserData {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn key_prefix() -> &'static str {
        "user_data"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::database::Database;
    use serial_test::serial;

    fn create_test_user_data() -> UserData {
        let mut user = UserData::new("test_user", "Test User");
        user.set_avatar_url(Some("https://example.com/avatar.jpg".to_string()));
        user.set_status_message(Some("Working from home".to_string()));
        user.add_recent_room("room1".to_string());
        user.add_recent_room("room2".to_string());
        user.set_theme("light".to_string());
        user.set_language("es".to_string());
        user.set_notifications_enabled(false);
        user.set_sound_enabled(false);
        user.set_auto_away_minutes(30);
        user
    }

    #[test]
    fn test_user_data_creation() {
        let user = UserData::new("alice", "Alice Smith");

        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Smith");
        assert_eq!(user.theme, "dark");
        assert_eq!(user.language, "en");
        assert_eq!(user.max_recent_rooms, 10);
        assert!(user.notifications_enabled);
        assert!(user.sound_enabled);
        assert_eq!(user.auto_away_minutes, 15);
        assert!(user.recent_rooms.is_empty());
        assert!(user.avatar_url.is_none());
        assert!(user.status_message.is_none());
    }

    #[test]
    fn test_default_for_user() {
        let user = UserData::default_for_user("bob");

        assert_eq!(user.username, "bob");
        assert_eq!(user.display_name, "bob");
        assert_eq!(user.effective_display_name(), "bob");
    }

    #[test]
    fn test_effective_display_name() {
        let mut user = UserData::new("alice", "");
        assert_eq!(user.effective_display_name(), "alice");

        user.set_display_name("Alice Smith".to_string());
        assert_eq!(user.effective_display_name(), "Alice Smith");
    }

    #[test]
    fn test_recent_rooms_management() {
        let mut user = UserData::new("test", "Test");

        // Test adding rooms
        user.add_recent_room("room1".to_string());
        user.add_recent_room("room2".to_string());
        user.add_recent_room("room3".to_string());

        let recent = user.get_recent_rooms();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0], "room3"); // Most recent first
        assert_eq!(recent[1], "room2");
        assert_eq!(recent[2], "room1");

        // Test duplicate handling
        user.add_recent_room("room1".to_string());
        let recent = user.get_recent_rooms();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0], "room1"); // Moved to front

        // Test max limit
        user.set_max_recent_rooms(2);
        let recent = user.get_recent_rooms();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], "room1");
        assert_eq!(recent[1], "room3");

        // Test is_recent_room
        assert!(user.is_recent_room("room1"));
        assert!(!user.is_recent_room("room2"));

        // Test get_most_recent_room
        assert_eq!(user.get_most_recent_room(), Some(&"room1".to_string()));

        // Test remove_recent_room
        user.remove_recent_room("room1");
        assert!(!user.is_recent_room("room1"));
        assert_eq!(user.get_most_recent_room(), Some(&"room3".to_string()));

        // Test clear_recent_rooms
        user.clear_recent_rooms();
        assert!(user.get_recent_rooms().is_empty());
        assert!(user.get_most_recent_room().is_none());
    }

    #[test]
    fn test_setters_update_timestamp() {
        let mut user = UserData::new("test", "Test");

        // Set a known timestamp in the past
        user.last_updated = 1000000000; // Way in the past
        let original_timestamp = user.last_updated;

        user.set_display_name("New Name".to_string());
        assert!(user.last_updated > original_timestamp);

        let _prev_timestamp = user.last_updated;

        // Set timestamp back again and test another setter
        user.last_updated = 1000000000;
        user.set_avatar_url(Some("new-url".to_string()));
        assert!(user.last_updated > 1000000000);
    }

    #[test]
    fn test_json_serialization() {
        let user = create_test_user_data();

        // Test to_json
        let json = user.to_json().expect("Should serialize to JSON");
        assert!(json.contains("test_user"));
        assert!(json.contains("Test User"));
        assert!(json.contains("Working from home"));

        // Test from_json
        let deserialized = UserData::from_json(&json).expect("Should deserialize from JSON");
        assert_eq!(deserialized.username, user.username);
        assert_eq!(deserialized.display_name, user.display_name);
        assert_eq!(deserialized.status_message, user.status_message);
        assert_eq!(deserialized.theme, user.theme);
        assert_eq!(deserialized.language, user.language);
        assert_eq!(
            deserialized.notifications_enabled,
            user.notifications_enabled
        );
        assert_eq!(deserialized.recent_rooms, user.recent_rooms);
    }

    #[test]
    fn test_preferences_setters() {
        let mut user = UserData::default();

        user.set_theme("light".to_string());
        assert_eq!(user.theme, "light");

        user.set_language("fr".to_string());
        assert_eq!(user.language, "fr");

        user.set_notifications_enabled(false);
        assert!(!user.notifications_enabled);

        user.set_sound_enabled(false);
        assert!(!user.sound_enabled);

        user.set_auto_away_minutes(60);
        assert_eq!(user.auto_away_minutes, 60);
    }

    #[test]
    fn test_age_and_recent_modification() {
        let user = UserData::new("test", "Test");

        // Should be recently created
        assert!(user.age_seconds() < 5);
        assert!(user.is_recently_modified());
    }

    #[test]
    #[serial(local_db)]
    fn test_save_and_load_user_data() -> Result<()> {
        let db = Database::new();
        let _ = db.clear();

        // Create test user data
        let mut user_data = create_test_user_data();

        // Save the UserData
        let key = db.save_entity(&mut user_data)?;
        assert!(user_data.id.is_some());

        // Load it back
        let loaded_user: Option<UserData> = db.load_entity(&key)?;

        // Verify it was loaded correctly
        assert!(loaded_user.is_some());
        let loaded = loaded_user.unwrap();

        assert_eq!(loaded.username, user_data.username);
        assert_eq!(loaded.display_name, user_data.display_name);
        assert_eq!(loaded.avatar_url, user_data.avatar_url);
        assert_eq!(loaded.status_message, user_data.status_message);
        assert_eq!(loaded.recent_rooms, user_data.recent_rooms);
        assert_eq!(loaded.theme, user_data.theme);
        assert_eq!(loaded.language, user_data.language);
        assert_eq!(
            loaded.notifications_enabled,
            user_data.notifications_enabled
        );
        assert_eq!(loaded.sound_enabled, user_data.sound_enabled);
        assert_eq!(loaded.auto_away_minutes, user_data.auto_away_minutes);
        assert_eq!(loaded.id, user_data.id);

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_update_user_data() -> Result<()> {
        let db = Database::new();
        let _ = db.clear();

        // Create and save initial user data
        let mut user_data = UserData::new("alice", "Alice");
        let key = db.save_entity(&mut user_data)?;

        // Update the user data
        user_data.set_display_name("Alice Smith".to_string());
        user_data.set_theme("light".to_string());
        user_data.add_recent_room("room123".to_string());

        // Save the updated data
        db.save_entity(&mut user_data)?;

        // Load it back
        let loaded_user: Option<UserData> = db.load_entity(&key)?;
        assert!(loaded_user.is_some());
        let loaded = loaded_user.unwrap();

        // Verify updates were persisted
        assert_eq!(loaded.display_name, "Alice Smith");
        assert_eq!(loaded.theme, "light");
        assert!(loaded.is_recent_room("room123"));

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_delete_user_data() -> Result<()> {
        let db = Database::new();
        let _ = db.clear();

        // Create and save user data
        let mut user_data = create_test_user_data();
        let key = db.save_entity(&mut user_data)?;

        // Verify it exists
        let loaded: Option<UserData> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        // Delete it
        let _deleted: UserData = db.delete(&key)?;

        // Verify it's gone
        let loaded_after_delete: Option<UserData> = db.load_entity(&key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_multiple_user_data() -> Result<()> {
        let db = Database::new();
        let _ = db.clear();

        // Create multiple users
        let mut alice = UserData::new("alice", "Alice");
        let mut bob = UserData::new("bob", "Bob");
        let mut charlie = UserData::new("charlie", "Charlie");

        // Save them
        let alice_key = db.save_entity(&mut alice)?;
        let bob_key = db.save_entity(&mut bob)?;
        let charlie_key = db.save_entity(&mut charlie)?;

        // Load them back individually
        let loaded_alice: Option<UserData> = db.load_entity(&alice_key)?;
        let loaded_bob: Option<UserData> = db.load_entity(&bob_key)?;
        let loaded_charlie: Option<UserData> = db.load_entity(&charlie_key)?;

        assert!(loaded_alice.is_some());
        assert!(loaded_bob.is_some());
        assert!(loaded_charlie.is_some());

        assert_eq!(loaded_alice.unwrap().username, "alice");
        assert_eq!(loaded_bob.unwrap().username, "bob");
        assert_eq!(loaded_charlie.unwrap().username, "charlie");

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_load_nonexistent_user_data() -> Result<()> {
        let db = Database::new();
        let _ = db.clear();

        // Try to load non-existent user data
        let loaded: Option<UserData> = db.load_entity("nonexistent_key")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    fn test_entity_trait_implementation() {
        let mut user = UserData::new("test", "Test");

        // Test initial state
        assert!(user.id().is_none());
        assert_eq!(UserData::key_prefix(), "user_data");

        // Test setting ID
        user.set_id("test_id_123".to_string());
        assert_eq!(user.id(), Some("test_id_123"));
        assert_eq!(user.id, Some("test_id_123".to_string()));
    }
}
