use std::path::Path;

use sled::Db;

use crate::Storage;

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

/// flip Option<Result<T, E>> into Result<Option<T>, E>
fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

impl Storage for SledDb {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn set(&self, table: &str, key: String, value: crate::Value) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, crate::KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::KvError> {
        todo!()
    }
}