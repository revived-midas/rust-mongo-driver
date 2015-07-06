use bson::Bson;

use mongodb::Client;
use mongodb::coll::options::{FindOneAndUpdateOptions, ReturnDocument};

#[test]
fn find_and_insert() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("find_and_insert");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert document
    let doc = doc! { "title" => "Jaws" };
    coll.insert_one(doc, None).ok().expect("Failed to insert document");

    // Find document
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = match cursor.next() {
        Some(Ok(res)) => res,
        Some(Err(_)) => panic!("Received error from 'cursor.next()'."),
        None => panic!("Expected bson."),
    };

    // Assert expected title of document
    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    };

    assert!(cursor.next().is_none());
}

#[test]
fn find_and_insert_one() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("find_and_insert");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert document
    let doc = doc! { "title" => "Jaws" };
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
fn find_one_and_delete() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("find_one_and_delete");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };

    coll.insert_many(vec![doc1.clone(), doc2.clone()], false, None)
        .ok().expect("Failed to insert documents.");

    // Find and Delete document
    let result = coll.find_one_and_delete(doc2.clone(), None)
        .ok().expect("Failed to execute find_one_and_delete command.");

    match result.unwrap().get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Back to the Future", title),
        _ => panic!("Expected Bson::String!"),
    }

    // Validate state of collection
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = match cursor.next() {
        Some(Ok(res)) => res,
        Some(Err(_)) => panic!("Received error from 'cursor.next()'."),
        None => panic!("Expected bson."),
    };

    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }

    assert!(cursor.next().is_none());
}

#[test]
fn find_one_and_replace() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("find_one_and_replace");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };
    let doc3 = doc! { "title" => "12 Angry Men" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Replace single document
    let result = coll.find_one_and_replace(doc2.clone(), doc3.clone(), None)
        .ok().expect("Failed to execute find_one_and_replace command.");

    match result.unwrap().get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Back to the Future", title),
        _ => panic!("Expected Bson::String!"),
    }

    // Validate state of collection
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3).ok().expect("Failed to get next 3 from cursor.");
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

    // Replace with 'new' option
    let mut opts = FindOneAndUpdateOptions::new();
    opts.return_document = ReturnDocument::After;
    let result = coll.find_one_and_replace(doc3.clone(), doc2.clone(), Some(opts))
        .ok().expect("Failed to execute find_one_and_replace command.");

    match result.unwrap().get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Back to the Future", title),
        _ => panic!("Expected Bson::String!"),
    }
}

#[test]
fn find_one_and_update() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("find_one_and_update");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };
    let doc3 = doc! { "title" => "12 Angry Men" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Update single document
    let update = doc! { "$set" => { "director" => "Robert Zemeckis" } };

    let result = coll.find_one_and_update(doc2.clone(), update, None)
        .ok().expect("Failed to execute find_one_and_update command.");

    match result.unwrap().get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Back to the Future", title),
        _ => panic!("Expected Bson::String!"),
    }

    // Validate state of collection
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3).ok().expect("Failed to get next 3 from cursor.");
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
fn aggregate() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("aggregate");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "tags" => ["a", "b", "c"] };
    let doc2 = doc! { "tags" => ["a", "b", "d"] };
    let doc3 = doc! { "tags" => ["d", "e", "f"] };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone()], false, None)
        .ok().expect("Failed to execute insert_many command.");

    // Build aggregation pipeline to unwind tag arrays and group distinct tags
    let project = doc! { "$project" => { "tags" => 1 } };
    let unwind = doc! { "$unwind" => ("$tags") };
    let group = doc! { "$group" => { "_id" => "$tags" } };

    // Aggregate
    let mut cursor = coll.aggregate(vec![project, unwind, group], None)
        .ok().expect("Failed to execute aggregate command.");

    let results = cursor.next_n(10).ok().expect("Failed to get next 10 from cursor.");
    assert_eq!(6, results.len());

    // Grab ids from aggregated docs
    let vec: Vec<_> = results.iter().filter_map(|bdoc| {
        match bdoc.get("_id") {
            Some(&Bson::String(ref tag)) => Some(tag.to_owned()),
            _ => None,
        }
    }).collect();

    // Validate that all distinct tags were received.
    assert_eq!(6, vec.len());
    assert!(vec.contains(&"a".to_owned()));
    assert!(vec.contains(&"b".to_owned()));
    assert!(vec.contains(&"c".to_owned()));
    assert!(vec.contains(&"d".to_owned()));
    assert!(vec.contains(&"e".to_owned()));
    assert!(vec.contains(&"f".to_owned()));
}

#[test]
fn count() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("count");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };

    let mut vec = vec![doc1.clone()];
    for _ in 0..10 {
        vec.push(doc2.clone());
    }

    coll.insert_many(vec, false, None).ok().expect("Failed to insert documents.");
    let count_doc1 = coll.count(Some(doc1), None).ok().expect("Failed to execute count.");
    assert_eq!(1, count_doc1);

    let count_doc2 = coll.count(Some(doc2), None).ok().expect("Failed to execute count.");
    assert_eq!(10, count_doc2);

    let count_all = coll.count(None, None).ok().expect("Failed to execute count.");
    assert_eq!(11, count_all);

    let no_doc = doc! { "title" => "Houdini" };
    let count_none = coll.count(Some(no_doc), None).ok().expect("Failed to execute count.");
    assert_eq!(0, count_none);
}

#[test]
fn distinct_none() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("distinct_none");

    db.drop_database().ok().expect("Failed to drop database");
    let distinct_titles = coll.distinct("title", None, None).ok().expect("Failed to execute 'distinct'.");
    assert_eq!(0, distinct_titles.len());
}

#[test]
fn distinct_one() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("distinct_none");

    db.drop_database().ok().expect("Failed to drop database");
    let doc2 = doc! { "title" => "Back to the Future" };
    coll.insert_one(doc2, None).ok().expect("Failed to insert document.");

    let distinct_titles = coll.distinct("title", None, None).ok().expect("Failed to execute 'distinct'.");
    assert_eq!(1, distinct_titles.len());
}

#[test]
fn distinct() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("distinct");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws",
                      "director" => "MB" };

    let doc2 = doc! { "title" => "Back to the Future" };

    let doc3 = doc! { "title" => "12 Angry Men",
                      "director" => "MB" };

    let mut vec = vec![doc1.clone()];
    for _ in 0..4 {
        vec.push(doc2.clone());
    }
    for _ in 0..60 {
        vec.push(doc3.clone());
    }

    coll.insert_many(vec, false, None).ok().expect("Failed to insert documents.");

    // Distinct titles over all documents
    let distinct_titles = coll.distinct("title", None, None).ok().expect("Failed to execute 'distinct'.");
    assert_eq!(3, distinct_titles.len());

    let titles: Vec<_> = distinct_titles.iter().filter_map(|bson| match bson {
        &Bson::String(ref title) => Some(title.to_owned()),
        _ => None,
    }).collect();

    assert_eq!(3, titles.len());
    assert!(titles.contains(&"Jaws".to_owned()));
    assert!(titles.contains(&"Back to the Future".to_owned()));
    assert!(titles.contains(&"12 Angry Men".to_owned()));

    // Distinct titles over documents with certain director
    let filter = doc! { "director" => "MB" };
    let distinct_titles = coll.distinct("title", Some(filter), None)
        .ok().expect("Failed to execute 'distinct'.");

    assert_eq!(2, distinct_titles.len());

    let titles: Vec<_> = distinct_titles.iter().filter_map(|bson| match bson {
        &Bson::String(ref title) => Some(title.to_owned()),
        _ => None,
    }).collect();

    assert_eq!(2, titles.len());
    assert!(titles.contains(&"Jaws".to_owned()));
    assert!(titles.contains(&"12 Angry Men".to_owned()));
}

#[test]
fn insert_many() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("insert_many");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };

    coll.insert_many(vec![doc1, doc2], false, None).ok().expect("Failed to insert documents.");

    // Find documents
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(2).ok().expect("Failed to get next 2 from cursor.");
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
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("delete_one");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };

    coll.insert_many(vec![doc1.clone(), doc2.clone()], false, None)
        .ok().expect("Failed to insert documents.");

    // Delete document
    coll.delete_one(doc2.clone(), None).ok().expect("Failed to delete document.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = match cursor.next() {
        Some(Ok(res)) => res,
        Some(Err(_)) => panic!("Received error from 'cursor.next()'."),
        None => panic!("Expected bson."),
    };

    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }

    assert!(cursor.next().is_none());
}

#[test]
fn delete_many() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("delete_many");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc2.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Delete document
    coll.delete_many(doc2.clone(), None).ok().expect("Failed to delete documents.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let result = match cursor.next() {
        Some(Ok(res)) => res,
        Some(Err(_)) => panic!("Received error from 'cursor.next()'."),
        None => panic!("Expected bson."),
    };

    match result.get("title") {
        Some(&Bson::String(ref title)) => assert_eq!("Jaws", title),
        _ => panic!("Expected Bson::String!"),
    }

    assert!(cursor.next().is_none());
}

#[test]
fn replace_one() {
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("replace_one");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };
    let doc3 = doc! { "title" => "12 Angry Men" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Replace single document
    coll.replace_one(doc2.clone(), doc3.clone(), false, None).ok().expect("Failed to replace document.");
    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3).ok().expect("Failed to get next 3 from cursor.");
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
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("update_one");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };
    let doc3 = doc! { "title" => "12 Angry Men" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Update single document
    let update = doc! { "$set" => { "director" => "Robert Zemeckis" } };

    coll.update_one(doc2.clone(), update, false, None).ok().expect("Failed to update document.");

    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(3).ok().expect("Failed to get next 3 from cursor.");
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
    let client = Client::with_uri("mongodb://localhost:27017").unwrap();
    let db = client.db("test");
    let coll = db.collection("update_many");

    db.drop_database().ok().expect("Failed to drop database");

    // Insert documents
    let doc1 = doc! { "title" => "Jaws" };
    let doc2 = doc! { "title" => "Back to the Future" };
    let doc3 = doc! { "title" => "12 Angry Men" };

    coll.insert_many(vec![doc1.clone(), doc2.clone(), doc3.clone(), doc2.clone()], false, None)
        .ok().expect("Failed to insert documents into collection.");

    // Update single document
    let update = doc! { "$set" => { "director" => "Robert Zemeckis" } };

    coll.update_many(doc2.clone(), update, false, None).ok().expect("Failed to update documents.");

    let mut cursor = coll.find(None, None).ok().expect("Failed to execute find command.");
    let results = cursor.next_n(4).ok().expect("Failed to get next 4 from cursor.");
    assert_eq!(4, results.len());

    // Assert director attributes
    assert!(results[0].get("director").is_none());
    assert!(results[2].get("director").is_none());

    match results[1].get("director") {
        Some(&Bson::String(ref director)) => assert_eq!("Robert Zemeckis", director),
        _ => panic!("Expected Bson::String for director!"),
    }

    match results[3].get("director") {
        Some(&Bson::String(ref director)) => assert_eq!("Robert Zemeckis", director),
        _ => panic!("Expected Bson::String for director!"),
    }
}
