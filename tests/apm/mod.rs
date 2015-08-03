use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use bson::Bson;
use mongodb::{Client, CommandResult, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use rand;

fn timed_query(_client: Client, command_result: &CommandResult) {
    let duration = match command_result {
        &CommandResult::Success { duration, .. } => duration,
        _ => panic!("Command failed!")
    };

    // Sanity check
    assert!(duration >= 1500000000);

    // Technically not guaranteed, but since the query is running locally, it shouldn't even be close
    assert!(duration < 2000000000);
}

#[test]
fn command_duration() {
    let mut client = Client::connect("localhost", 27017).ok().expect("damn it!");
    let db = client.db("test");
    let coll = db.collection("event_hooks");
    coll.drop().unwrap();

    let docs = (1..4).map(|i| doc! { "_id" => i, "x" => (rand::random::<u8>() as u32) }).collect();
    coll.insert_many(docs, false, None).unwrap();
    client.add_completion_hook(timed_query).unwrap();

    let doc = doc! {
        "$where" => (Bson::JavaScriptCode("function() { sleep(500); }".to_owned()))
    };

    coll.find(Some(doc), None).unwrap();
}

#[test]
fn logging() {
    for file in fs::read_dir(".").unwrap() {
        if file.unwrap().file_name().eq("test_log.txt") {
            fs::remove_file("test_log.txt").unwrap();
        }
    }

    let client = Client::connect_with_log_file("localhost", 27017, "test_log.txt").unwrap();
    let db = client.db("test");
    let coll = db.collection("logging");
    coll.drop().unwrap();

    let doc1 = doc! { "_id" => 1 };
    let doc2 = doc! { "_id" => 2 };
    let doc3 = doc! { "_id" => 3 };

    coll.insert_one(doc1, None).unwrap();
    coll.insert_one(doc2, None).unwrap();
    coll.insert_one(doc3, None).unwrap();

    let filter = doc! {
        "_id" => { "$gt" => 1 }
    };

    coll.find(Some(filter), None).unwrap();

    let f = File::open("test_log.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut line = String::new();

    // Drop collection started
    file.read_line(&mut line).unwrap();
    assert_eq!("COMMAND.drop_collection 127.0.0.1:27017 STARTED: { drop: \"logging\" }\n", &line);

    // Drop collection completed
    line.clear();
    file.read_line(&mut line).unwrap();
    // Can't assert the contents of the response until `create_collection` is implemented, otherwise
    // the collection might not exist, so there might be an error message.
    assert!(line.starts_with("COMMAND.drop_collection 127.0.0.1:27017 COMPLETED: {"));

    // First insert started
    line.clear();
    file.read_line(&mut line).unwrap();
    assert_eq!("COMMAND.insert_one 127.0.0.1:27017 STARTED: { insert: \"logging\", documents: [{ _id: 1 }], ordered: true, writeConcern: { w: 1, wtimeout: 0, j: false } }\n", &line);

    // First insert completed
    line.clear();
    file.read_line(&mut line).unwrap();
    assert!(line.starts_with("COMMAND.insert_one 127.0.0.1:27017 COMPLETED: { ok: 1, n: 1 } ("));
    assert!(line.ends_with(" ns)\n"));

    // Second insert started
    line.clear();
    file.read_line(&mut line).unwrap();
    assert_eq!("COMMAND.insert_one 127.0.0.1:27017 STARTED: { insert: \"logging\", documents: [{ _id: 2 }], ordered: true, writeConcern: { w: 1, wtimeout: 0, j: false } }\n", &line);

    // Second insert completed
    line.clear();
    file.read_line(&mut line).unwrap();
    assert!(line.starts_with("COMMAND.insert_one 127.0.0.1:27017 COMPLETED: { ok: 1, n: 1 } ("));
    assert!(line.ends_with(" ns)\n"));

    // Third insert started
    line.clear();
    file.read_line(&mut line).unwrap();
    assert_eq!("COMMAND.insert_one 127.0.0.1:27017 STARTED: { insert: \"logging\", documents: [{ _id: 3 }], ordered: true, writeConcern: { w: 1, wtimeout: 0, j: false } }\n", &line);

    // Third insert completed
    line.clear();
    file.read_line(&mut line).unwrap();
    assert!(line.starts_with("COMMAND.insert_one 127.0.0.1:27017 COMPLETED: { ok: 1, n: 1 } ("));
    assert!(line.ends_with(" ns)\n"));

    // Find command started
    line.clear();
    file.read_line(&mut line).unwrap();
    assert_eq!("COMMAND.find 127.0.0.1:27017 STARTED: { find: \"logging\", filter: {  }, projection: {  }, skip: 0, limit: 0, batchSize: 20, sort: {  } }\n", &line);

    // Find command completed
    line.clear();
    file.read_line(&mut line).unwrap();
    assert!(line.starts_with("COMMAND.find 127.0.0.1:27017 COMPLETED: { cursor: { id: 0, ns: \"test.logging\", firstBatch: [{ _id: 2 }, { _id: 3 }] }, ok: 1 } ("));
    assert!(line.ends_with(" ns)\n"));
}