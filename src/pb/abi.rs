/// command request from client
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(oneof="command_request::RequestData", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `commandRequest`.
pub mod command_request {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag="1")]
        Hget(super::Hget),
        #[prost(message, tag="2")]
        Hgetall(super::Hgetall),
        #[prost(message, tag="3")]
        Hmget(super::Hmget),
        #[prost(message, tag="4")]
        Hset(super::Hset),
        #[prost(message, tag="5")]
        Hmset(super::Hmset),
        #[prost(message, tag="6")]
        Hdel(super::Hdel),
        #[prost(message, tag="7")]
        Hmdel(super::Hmdel),
        #[prost(message, tag="8")]
        Hexist(super::Hexist),
        #[prost(message, tag="9")]
        Hmexist(super::Hmexist),
    }
}
/// response from server
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    /// status code, re-use HTTP status code like 2xx/4xx/5xx
    #[prost(uint32, tag="1")]
    pub status: u32,
    /// used when status code != 2xx, detail error message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    /// returned values
    #[prost(message, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// returned kv pairs
    #[prost(message, repeated, tag="4")]
    pub pairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// use a key to get a value from a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hget {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// get all the Kvpairs from a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hgetall {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
}
/// use a list of keys, to get their values from a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmget {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// returned value
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag="3")]
        Integer(i64),
        #[prost(double, tag="4")]
        Float(f64),
        #[prost(bool, tag="5")]
        Bool(bool),
    }
}
/// returned Kvpair
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kvpair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Value>,
}
/// save a Kvpair into a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hset {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pair: ::core::option::Option<Kvpair>,
}
/// save a list of Kvpairs into a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmset {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// delete a key from a table, return the former value
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hdel {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// delete a list of keys from a table, return their former values
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmdel {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// check if key exist in a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hexist {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// check if a list of keys exist in a table
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmexist {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
