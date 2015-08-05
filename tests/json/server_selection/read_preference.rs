use mongodb::common::{ReadMode, ReadPreference};
use rustc_serialize::json::{Json, Object};
use std::collections::BTreeMap;
use std::str::FromStr;

use super::super::FromJsonResult;

impl FromJsonResult for ReadPreference {
    fn from_json(object: &Object) -> Result<ReadPreference, String> {
        let mode = val_or_err!(object.get("mode"),
                               Some(&Json::String(ref s)) => ReadMode::from_str(s).unwrap(),
                               "read preference must have a mode.");

        let tag_sets_array = val_or_err!(object.get("tag_sets"),
                                         Some(&Json::Array(ref arr)) => arr.clone(),
                                         "read preference must have tag sets");

        let mut tag_sets_objs = Vec::new();
        let mut tag_sets = Vec::new();
        
        for json in tag_sets_array.into_iter() {
            match json {
                Json::Object(ref obj) => tag_sets_objs.push(obj.clone()),
                _ => return Err("tags must be document objects.".to_owned()),
            }
        }

        for obj in tag_sets_objs.into_iter() {
            let mut tags = BTreeMap::new();
            for (ref key, ref json) in obj {
                match json {
                    &Json::String(ref s) => { tags.insert(key.to_owned(), s.to_owned()); },
                    _ => return Err("tags must be string => string maps.".to_owned()),
                }
            }
            tag_sets.push(tags);
        }

        Ok(ReadPreference::new(mode, Some(tag_sets)))
    }
}
