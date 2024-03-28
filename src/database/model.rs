use bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub name: String,
    pub message: String,
    pub status: String,
    pub content: String,
}

pub fn new_entry(name: &str, message: &str, status: &str, content: &str) -> LogEntry {
    LogEntry {
        name: name.to_string(),
        message: message.to_string(),
        status: status.to_string(),
        content: content.to_string(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogSchema {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub created_at: DateTime,

    pub name: String,
    pub message: String,
    pub status: String,
    pub content: String,
}

impl Debug for LogSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LogSchema {{ id: {}, created_at: {}, name: {}, message: {}, status: {}, content: {} }}",
            self.id, self.created_at, self.name, self.message, self.status, self.content
        )
    }
}

impl LogSchema {
    pub fn from_entry(entry: LogEntry) -> Self {
        Self {
            id: ObjectId::new(),
            created_at: DateTime::now(),
            name: entry.name,
            message: entry.message,
            status: entry.status,
            content: entry.content,
        }
    }
}
