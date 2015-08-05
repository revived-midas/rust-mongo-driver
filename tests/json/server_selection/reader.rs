use json::FromJsonResult;

use mongodb::common::ReadPreference;
use mongodb::topology::TopologyType;

use rustc_serialize::json::Json;

use std::fs::File;
use std::str::FromStr;

use super::server::Server;
use super::topology_description::TopologyDescription;

pub struct Suite {
    pub in_latency_window: Vec<Server>,
    pub write: bool,
    pub read_preference: ReadPreference,
    pub suitable_servers: Vec<Server>,
    pub topology_description: TopologyDescription,
}

fn get_server_array(arr: &Vec<Json>) -> Result<Vec<Server>, String> {
    let mut servers = Vec::new();

    for json in arr.iter() {
        match json {
            &Json::Object(ref obj) => match Server::from_json(obj) {
                Ok(server) => servers.push(server),
                Err(err) => return Err(err),
            },
            _ => return Err("Some servers could not be parsed for topology".to_owned()),
        }
    }

    Ok(servers)
}

pub trait SuiteContainer {
    fn from_file(path: &str) -> Result<Self, String>;
    fn get_suite(&self) -> Result<Suite, String>;
}

impl SuiteContainer for Json {
    fn from_file(path: &str) -> Result<Json, String> {
        let mut file = File::open(path).ok().expect(&format!("Unable to open file: {}", path));
        Ok(Json::from_reader(&mut file).ok().expect(&format!("Invalid JSON file: {}", path)))
    }

    fn get_suite(&self) -> Result<Suite, String> {
        let object = val_or_err!(self,
                                 &Json::Object(ref object) => object.clone(),
                                 "`get_suite` requires a JSON object");

        let operation = val_or_err!(object.get("operation"),
                                    Some(&Json::String(ref s)) => s.to_owned(),
                                    "suite requires an operation string.");

        let write = operation == "write";

        let read_preference = val_or_err!(object.get("read_preference"),
                                          Some(&Json::Object(ref object)) => try!(ReadPreference::from_json(object)),
                                          "suite requires a read_preference object.");

        let in_latency_window = val_or_err!(object.get("in_latency_window"),
                                           Some(&Json::Array(ref array)) => try!(get_server_array(array)),
                                           "suite requires an in_latency_window array.");

        let suitable_servers = val_or_err!(object.get("suitable_servers"),
                                           Some(&Json::Array(ref array)) => try!(get_server_array(array)),
                                           "suite requires a suitable_servers array.");

        let topology_obj = val_or_err!(object.get("topology_description"),
                                       Some(&Json::Object(ref obj)) => obj,
                                       "suite requires a topology_description object.");

        let top_servers = val_or_err!(topology_obj.get("servers"),
                                      Some(&Json::Array(ref array)) => try!(get_server_array(array)),
                                      "topology requires an array of servers.");

        let ttype = val_or_err!(topology_obj.get("type"),
                                Some(&Json::String(ref s)) => TopologyType::from_str(s).unwrap(),
                                "topology requires a type");

        Ok(Suite {
            in_latency_window: in_latency_window,
            write: write,
            read_preference: read_preference,
            suitable_servers: suitable_servers,
            topology_description: TopologyDescription::new(top_servers, ttype),
        })
    }
}
