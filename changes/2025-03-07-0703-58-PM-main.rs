//! Tor Metrics MVP: Fetch, Parse, and Export Consensus Documents to PostgreSQL
//!
//! This application demonstrates fetching Tor network consensus documents from CollecTor,
//! parsing their contents into metrics, and exporting the results to a PostgreSQL database.
//! It mirrors the style and structure of the Tor Project's `metrics-lib`, adapted for Rust.
//!
//! ## Purpose
//! The goal is to showcase a minimal yet functional pipeline for processing Tor consensus
//! data in Rust, with the final step being storage in a PostgreSQL database.
//!
//! ## Design Overview
//! - **Fetching**: Retrieves consensus documents from CollecTor using the `fetch` module.
//! - **Parsing**: Extracts metrics (e.g., relay counts) using the `parse` module.
//! - **Exporting**: Saves parsed metrics to a PostgreSQL database via the `export` module.
//!
//! ## Dependencies
//! - `reqwest`: For HTTP requests to fetch data from CollecTor.
//! - `tokio`: For asynchronous runtime to handle network and database operations.
//! - `tokio-postgres`: For PostgreSQL database interaction.
//! - `log` and `env_logger`: For structured logging instead of `println!`.
//! - `clap`: For parsing command-line arguments to configure the application.
//! - `dotenv`: For loading environment variables from a `.env` file.
//!
//! These dependencies are stable and widely used, aligning with the guideline to minimize
//! external dependencies while enhancing functionality.
//!
//! ## Usage
//! 1. Ensure a PostgreSQL database is running (e.g., database `tor_metrics`, user `postgres`,
//!    password `2099`).
//! 2. Configure the application using either a `.env` file or command-line arguments:
//!    - **Using a `.env` file**: Create a `.env` file in the project root with:
//!