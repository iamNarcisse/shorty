extern crate redis;
use redis::{Commands, Connection, RedisError, RedisResult};
use std::env;

// Cache
pub struct Store {
    store: Connection,
}

impl Store {
    pub fn new() -> Result<Store, RedisError> {
        let conn_string = match env::var("REDIS_URL") {
            Ok(val) => val,
            Err(_e) => panic!("REDIS_URL is required"),
        };
        let client = redis::Client::open(conn_string)?;
        let store = client.get_connection()?;
        Ok(Store { store })
    }

    pub fn save(&mut self, key: &str, value: &str) -> RedisResult<String> {
        return self.store.set(key, value);
    }

    pub fn retrieve(&mut self, key: &str) -> Option<String> {
        let result: RedisResult<String> = self.store.get(key);
        match result {
            Ok(val) => Some(val),
            Err(_e) => None,
        }
    }
}
