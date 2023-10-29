use std::io::Result;

use clap::Parser;
use sqlx::SqlitePool;

use crate::ops;

/// Drop environment
#[derive(Parser)]
pub struct Cmd {
    /// Environment to drop
    env: String,
}

impl Cmd {
    pub async fn run(&self, db: &SqlitePool) -> Result<()> {
        ops::drop(db, &self.env).await
    }
}
