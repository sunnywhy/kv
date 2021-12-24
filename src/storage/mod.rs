mod memory;
pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    // #[test]
    // fn memtable_iter_should_work() {
    //     let store = MemTable::new();
    //     test_get_iter(store);
    // }

    fn test_basi_interface(store: impl Storage) {
        // first set will create the table, insert key, and return None
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        // set the same key, it will update and return the former value
        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));

        // get existing key will get the latest value
        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        // get non-existing key/table will get None
        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert_eq!(Ok(None), store.get("t2", "hello"));

        // contains existing key will return true, otherwise return false
        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        // del existing key will return the former value
        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        // del non-existing key will return Ok(None)
        assert_eq!(store.del("t1", "hello1"), Ok(None));
        assert_eq!(store.del("t2", "hello"), Ok(None));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }
}
