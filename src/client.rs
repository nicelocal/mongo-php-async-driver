use ext_php_rs::types::{ZendClassObject};
use ext_php_rs::{prelude::*};

use anyhow::{Context};

use mongodb::{Client, options::ClientOptions};

use crate::database;

#[php_class]
pub struct MongoClientOptions {
    opts: ClientOptions
}

#[php_impl]
impl MongoClientOptions {
    pub async fn parse(s: &str) -> anyhow::Result<Self> {
        Ok(Self{ opts: ClientOptions::parse(s).await.context("Could not parse client options")? })
    }
}

#[php_class]
pub struct MongoClient {
    client: Client
}

#[php_impl]
impl MongoClient {
    pub fn withOptions(opts: &ZendClassObject<MongoClientOptions>) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::with_options(opts.opts.clone()).context("Could not create client!")?
        })
    }
    pub async fn withUriStr(uri: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::with_uri_str(uri).await.context("Could not create client")?
        })
    }

    pub fn database(&self, name: &str) -> database::MongoDatabase {
        database::MongoDatabase {
            database: self.client.database(name)
        }
    }

    pub fn defaultDatabase(&self) -> Option<database::MongoDatabase> {
        match self.client.default_database() {
            None => None,
            Some(database) => Some(database::MongoDatabase {
                database: database
            })
        }
    }
}


