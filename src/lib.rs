#![cfg_attr(windows, feature(abi_vectorcall))]
#![feature(thread_local, local_key_cell_methods)]
#![allow(non_snake_case)]
mod client;
mod collection;
mod cursor;
mod database;
mod conversion;
mod bson;

use crate::client::MongoClient;
use crate::client::MongoClientOptions;
use crate::collection::MongoCollection;
use crate::collection::MongoFindOptions;
use crate::database::MongoDatabase;
use crate::cursor::MongoCursor;
use crate::bson::MongoObjectId;

use conversion::PhpDocument;
use conversion::PhpRawDocument;

use ext_php_rs::types::ZendClassObject;

use ext_php_rs::zend::EventLoop;

use ext_php_rs::zend::{ce};
use ext_php_rs::{prelude::*};

#[php_function]
pub fn tokio_init() -> PhpResult<u64>{
    EventLoop::init()
}

#[php_function]
pub fn tokio_wakeup() -> PhpResult<()> {
    EventLoop::wakeup()
}

pub extern "C" fn request_shutdown(_type: i32, _module_number: i32) -> i32 {
    EventLoop::shutdown();
    0
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .request_shutdown_function(request_shutdown)
}

