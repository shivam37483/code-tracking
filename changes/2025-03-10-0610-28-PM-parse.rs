//! # Parsing Tor Bridge Pool Assignment Files into Structured Metrics
//!
//! This module provides functionality to parse bridge pool assignment files fetched from a CollecTor
//! instance into structured data. It processes the raw textual content, extracting publication
//! timestamps and bridge assignment entries, which are then encapsulated in
//! `ParsedBridgePoolAssignment` structs for further analysis or storage.
//!
//! The raw file content is stored to compute a SHA-256 digest (Base64-encoded without padding), while
//! each assignment line’s SHA-1 digest (hex-encoded) is calculated for use as a unique identifier.
//!
//! ## Usage
//!
//! The main entry point is `parse_bridge_pool_files`, which accepts a vector of `BridgePoolFile`
//! structs and returns a vector of `ParsedBridgePoolAssignment` instances.
//!
//! ## Dependencies
//!
//! - `chrono`: For timestamp parsing and manipulation.
//! - `std::collections::BTreeMap`: For ordered storage of bridge entries.
//! - `anyhow`: For robust error handling.
//! - `sha1`: For computing SHA-1 digests of individual assignment lines.
//! - `sha2`: For computing SHA-256 digests of the entire file content.
//! - `base64`: For encoding the file digest.
//!
//! ## Error Handling
//!
//! Errors are handled with `anyhow::Result`, providing detailed context for parsing failures.

use crate::fetch::BridgePoolFile; // Import BridgePoolFile from fetch.rs
use anyhow::{Context, Result as AnyhowResult};
use chrono::NaiveDateTime;
use std::collections::BTreeMap;
use sha1::{Digest, Sha1};
use sha2::Sha256;
use base64::Engine;

/// Represents a parsed bridge pool assignment, containing the publication timestamp, raw file content,
/// and a map of bridge entries with their digests.
#[derive(Debug)]
pub struct ParsedBridgePoolAssignment {
  /// The time in milliseconds since the epoch when this descriptor was published.
  pub published_millis: i64,
  /// The raw content of the file, used for computing the file digest.
  pub file_content: String,
  /// A map of bridge fingerprints to a tuple of (assignment string, SHA-1 digest of the line).
  pub entries: BTreeMap<String, (String, String)>,
}


/// Parses bridge pool assignment files into a structured format.
... (truncated for brevity)