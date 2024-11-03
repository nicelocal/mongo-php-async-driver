
use ext_php_rs::{prelude::*};
use mongodb::Collection;
use mongodb::bson::{RawDocumentBuf};
use mongodb::options::FindOptions;
use php_tokio::php_async_impl;

use crate::conversion::{PhpRawDocument, PhpDocument};
use crate::cursor::MongoCursor;

use std::time::Duration;

use anyhow::Context;


#[php_class]
pub struct MongoFindOptions {
    options: FindOptions
}

#[php_async_impl]
impl MongoFindOptions {
    pub fn __construct(
        allow_disk_use: Option<bool>,
        allow_partial_results: Option<bool>,
        batch_size: Option<u32>,
        comment: Option<String>,
        _comment_bson: Option<PhpDocument>,
        _cursor_type: Option<u8>,
        limit: Option<i64>,
        max: Option<PhpDocument>,
        max_await_time: Option<u64>,
        max_scan: Option<u64>,
        max_time: Option<u64>,
        min: Option<PhpDocument>,
        no_cursor_timeout: Option<bool>,
        projection: Option<PhpDocument>,
        return_key: Option<bool>,
        show_record_id: Option<bool>,
        skip: Option<u64>,
        sort: Option<PhpDocument>,
        let_vars: Option<PhpDocument>,
    ) -> PhpResult<Self> {
        Ok(Self {
            options: FindOptions::builder()
                .allow_disk_use(allow_disk_use)
                .allow_partial_results(allow_partial_results)
                .batch_size(batch_size)
                .comment(comment)
                .comment_bson(None) //comment_bson
                .cursor_type(None) //cursor_type,
                .hint(None)
                .limit(limit)
                .max(max.and_then(|v|Some(v.0)))
                .max_await_time(max_await_time.and_then(|v| Some(Duration::from_nanos(v))))
                .max_scan(max_scan)
                .max_time(max_time.and_then(|v| Some(Duration::from_nanos(v))))
                .min(min.and_then(|v|Some(v.0)))
                .no_cursor_timeout(no_cursor_timeout)
                .projection(projection.and_then(|v|Some(v.0)))
                .return_key(return_key)
                .show_record_id(show_record_id)
                .skip(skip)
                .sort(sort.and_then(|v|Some(v.0)))
                .collation(None)
                .let_vars(let_vars.and_then(|v|Some(v.0)))
                .read_concern(None)
                .selection_criteria(None)
                .build()
        })
    }
}

#[php_class]
pub struct MongoCollection {
    pub collection: Collection<RawDocumentBuf>,
}

#[php_async_impl]
impl MongoCollection {
    pub async fn find(&self, filter: Option<PhpDocument>, options: Option<&MongoFindOptions>) -> anyhow::Result<MongoCursor> {
        Ok(MongoCursor::new(
            this.collection
                .find(
                    filter.and_then(|v|Some(v.0)),
                    match options {
                        None => None,
                        Some(v) => Some(v.options.clone())
                    },
                )
                .await
                .context("find failed")?,
        ))
    }
    pub async fn find_one(&self, filter: Option<PhpDocument>) -> anyhow::Result<Option<PhpRawDocument>> {
        Ok(match this.collection
            .find_one(
                filter.and_then(|v|Some(v.0)),
                None,
            )
            .await
            .context("find failed")?
        {
            None => None,
            Some(v) => Some(v.into())
        })
    }
}
