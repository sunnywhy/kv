use crate::{KvError, Kvpair, Value}

mod memory;

pub trait Storage {
    /// get a value from a key
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// set a new key/value pair, return the old value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// check if key exists
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// delete a key from the table, return the old value
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// get all the kv pairs from a table
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// get a Iterator for returning kv pair 
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}