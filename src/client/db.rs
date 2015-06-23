use bson;
use bson::Bson;

use client::MongoClient;
use client::coll::Collection;
use client::coll::options::FindOptions;
use client::common::{ReadPreference, WriteConcern};
use client::cursor::{Cursor, DEFAULT_BATCH_SIZE};

use client::Result;
use client::Error::OperationError;

/// Interfaces with a MongoDB database.
pub struct Database<'a> {
    pub name: String,
    pub client: &'a MongoClient,
    pub read_preference: ReadPreference,
    pub write_concern: WriteConcern,
}

impl<'a> Database<'a> {
    /// Creates a database representation with optional read and write controls.
    pub fn new(client: &'a MongoClient, name: &str,
               read_preference: Option<ReadPreference>, write_concern: Option<WriteConcern>) -> Database<'a> {

        let rp = read_preference.unwrap_or(client.read_preference.to_owned());
        let wc = write_concern.unwrap_or(client.write_concern.to_owned());

        Database {
            name: name.to_owned(),
            client: client,
            read_preference: rp,
            write_concern: wc,
        }
    }

    /// Creates a collection representation with inherited read and write controls.
    pub fn collection(&'a self, coll_name: &str) -> Collection<'a> {
        Collection::new(self, coll_name, false, Some(self.read_preference.to_owned()), Some(self.write_concern.to_owned()))
    }

    /// Creates a collection representation with custom read and write controls.
    pub fn collection_with_prefs(&'a self, coll_name: &str, create: bool,
                                 read_preference: Option<ReadPreference>, write_concern: Option<WriteConcern>) -> Collection<'a> {
        Collection::new(self, coll_name, create, read_preference, write_concern)
    }

    /// Return a unique operational request id.
    pub fn get_req_id(&self) -> i32 {
        self.client.get_req_id()
    }

    /// Generates a cursor for a relevant operational command.
    pub fn command_cursor(&self, spec: bson::Document) -> Result<Cursor> {
        Cursor::command_cursor(self.client, &self.name[..], spec)
    }

    /// Sends an administrative command over find_one.
    pub fn command(&'a self, spec: bson::Document) -> Result<bson::Document> {
        let coll = self.collection("$cmd");
        let mut options = FindOptions::new();
        options.batch_size = 1;
        let res = try!(coll.find_one(Some(spec.clone()), Some(options)));
        res.ok_or(OperationError(format!("Failed to execute command with spec {:?}", spec)))
    }

    /// Returns a list of collections within the database.
    pub fn list_collections(&'a self, filter: Option<bson::Document>) -> Result<Cursor> {
        self.list_collections_with_batch_size(filter, DEFAULT_BATCH_SIZE)
    }

    pub fn list_collections_with_batch_size(&'a self, filter: Option<bson::Document>,
                                            batch_size: i32) -> Result<Cursor> {

        let mut spec = bson::Document::new();
        let mut cursor = bson::Document::new();

        cursor.insert("batchSize".to_owned(), Bson::I32(batch_size));
        spec.insert("listCollections".to_owned(), Bson::I32(1));
        spec.insert("cursor".to_owned(), Bson::Document(cursor));
        if filter.is_some() {
            spec.insert("filter".to_owned(), Bson::Document(filter.unwrap()));
        }

        self.command_cursor(spec)
    }


    /// Returns a list of collection names within the database.
    pub fn collection_names(&'a self, filter: Option<bson::Document>) -> Result<Vec<String>> {
        let mut cursor = try!(self.list_collections(filter));
        let mut results = Vec::new();
        loop {
            match cursor.next() {
                Some(Ok(doc)) => if let Some(&Bson::String(ref name)) = doc.get("name") {
                    results.push(name.to_owned());
                },
                Some(Err(err)) => return Err(err),
                None => return Ok(results),
            }
        }
    }

    /// Creates a new collection.
    ///
    /// Note that due to the implicit creation of collections during insertion, this
    /// method should only be used to instantiate capped collections.
    pub fn create_collection(&'a self, name: &str) -> Result<()> {
        unimplemented!()
    }

    /// Permanently deletes the database from the server.
    pub fn drop_database(&'a self) -> Result<()> {
        let mut spec = bson::Document::new();
        spec.insert("dropDatabase".to_owned(), Bson::I32(1));
        try!(self.command(spec));
        Ok(())
    }

    /// Permanently deletes the collection from the database.
    pub fn drop_collection(&'a self, name: &str) -> Result<()> {
        let mut spec = bson::Document::new();
        spec.insert("drop".to_owned(), Bson::String(name.to_owned()));
        try!(self.command(spec));
        Ok(())
    }
}
