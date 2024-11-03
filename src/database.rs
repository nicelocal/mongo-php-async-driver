use ext_php_rs::prelude::*;
use mongodb::Database;
use php_tokio::php_async_impl;

use crate::collection;


#[php_class]
pub struct MongoDatabase {
    pub database: Database
}

#[php_async_impl]
impl MongoDatabase {
    pub fn name(&self) -> &str {
        self.database.name()
    }
    pub fn collection(&self, name: &str) -> collection::MongoCollection {
        collection::MongoCollection {
            collection: self.database.collection(name)
        }
    }
}

