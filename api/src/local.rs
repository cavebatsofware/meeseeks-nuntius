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

//! Local api functions - these run on the same device as the client
//! These handle sensitive operations like database access and cryptography

use dioxus::prelude::*;
use crate::persistence::database::{Database, Entity};
use crate::crypto::message::{Contact, Room};

// Room management functions (local database operations)
pub async fn create_room(
    name: String,
    description: Option<String>,
) -> Result<String, ServerFnError> {
    let db = Database::new();
    let mut room = Room::new(&name);
    if let Some(desc) = description {
        room.description = desc;
    }
    db.save_entity(&mut room)
        .map_err(|e| ServerFnError::new(e.to_string()))
}

pub async fn get_room(id: String) -> Result<Option<String>, ServerFnError> {
    let db = Database::new();
    match db
        .load_entity::<Room>(&id)
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(room) => Ok(Some(
            room.to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )),
        None => Ok(None),
    }
}

pub async fn update_room(room_json: String) -> Result<(), ServerFnError> {
    let db = Database::new();
    let room = Room::from_json(&room_json).map_err(|e| ServerFnError::new(e.to_string()))?;
    db.update_entity(&room)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

pub async fn delete_room(id: String) -> Result<(), ServerFnError> {
    let db = Database::new();
    db.delete::<Room>(&id)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

pub async fn get_all_rooms() -> Result<Vec<String>, ServerFnError> {
    let db = Database::new();
    let rooms = db
        .load_all_entities::<Room>(Room::key_prefix())
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let mut result = Vec::new();
    for room in rooms {
        result.push(
            room.to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        );
    }
    Ok(result)
}

pub async fn find_room_by_name(name: String) -> Result<Option<String>, ServerFnError> {
    let db = Database::new();
    match db
        .find_entity::<Room, _>(Room::key_prefix(), |room| room.name == name)
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(room) => Ok(Some(
            room.to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )),
        None => Ok(None),
    }
}

// Contact management functions (local database + crypto operations)
pub async fn create_contact(name: String, public_key: String) -> Result<String, ServerFnError> {
    let db = Database::new();
    use crypto_box::PublicKey;

    let public_key_bytes: [u8; 32] = hex::decode(&public_key)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .try_into()
        .map_err(|_| ServerFnError::new("Invalid public key length"))?;
    let public_key = PublicKey::from(public_key_bytes);

    let mut contact = Contact::new(&name, &public_key);
    db.save_entity(&mut contact)
        .map_err(|e| ServerFnError::new(e.to_string()))
}

pub async fn get_contact(id: String) -> Result<Option<String>, ServerFnError> {
    let db = Database::new();
    match db
        .load_entity::<Contact>(&id)
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(contact) => Ok(Some(
            contact
                .to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )),
        None => Ok(None),
    }
}

pub async fn update_contact(contact_json: String) -> Result<(), ServerFnError> {
    let db = Database::new();
    let contact =
        Contact::from_json(&contact_json).map_err(|e| ServerFnError::new(e.to_string()))?;
    db.update_entity(&contact)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

pub async fn delete_contact(id: String) -> Result<(), ServerFnError> {
    let db = Database::new();
    db.delete::<Contact>(&id)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

pub async fn get_all_contacts() -> Result<Vec<String>, ServerFnError> {
    let db = Database::new();
    let contacts = db
        .load_all_entities::<Contact>(Contact::key_prefix())
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let mut result = Vec::new();
    for contact in contacts {
        result.push(
            contact
                .to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        );
    }
    Ok(result)
}

pub async fn find_contact_by_name(name: String) -> Result<Option<String>, ServerFnError> {
    let db = Database::new();
    match db
        .find_entity::<Contact, _>(Contact::key_prefix(), |contact| contact.name == name)
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(contact) => Ok(Some(
            contact
                .to_json()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )),
        None => Ok(None),
    }
}