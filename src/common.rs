use bson::{self, Bson};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    Primary,
    PrimaryPreferred,
    Secondary,
    SecondaryPreferred,
    Nearest,
}

#[derive(Debug, Clone)]
pub struct ReadPreference {
    pub mode: ReadMode,
    pub tags: Vec<bson::Document>,
}

impl ReadPreference {
    pub fn new(mode: ReadMode, tags: Option<Vec<bson::Document>>) -> ReadPreference {
        ReadPreference {
            mode: mode,
            tags: tags.unwrap_or(Vec::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteConcern {
    pub w: i32,          // Write replication
    pub w_timeout: i32,  // Used in conjunction with 'w'. Propagation timeout in ms.
    pub j: bool,         // If true, will block until write operations have been committed to journal.
    pub fsync: bool,     // If true and server is not journaling, blocks until server has synced all data files to disk.
}

impl WriteConcern {
    pub fn new() -> WriteConcern {
        WriteConcern {
            w: 1,
            w_timeout: 0,
            j: false,
            fsync: false,
        }
    }

    pub fn to_bson(&self) -> bson::Document {
        let mut bson = bson::Document::new();
        bson.insert("w".to_owned(), Bson::I32(self.w));
        bson.insert("wtimeout".to_owned(), Bson::I32(self.w_timeout));
        bson.insert("j".to_owned(), Bson::Boolean(self.j));
        bson
    }
}
