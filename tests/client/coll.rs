use bson;
use bson::Bson;

use mongodb::client::MongoClient;

#[test]
fn find_and_insert() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert document
    let mut doc = bson::Document::new();
    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    coll.insert_one(doc, None).ok().expect("Failed to insert document");

    // Find document
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = cursor.next().unwrap();

    // Assert expected title of document
    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    };

    assert!(cursor.next().is_none());
}

#[test]
fn find_and_insert_one() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert document
    let mut doc = bson::Document::new();
    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    coll.insert_one(doc, None).ok().expect("Failed to insert document");

    // Find single document
    let result = coll.find_one(None, None).ok().expect("Failed to execute find command.");
    assert!(result.is_some());

    // Assert expected title of document
    match result.unwrap().get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    };
}

#[test]
fn insert_many() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    coll.insert_many(vec!(doc, doc2), false, None).ok().expect("Failed to insert documents.");

    // Find documents
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(2);
    assert_eq!(2, results.len());

    // Assert expected title of documents
    match results[0].get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }
    match results[1].get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Back to the Future", title),
        _ => panic!("Expected Bson::String!"),
    }
}

#[test]
fn delete_one() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    coll.insert_many(vec!(doc.clone(), doc2.clone()), false, None)
        .ok().expect("Failed to insert documents.");

    // Delete document
    coll.delete_one(doc2.clone(), None).ok().expect("Failed to delete document.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = cursor.next().unwrap();

    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }

    assert!(cursor.next().is_none());
}

#[test]
fn delete_many() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    coll.insert_many(vec!(doc.clone(), doc2.clone(), doc2.clone()), false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Delete document
    coll.delete_many(doc2.clone(), None).ok().expect("Failed to delete documents.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = cursor.next().unwrap();

    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }

    assert!(cursor.next().is_none());
}

#[test]
fn replace_one() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();
    let mut doc3 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    doc3.insert("title".to_owned(), Bson::String("12 Angry Men".to_owned()));
    coll.insert_many(vec!(doc.clone(), doc2.clone(), doc3.clone()), false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Replace single document
    coll.replace_one(doc2.clone(), doc3.clone(), false, None).ok().expect("Failed to replace document.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3);
    assert_eq!(3, results.len());

    // Assert expected title of documents
    match results[0].get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    };
    match results[1].get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("12 Angry Men", title),
        _ => panic!("Expected Bson::String!"),
    };
    match results[2].get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("12 Angry Men", title),
        _ => panic!("Expected Bson::String!"),
    };
}

#[test]
fn update_one() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();
    let mut doc3 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    doc3.insert("title".to_owned(), Bson::String("12 Angry Men".to_owned()));
    coll.insert_many(vec!(doc.clone(), doc2.clone(), doc3.clone()), false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Update single document
    let mut update = bson::Document::new();
    let mut set = bson::Document::new();

    set.insert("director".to_owned(), Bson::String("Robert Zemeckis".to_owned()));
    update.insert("$set".to_owned(), Bson::Document(set));
    coll.update_one(doc2.clone(), update, false, None).ok().expect("Failed to update document.");

    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3);
    assert_eq!(3, results.len());

    // Assert director attributes
    assert!(results[0].get("director").is_none());
    assert!(results[2].get("director").is_none());
    match results[1].get("director") {
        Some(&Bson::String(ref director)) => assert_eq!("Robert Zemeckis", director),
        _ => panic!("Expected Bson::String!"),
    }
}

#[test]
fn update_many() {
    let client = MongoClient::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("test");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let mut doc = bson::Document::new();
    let mut doc2 = bson::Document::new();
    let mut doc3 = bson::Document::new();

    doc.insert("title".to_owned(), Bson::String("Jaws".to_owned()));
    doc2.insert("title".to_owned(), Bson::String("Back to the Future".to_owned()));
    doc3.insert("title".to_owned(), Bson::String("12 Angry Men".to_owned()));
    coll.insert_many(vec!(doc.clone(), doc2.clone(), doc3.clone(), doc2.clone()), false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Update single document
    let mut update = bson::Document::new();
    let mut set = bson::Document::new();

    set.insert("director".to_owned(), Bson::String("Robert Zemeckis".to_owned()));
    update.insert("$set".to_owned(), Bson::Document(set));
    coll.update_many(doc2.clone(), update, false, None).ok().expect("Failed to update documents.");

    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(4);
    assert_eq!(4, results.len());

    // Assert director attributes
    for i in 0..3 {
        // Check title
        match results[i].get("title") {
            Some(&Bson::String(ref title)) => {
                let dir_opt = results[i].get("director");

                // Only doc2 models should have a director field
                if "Back to the Future" == title {
                    match dir_opt {
                        Some(&Bson::String(ref director)) => assert_eq!("Robert Zemeckis", director),
                        _ => panic!("Expected Bson::String!"),
                    }
                } else {
                    assert!(dir_opt.is_none());
                }
            },
            _ => panic!("Expected Bson::String!"),
        }
    }
}
