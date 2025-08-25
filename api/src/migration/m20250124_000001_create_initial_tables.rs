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

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create message_tokens table
        manager
            .create_table(
                Table::create()
                    .table(MessageToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageToken::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MessageToken::TokenHash)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(MessageToken::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageToken::MaxMessages)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageToken::MessagesUsed)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(MessageToken::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(MessageToken::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MessageToken::LastUsedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Create relay_messages table
        manager
            .create_table(
                Table::create()
                    .table(RelayMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelayMessage::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelayMessage::RecipientHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelayMessage::SenderPublicKey)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelayMessage::EncryptedContent)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RelayMessage::Nonce).binary().not_null())
                    .col(
                        ColumnDef::new(RelayMessage::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelayMessage::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create relay_keys table
        manager
            .create_table(
                Table::create()
                    .table(RelayKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelayKey::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RelayKey::PublicKey).binary().not_null())
                    .col(ColumnDef::new(RelayKey::PrivateKey).binary().not_null())
                    .col(
                        ColumnDef::new(RelayKey::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add indexes for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_relay_messages_recipient_hash")
                    .table(RelayMessage::Table)
                    .col(RelayMessage::RecipientHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_relay_messages_expires_at")
                    .table(RelayMessage::Table)
                    .col(RelayMessage::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_message_tokens_expires_at")
                    .table(MessageToken::Table)
                    .col(MessageToken::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RelayKey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RelayMessage::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(MessageToken::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum MessageToken {
    Table,
    Id,
    TokenHash,
    ExpiresAt,
    MaxMessages,
    MessagesUsed,
    Status,
    CreatedAt,
    LastUsedAt,
}

#[derive(DeriveIden)]
enum RelayMessage {
    Table,
    Id,
    RecipientHash,
    SenderPublicKey,
    EncryptedContent,
    Nonce,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum RelayKey {
    Table,
    Id,
    PublicKey,
    PrivateKey,
    CreatedAt,
}
