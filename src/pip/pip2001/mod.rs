extern crate regex;

use serde::{Serialize};
use serde_json::{Value};
use serde::ser::{Serializer, SerializeStruct};
use regex::Regex;
use std::fmt;
use std::collections::HashMap;
use crate::pip;

fn is_pub_address(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(0x)?[A-Fa-f0-9]{40}$").unwrap();
    }
    RE.is_match(input)
}

fn is_hash_hex(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(0x)?[A-Fa-f0-9]+").unwrap();
    }
    RE.is_match(input)
}

#[derive(Copy, Clone)]
pub enum Pip2001MessageType {
    PUBLISH_MANAGEMENT,
    PUBLISH,
    NA
}

impl fmt::Debug for Pip2001MessageType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pip2001MessageType::PUBLISH_MANAGEMENT=> write!(f, "PUBLISH_MANAGEMENT"),
            Pip2001MessageType::PUBLISH=> write!(f, "PUBLISH"),
            Pip2001MessageType::NA=> write!(f, "N/A"),
        }
    }
}

pub struct Pip2001 {
    pub VERSION : i16  ,
    pub STATUS : String,
    pub Type: pip::PipId ,
    pub msg_type: Pip2001MessageType,
    pub data: HashMap<String, String>,
    pub meta: HashMap<String, pip::InputObject>
}


impl fmt::Debug for Pip2001{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\nVERSION: {}, \nSTATUS: {}\nType: {:?}\ndata: {:?}\nmeta: {:?}}}", self.VERSION, self.STATUS, self.Type, self.data, self.meta)
    }
}

impl Serialize for Pip2001{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Pip2001", 5)?;
        state.serialize_field("VERSION", &self.VERSION)?;
        state.serialize_field("STATUS", &self.STATUS)?;
        state.serialize_field("Type", &self.Type)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("meta", &self.meta)?;
        state.end()
    }
}

impl pip::Pip for Pip2001 {
    fn pip_id(&self) -> pip::PipId {
        pip::PipId::PIP2001
    }

    fn validate(&self) -> bool {
        true
    }

    //fn new(data: &HashMap<&str, &str>) -> Pip2001 {
    fn new() -> Pip2001 {
        //for (k , v) in data{
        //    println!("{}: \"{}\"", k, v);
        //}

        //let datatype = data.get("topic");
        //let exist = data.contains_key("topic1");
        //println!("{:?}", datatype.unwrap());
        //println!("exist {:?}", exist);
        Pip2001 {
            Type:pip::PipId::PIP2001,
            VERSION : 1,
            STATUS:  String::from("draft"),
            msg_type: Pip2001MessageType::NA,
            data: HashMap::new(),
            meta: HashMap::new(),
        }
    }

    fn from_dict(&mut self, data: HashMap<String, String>, meta: HashMap<String, pip::InputObject>) -> std::result::Result<Option<Self>, &'static str> where Self: Sized{
        let _datatype = data.get("topic");
        let verify_result = self.verify_fields(&data, &meta);
        match verify_result{
            Ok(pip2001msgtype) => {
                let verify_format_result = self.verify_object_format(pip2001msgtype, &data, &meta);
                match verify_format_result{
                    Ok(_v) => {
                        let msgtype = pip::PipId::PIP2001;
                        Ok(Some(Pip2001 {
                            Type:msgtype,
                            VERSION : 1,
                            STATUS:  String::from("draft"),
                            msg_type: pip2001msgtype,
                            data: data,
                            meta: meta,
                        }))
                    },
                    Err(e) => Err(e),
                }

            },
            Err(e) => Err(e),
        }
    }

    fn from_json(&mut self, jsonstr: &str) -> std::result::Result<Option<Self>, &'static str> where Self: Sized {
        let v: Value = serde_json::from_str(jsonstr).unwrap();
        let mut data: HashMap<String, String> = HashMap::new();
        let mut meta: HashMap<String, pip::InputObject> = HashMap::new();

        if !v["allow"].is_null()  {
            if let Value::String(_v) = &v["allow"] {
                data.insert(String::from("allow"), _v.clone());
            }
        }
        if !v["deny"].is_null()  {
            if let Value::String(_v) = &v["deny"] {
                data.insert(String::from("deny"), _v.clone());
            }
        }
        if !v["topic"].is_null()  {
            if let Value::String(_v) = &v["topic"] {
                data.insert(String::from("topic"), _v.clone());
            }
        }
        if !v["file_hash"].is_null()  {
            if let Value::String(_v) = &v["file_hash"] {
                data.insert(String::from("file_hash"), _v.clone());
            }
        }
        if !v["uris"].is_null()  {
            if let Value::String(_v) = &v["uris"] {
                let uris_vec: Vec<String> = serde_json::from_str(&_v)
                    .expect("parse meta.uris failed");
                let v = pip::InputObject::VecOfString(uris_vec);
                meta.insert(String::from("uris"), v);
            }
        }
        let verify_result = self.verify_fields(&data, &meta);
        match verify_result{
            Ok(pip2001msgtype) => {
                let verify_format_result = self.verify_object_format(pip2001msgtype, &data, &meta);
                match verify_format_result{
                    Ok(_) => {
                        let msgtype = pip::PipId::PIP2001;
                        Ok(Some(Pip2001 {
                            Type:msgtype,
                            VERSION : 1,
                            STATUS:  String::from("draft"),
                            msg_type: pip2001msgtype,
                            data: data,
                            meta: meta,
                        }))
                    },
                    Err(e) => Err(e),
                }

            },
            Err(e) => Err(e),
        }
    }

    fn to_json(&self) -> String{
        let json = serde_json::to_string(&self);
        match json {
            Ok(v) => v,
            Err(_) => String::from("")
        }
    }
}


impl Pip2001{
    fn verify_fields(&self, data: &HashMap<String, String>, meta: &HashMap<String, pip::InputObject>) -> std::result::Result<Pip2001MessageType, &'static str> {
        println!("fn verify_fields");
        //let data_exist = data.contains_key("allow");
        if data.contains_key("allow") || data.contains_key("deny") {
            if data.contains_key("topic") {
                Ok(Pip2001MessageType::PUBLISH_MANAGEMENT)
            } else {
                Err("topic fields must exist")
            }
        } else {
            if data.contains_key("file_hash") && data.contains_key("topic") {
                if meta.contains_key("uris") {
                    let uris = meta.get("uris").unwrap();
                    match uris {
                     pip::InputObject::String(_s) => {
                        Err("urls should be a url list")

                     },
                     pip::InputObject::VecOfString(_v) => {
                        Ok(Pip2001MessageType::PUBLISH)
                     }
                    }
                    //println!("uris: {:?} ", uris);
                } else {
                    Err("meta.urls must exist")
                }

            } else {
                Err("file_hash, topic fields must exist.")
            }
        }
    }

    fn verify_object_format(&self, message_type: Pip2001MessageType, data: &HashMap<String, String>, meta: &HashMap<String, pip::InputObject>) -> std::result::Result<Pip2001MessageType, &'static str> {
        match message_type{
            Pip2001MessageType::PUBLISH_MANAGEMENT => {
                let mut datatype = data.get("allow");
                if datatype == None  {
                    datatype = data.get("deny");
                }

                let pub_addrs = datatype.unwrap().split(",");
                let mut pubaddrs_correct = true;
                for pub_addr in pub_addrs{
                    if !is_pub_address(pub_addr) {
                        pubaddrs_correct=false;
                    }
                }

                if pubaddrs_correct == false {
                    Err("public address error in the data object")
                } else {
                    Ok(message_type)
                }
            },
            Pip2001MessageType::PUBLISH => {

                let file_hash = data.get("file_hash").unwrap();
                let topic = data.get("topic").unwrap();
                let uris_len;
                let uris = meta.get("uris").unwrap();
                match uris {
                 pip::InputObject::String(_s) => {
                    uris_len = 0;
                 },
                 pip::InputObject::VecOfString(v) => {
                    uris_len = v.len();
                 }
                }

                if !is_hash_hex(file_hash) {
                    Err("file_hash error in the data object")
                } else if !is_pub_address(topic) {
                    Err("topic format error in the data object")
                } else if uris_len == 0 {
                    Err("at least one uri was required.")
                } else {
                    Ok(message_type)
                }
            },
            Pip2001MessageType::NA=> {
                    Ok(message_type)
            }
        }
    }
}
