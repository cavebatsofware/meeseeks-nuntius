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

use sea_orm::*;
use sea_orm_migration::MigrationStatus;
use std::time::Duration;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = get_database_url();

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(50)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(1800));

    Database::connect(opt).await
}

fn get_database_url() -> String {
    dotenvy::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!(
            "DATABASE_URL environment variable must be set and should not use insecure defaults."
        );
    })
}

pub async fn ping_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await
}

pub async fn close_connection(db: DatabaseConnection) -> Result<(), DbErr> {
    db.close().await
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    use crate::migration::{Migrator, MigratorTrait};

    // Apply all pending migrations
    Migrator::up(db, None).await
}

pub async fn check_migration_status(
    db: &DatabaseConnection,
) -> Result<Vec<MigrationStatus>, DbErr> {
    use crate::migration::{Migrator, MigratorTrait};

    let migrations = Migrator::get_pending_migrations(db).await?;
    Ok(migrations.into_iter().map(|m| m.status()).collect())
}
