use std::str::FromStr;

use anyhow::Context;

use ext_php_rs::php_class;
use ext_php_rs::php_impl;
use ext_php_rs::prelude::PhpResult;

use mongodb::bson::oid::ObjectId;

#[php_class]
pub struct MongoObjectId {
    pub id: ObjectId
}

#[php_impl]
impl MongoObjectId {
    pub fn __construct(id: Option<&str>) -> PhpResult<Self> {
        Ok(Self {
            id: match id {
                None => ObjectId::new(),
                Some(v) => ObjectId::from_str(v).context("An invalid object ID was provided!")?
            }
        })
    }
    pub fn __toString(&self) {
    }
}