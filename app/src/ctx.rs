use database::Database;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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
}

// allow our ctx to be valid as a Juniper Context. It will be accessible
// with ""executor.context()" from our GraphQL resolvers
impl juniper::Context for Ctx {}
