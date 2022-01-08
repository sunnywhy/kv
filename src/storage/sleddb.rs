use std::path::Path;

use sled::Db;

#[derive(Debug)]
pub struct SledDb(Db);

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }

    // in sleddb, since it can do scan_prefix, we use prefix to simulate a table
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    // when iterating through the table keys, we use prefix: as table
    fn get_table_prefix(table: &str) -> String {
        format!("{}:", table)
    }
}