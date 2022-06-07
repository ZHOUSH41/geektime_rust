pub mod abi;

use crate::KvError;

use self::abi::{command_request::RequestData, *};
use http::StatusCode;

impl CommandRequest {
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hmget(table: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmget(Hmget {
                table: table.into(),
                keys: keys.into(),
            })),
        }
    }

    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair {
                    key: key.into(),
                    value: Some(value),
                }),
            })),
        }
    }

    pub fn new_hmset(table: impl Into<String>, pairs: impl Into<Vec<Kvpair>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmset(Hmset {
                table: table.into(),
                pairs: pairs.into(),
            })),
        }
    }
    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmdel(table: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmdel(Hmdel {
                table: table.into(),
                keys: keys.into(),
            })),
        }
    }

    pub fn new_hexist(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmexist(table: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmexist(Hmexist {
                table: table.into(),
                keys: keys.into(),
            })),
        }
    }
}

impl Kvpair {
    /// new KV pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(b)),
        }
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self {
            value: Some(value::Value::Float(f)),
        }
    }
}

/// 从 Value 转换成 CommandResponse
impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

/// 从 Vec<Kvpair> 转换成 CommandResponse
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

/// 从 Vec<Value> 转换成 CommandResponse
impl From<Vec<Value>> for CommandResponse {
    fn from(v: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: v,
            ..Default::default()
        }
    }
}

/// 从 KvError 转换成 CommandResponse
impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}
