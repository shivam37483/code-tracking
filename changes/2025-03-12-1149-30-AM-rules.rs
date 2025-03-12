use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Rule {
    pub keyword: String,
    pub score: f32,
}

pub struct RuleManager {
    conn: Connection,
    rules: Arc<Mutex<Vec<Rule>>>,
}

impl RuleManager {
    // pub fn new(db_path: &str) -> Result<Self> {
    //     let conn = Connection::open(db_path)?; // Creates rules.db if it doesn’t exist
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS rules (
    //             id INTEGER PRIMARY KEY,
    //             keyword TEXT NOT NULL,
    //             score REAL NOT NULL
    //         )",
    //         [],
    //     )?;
        
    //     let mut stmt = conn.prepare("SELECT keyword, score FROM rules")?;
    //     let rule_iter = stmt.query_map([], |row| {
    //         Ok(Rule {
    //             keyword: row.get(0)?,
    //             score: row.get(1)?,
    //         })
    //     })?;
    //     let rules = rule_iter.collect::<Result<Vec<_>>>()?;
    //     Ok(Self {
    //         conn,
    //         rules: Arc::new(Mutex::new(rules)),
    //     })
    // }

    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY,
                keyword TEXT NOT NULL,
                score REAL NOT NULL
            )",
            [],
        )?;
... (truncated for brevity)