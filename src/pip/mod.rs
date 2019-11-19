use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq)]
pub enum PipId {
    PIP1001,
    PIP2001,
}

impl fmt::Debug for PipId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PipId::PIP1001 => write!(f, "PIP:1001"),
            PipId::PIP2001 => write!(f, "PIP:2001"),
        }
    }
}

impl Serialize for PipId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = match *self {
            PipId::PIP1001 => "PIP:1001",
            PipId::PIP2001 => "PIP:2001",
        };
        serializer.serialize_str(name)
    }
}

pub enum InputObject {
    String(String),
    VecOfString(Vec<String>),
}

impl Serialize for InputObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InputObject::String(v) => serializer.serialize_str(v),
            InputObject::VecOfString(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

impl fmt::Debug for InputObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputObject::String(v) => write!(f, "{}", v),
            InputObject::VecOfString(v) => {
                let mut outputstr = String::new();
                for item in v {
                    outputstr.push_str(&item.to_string());
                    outputstr.push_str(", ");
                }
                write!(f, "{}", outputstr)
            }
        }
    }
}

pub trait Pip {
    fn pip_id(&self) -> PipId;
    fn validate(&self) -> bool;
    //fn new(data: &HashMap<&str, &str> ) -> Self;
    fn new() -> Self;
    fn from_dict(
        &mut self,
        data: HashMap<String, String>,
        meta: HashMap<String, InputObject>,
    ) -> std::result::Result<Option<Self>, &'static str>
    where
        Self: Sized;
    fn from_json(&mut self, jsonstr: &str) -> std::result::Result<Option<Self>, &'static str>
    where
        Self: Sized;
    fn to_json(&self) -> String;
}

pub mod pip2001;
