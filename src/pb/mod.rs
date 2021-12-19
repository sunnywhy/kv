pub mod abi;

use abi::{command_request::RequestData, *};

impl CommandRequest {
    // create HSET command
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> CommandRequest {
        Self { 
            request_data: Some(RequestData::Hset(Hset { 
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// convert to Value from String
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// convert to Value from &str 
impl  From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}