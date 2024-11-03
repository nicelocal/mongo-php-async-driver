
use ext_php_rs::{prelude::*};
use mongodb::bson::{RawDocumentBuf};
use php_tokio::{RUNTIME, php_async_impl};
use crate::conversion::{PhpRawDocument};
use mongodb::{Cursor};
use tokio::sync::mpsc::{self, Receiver as TokioReceiver};

use anyhow::{Context, anyhow};



#[php_class]
#[implements(ce::iterator())]
pub struct MongoCursor {
    rx: TokioReceiver<mongodb::error::Result<RawDocumentBuf>>, 
    buf: Option<PhpRawDocument>,
    count: u32,
    started: bool,
    done: bool
}

impl MongoCursor {
    pub fn new(mut cursor: Cursor<RawDocumentBuf>) -> Self {
        let (tx, rx) = mpsc::channel(1);
        RUNTIME.spawn(async move {
            loop {
                if let Ok(permit) = tx.reserve().await {
                    permit.send(match cursor.advance().await {
                        Err(e) => Err(e),
                        Ok(v) => if v == true { cursor.deserialize_current() } else { break }
                    });
                } else {
                    break
                }
            }
            drop(tx);
        });
        Self{
            rx: rx,
            buf: None,
            count: 0,
            started: false,
            done: false
        }
    }
}

#[php_async_impl]
impl MongoCursor {
    pub fn current(&mut self) -> Option<&PhpRawDocument> {
        (&self.buf).as_ref()
    }
    pub fn key(&self) -> u32 {
        self.count
    }
    pub async fn next(&mut self) -> anyhow::Result<bool> {
        if let Some(v) = this.rx.recv().await {
            this.buf = Some(v.context("An error occurred while advancing")?.into());
            if this.started {
                this.count += 1;
            } else {
                this.started = true;
            }
            return Ok(true);
        }
        this.done = true;
        return Ok(false);
    }
    pub async fn rewind(&mut self) -> anyhow::Result<()> {
        if this.started {
            return Err(anyhow!("Cannot rewind iterator"))
        }
        if let Some(v) = this.rx.recv().await {
            this.buf = Some(v.context("An error occurred while advancing")?.into());
            this.started = true;
            return Ok(());
        }
        this.done = true;
        Ok(())
    }
    pub fn valid(&self) -> bool {
        !self.done
    }
}