use std::{collections::HashMap, io::BufRead, sync::Arc};
use parking_lot as pl;


#[derive(Debug, Clone)]
pub struct Tiny {
    pub(crate) lookup:  Arc<pl::RwLock<HashMap<String, LookupType>>>
}

#[derive(Debug, Clone)]

pub enum LookupType {
    Class(Class),
    Method(Method),
    Field(Field),
}
#[derive(Debug, Clone)]

pub struct Class {
    pub(crate) from: String,
    pub(crate) to: String,
}
#[derive(Debug, Clone)]

pub struct Method {
    pub(crate) parent: String,
    pub(crate) Signature: String,
    pub(crate) Obfuscated: String,
    pub(crate) Yarn: String,
}
#[derive(Debug, Clone)]

pub struct Field {
    pub(crate) parent: String,
    pub(crate) Signature: String,
    pub(crate) Obfuscated: String,
    pub(crate) Yarn: String,
}

impl LookupType {
    pub fn get_obfuscated(&self) -> String {
        match self {
            LookupType::Class(class) => class.from.clone(),
            LookupType::Method(method) => method.Obfuscated.clone(),
            LookupType::Field(field) => field.Obfuscated.clone(),
        }
    }
}


impl Tiny {
    pub fn new() -> Self {
        Self {
            lookup: Default::default(),
        }
    }
    pub fn populate_from_reader<T: BufRead>(&self, reader :T) {
        let lines = reader.lines();

        let lookup = self.lookup.clone();

        for line in lines {
            if let Ok(line) = line {

                let mut data = line.split_whitespace();
                let cmd = data.nth(0).unwrap_or("");
    
                match cmd {
    
                    "CLASS" => {
                        let obf = data.nth(0).unwrap_or("");
                        let deobf = data.nth(0).unwrap_or("");
                        let sc = Class {
                            from: obf.to_string(),
                            to: deobf.to_string(),
                        };
                        lookup.write().insert(format!("{}_c",deobf), LookupType::Class(sc));
                    }
                    "METHOD" => {
                        let class_obf = data.nth(0).unwrap_or("");
                        let sig = data.nth(0).unwrap_or("");
                        let obf = data.nth(0).unwrap_or("");
                        let id = data.nth(0).unwrap_or("");
                        let sm = Method{
                            Obfuscated: obf.to_string(),
                            Yarn: id.to_string(),
                            Signature: sig.to_string(),
                            parent: class_obf.to_string(),
                        };
                        lookup.write().insert(format!("{}_m",id), LookupType::Method(sm));
                    }
                    "FIELD" => {
                        let class_obf = data.nth(0).unwrap_or("");
                        let sig = data.nth(0).unwrap_or("");
                        let obf = data.nth(0).unwrap_or("");
                        let id = data.nth(0).unwrap_or("");
    
                        let sf = Field{
                            Obfuscated: obf.to_string(),
                            Yarn: id.to_string(),
                            Signature: sig.to_string(),
                            parent: class_obf.to_string(),
                        };
                        lookup.write().insert(format!("{}_f",id), LookupType::Field(sf));
                    }
    
                    _=>{}
                }
            }
        }
    }
}