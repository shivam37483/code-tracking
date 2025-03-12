use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use rlua::Lua;

#[derive(Clone)]
pub struct Rule {
    pub keyword: String,
    pub score: f32,
}

pub struct RuleManager {
    conn: Mutex<Connection>,
    rules: Arc<Mutex<Vec<Rule>>>,
}

impl RuleManager {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Mutex::new(Connection::open(db_path)?);
        {
            let conn = conn.lock().unwrap();
            conn.execute(
                "CREATE TABLE IF NOT EXISTS rules (
                    id INTEGER PRIMARY KEY,
                    keyword TEXT NOT NULL,
                    score REAL NOT NULL
                )",
                [],
            )?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS senders (
                    user_id TEXT PRIMARY KEY,
                    spam_score INTEGER DEFAULT 0,
                    message_count INTEGER DEFAULT 0
                )",
                [],
            )?;
        }
        let rules = {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare("SELECT keyword, score FROM rules")?;
            let rule_iter = stmt.query_map([], |row| {
                Ok(Rule {
                    keyword: row.get(0)?,
                    score: row.get(1)?,
                })
            })?;
            rule_iter.collect::<Result<Vec<_>>>()?
        };
        Ok(Self {
            conn,
... (truncated for brevity)