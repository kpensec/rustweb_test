use database::Database;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use redis::Commands;

pub struct Ctx {
    pub db: RwLock<Database>
}

impl Ctx {
    pub fn new() -> Ctx { Ctx{db: RwLock::new(Database::new())} }

    pub fn read_db(&self) -> RwLockReadGuard<Database> {
        self.db.read().unwrap()
    }

    pub fn write_db(&self) -> RwLockWriteGuard<Database> {
        self.db.write().unwrap()
    }

    pub fn foo(&self) -> redis::RedisResult<i32> {
        let client = redis::Client::open("redis://127.0.0.1:6379/")?;
        let con = client.get_connection()?;
        // throw away the result, just make sure it does not fail
        let value : i32 = con.get("foo").unwrap_or(0i32);
        Ok(value)
    }

    pub fn bar(&self, v: i32) -> redis::RedisResult<()> {
        let client = redis::Client::open("redis://127.0.0.1:6379/")?;
        let con = client.get_connection()?;
        let _ : () = con.set("foo", 42)?;
        Ok(())

    }
}

// allow our ctx to be valid as a Juniper Context. It will be accessible
// with ""executor.context()" from our GraphQL resolvers
impl juniper::Context for Ctx {}
