use std::sync::Arc;

use crate::{Storage, CommandResponse, MemTable};


mod command_service;

/// abstract layer for Command
pub trait CommandService {
    /// handle the Command, and return Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{CommandRequest, Value};

    use super::*;
    
    #[test]
    fn service_should_work() {
        // we need a service to include the Storage
        let service = Service::new(MemTable::default());

        // service should able to run in multiple threads
        let cloned = service.clone();

        // create a new thread, insert k1 and k2 into table t1 
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });

        handle.join().unwrap();

        // read k1 from table t1, should return v1
        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Value, Kvpair};
#[cfg(test)]
fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

#[cfg(test)]
fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}