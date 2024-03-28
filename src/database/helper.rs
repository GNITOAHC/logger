use super::database;
use super::model::{LogEntry, LogSchema};

#[derive(Debug)]
pub struct Helper {
    pub db: database::MyDatabase,
}

impl Helper {
    pub async fn new_log(&self, entry: LogEntry) {
        let schema = LogSchema::from_entry(entry);
        let res = self.db.insert_one(schema).await;
        println!("(helper) Result: {:?}", res);
    }
}

pub fn new(db: database::MyDatabase) -> Helper {
    Helper { db }
}
