//! Tools for exporting parsed bridge pool assignment data to a PostgreSQL database.

use crate::parse::ParsedBridgePoolAssignment;
use anyhow::{Context, Result as AnyhowResult};
use chrono::{DateTime, Utc, NaiveDateTime};
use tokio_postgres::{NoTls, Transaction};
use sha2::{Sha256, Digest};

/// Exports parsed bridge pool assignment data to a PostgreSQL database.
///
/// This function connects to a PostgreSQL database, creates the necessary tables (`bridge_pool_assignments_file`
/// and `bridge_pool_assignment`) if they don’t exist, and inserts the parsed data. It uses a transaction to ensure
/// data consistency across both tables.
///
/// # Arguments
/// - `parsed_assignments`: A vector of `ParsedBridgePoolAssignment` structs containing the parsed data.
/// - `db_params`: PostgreSQL connection string (e.g., "host=localhost user=postgres password=example").
/// - `clear`: If `true`, truncates the tables before inserting new data.
///
/// # Returns
/// - `Ok(())` if all data is successfully exported.
/// - `Err(anyhow::Error)` if the connection fails or a query cannot be executed.
pub async fn export_to_postgres(
    parsed_assignments: Vec<ParsedBridgePoolAssignment>,
    db_params: &str,
    clear: bool,
) -> AnyhowResult<()> {
    let (mut client, connection) = tokio_postgres::connect(db_params, NoTls)
        .await
        .context("Failed to connect to PostgreSQL")?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let transaction = client
        .transaction()
        .await
        .context("Failed to start transaction")?;

    create_tables(&transaction)
        .await
        .context("Failed to create tables")?;

    if clear {
        transaction
            .execute("TRUNCATE TABLE bridge_pool_assignment CASCADE", &[])
            .await
            .context("Failed to truncate bridge_pool_assignment")?;
... (truncated for brevity)