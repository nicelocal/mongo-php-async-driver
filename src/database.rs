use ext_php_rs::prelude::*;
use mongodb::Database;

use crate::collection;


#[php_class]
pub struct MongoDatabase {
    pub database: Database
}

#[php_impl]
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

