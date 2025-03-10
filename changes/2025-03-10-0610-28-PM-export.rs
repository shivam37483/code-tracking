//! Tools for exporting parsed bridge pool assignment data to a PostgreSQL database.
//!
//! This module provides functionality to export parsed bridge pool assignment data into a PostgreSQL database.
//! It manages database connections, table creation, and data insertion within a transactional context to ensure
//! consistency. The export process is optimized with batch inserts to handle large datasets efficiently.
//!
//! The `bridge_pool_assignments_file` table uses a SHA-256 digest (Base64-encoded without padding) of the raw file
//! content as its primary key, while the `bridge_pool_assignment` table uses a SHA-1 digest (hex-encoded) of each
//! assignment line as its primary key.
//!
//! ## Usage
//!
//! The main entry point is the [`export_to_postgres`] function, which takes a vector of parsed assignments,
//! a database connection string, and a flag to clear existing data. It establishes a connection, sets up tables,
//! and inserts data in a single transaction.
//!
//! ## Dependencies
//!
//! - **`tokio_postgres`**: Asynchronous PostgreSQL client for database operations.
//! - **`chrono`**: Handles timestamp conversions and formatting.
//! - **`anyhow`**: Simplifies error handling with context.
//! - **`sha1`**: Computes SHA-1 digests for individual assignments (via `parse.rs`).
//! - **`sha2`**: Computes SHA-256 digests for file integrity (via `parse.rs`).
//! - **`base64`**: Encodes file digests (via `parse.rs`).
//!
//! ## Error Handling
//!
//! All functions return `anyhow::Result` to provide detailed error messages for database failures, parsing issues,
//! or invalid data.

use crate::parse::{ParsedBridgePoolAssignment, calculate_file_digest};
use anyhow::{Context, Result as AnyhowResult};
use chrono::{DateTime, Utc};
use tokio_postgres::{NoTls, Transaction};

// Global constant to limit the number of files to export during testing
const MAX_FILES_TO_EXPORT: usize = 100;

/// Exports parsed bridge pool assignment data to a PostgreSQL database.
///
/// Connects to a PostgreSQL database, creates necessary tables if they don’t exist, and inserts the provided
/// parsed data. Uses a transaction to ensure atomicity across table operations. Optionally truncates existing
/// tables if the `clear` flag is set.
///
/// # Arguments
///
/// * `parsed_assignments` - Vector of parsed bridge pool assignments to export.
/// * `db_params` - PostgreSQL connection string (e.g., "host=localhost user=postgres password=example").
/// * `clear` - If `true`, truncates existing tables before inserting new data.
///
... (truncated for brevity)