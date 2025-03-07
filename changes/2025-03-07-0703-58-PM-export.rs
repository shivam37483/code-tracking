//! Tools for exporting parsed metrics to a PostgreSQL database.

use std::collections::HashMap;
use std::error::Error;
use tokio_postgres::NoTls;

/// Exports parsed metrics to a PostgreSQL database.
///
/// This function establishes a connection to a PostgreSQL database, creates the `tor_metrics`
/// table if it doesn’t exist, and inserts each metric from the provided map. Metrics are stored
/// with their name, value, and an automatic timestamp.
///
/// # Arguments
/// - `metrics`: A map of metric names to their values (e.g., "relay_count" -> 1000).
/// - `db_params`: PostgreSQL connection string (e.g., "host=localhost user=postgres").
///
/// # Returns
/// - `Ok(())` if all metrics are successfully exported.
/// - `Err(Box<dyn Error>)` if the connection fails or a query cannot be executed.
///
/// # Examples
///