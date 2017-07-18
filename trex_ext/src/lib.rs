#![allow(dead_code)]
#![allow(unused_imports)]
extern crate disk_utils;
extern crate trex;
extern crate serde;
extern crate serde_json;

pub mod serde_trex;

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, RwLock};

use disk_utils::wal::{LogData, LogStore};
use disk_utils::wal::entries::{ChangeEntry, Checkpoint, InsertEntry, SingleLogEntry, Transaction};

use disk_utils::wal::undo_log::UndoLog;

#[derive(Clone, PartialEq, Debug)]
struct MyLogData;

impl LogData for MyLogData {
    type Key = i32;
    type Value = String;
}

#[derive(Clone)]
struct MyStore<Data: LogData> {
    map: Arc<RwLock<HashMap<Data::Key, Data::Value>>>,
    flush_err: Arc<RwLock<bool>>,
}

impl<Data> MyStore<Data>
    where Data: LogData
{
    pub fn new() -> MyStore<Data> {
        MyStore {
            map: Arc::new(RwLock::new(HashMap::new())),
            flush_err: Arc::new(RwLock::new(false)),
        }
    }

    pub fn set_flush_err(&mut self, flush_err: bool) {
        *self.flush_err.write().unwrap() = flush_err;
    }
}

impl<Data> LogStore<Data> for MyStore<Data>
    where Data: LogData
{
    fn get(&self, key: &Data::Key) -> Option<Data::Value> {
        self.map.read().unwrap().get(key).cloned()
    }

    fn remove(&mut self, key: &Data::Key) {
        self.map.write().unwrap().remove(key);
    }

    fn update(&mut self, key: Data::Key, val: Data::Value) {
        self.map.write().unwrap().insert(key, val);
    }

    fn flush(&mut self) -> io::Result<()> {
        if *self.flush_err.read().unwrap() {
            Err(io::Error::new(io::ErrorKind::Interrupted, "Flush error occurred"))
        } else {
            Ok(())
        }
    }

    fn flush_change(&mut self, _: Data::Key, _: Data::Value) -> io::Result<()> {
        if *self.flush_err.read().unwrap() {
            Err(io::Error::new(io::ErrorKind::Interrupted, "Flush error occurred"))
        } else {
            Ok(())
        }
    }
}

use disk_utils::testing::create_test_file;

#[test]
fn test_start() {
    create_test_file("./files/start_undo_log", |path, _| {
        let store: MyStore<MyLogData> = MyStore::new();
        let mut undo_log = UndoLog::new(path, store).unwrap();
        let tid = undo_log.start();

        assert_eq!(tid, 1);
        assert_eq!(undo_log.entries().len(), 1);
        assert_eq!(undo_log.entries()[0], SingleLogEntry::Transaction(Transaction::Start(1)));
    }).unwrap();
}
