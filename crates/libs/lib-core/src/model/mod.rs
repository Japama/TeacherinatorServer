//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

// region:    --- Modules

use crate::model::store::{new_db_pool, PostgresDb};

pub use self::error::{Error, Result};

mod base;
mod error;
pub mod modql_utils;
mod store;
pub mod user;
pub mod department;
pub mod subject;
pub mod group;
pub mod classroom;
pub mod schedule;
pub mod schedule_hour;
pub mod control;
pub mod center_schedule_hour;
pub mod building;
pub mod classroom_type;


// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    postgres_db: PostgresDb,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let postgres_db = new_db_pool().await?;

        Ok(ModelManager { postgres_db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub(in crate::model) fn postgres_db(&self) -> &PostgresDb {
        &self.postgres_db
    }
}
