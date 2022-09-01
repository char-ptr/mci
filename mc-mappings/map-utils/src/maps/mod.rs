
pub mod tiny;
pub mod yarn;

use std::{collections::{HashSet, HashMap}, borrow::Cow, str::FromStr, fmt::Display};
#[derive(Debug,Hash,PartialEq, Eq,Clone)]

pub struct SigClass {
    pub from: String,
    pub to: String,
    pub module_stack: Vec<String>,
    pub fields: Vec<SigField>,
    pub methods: Vec<SigMethod>,
}
#[derive(Debug,Hash,PartialEq, Eq,Clone)]

pub struct SigField {
    pub from: String,
    pub to: String,
    pub parent : String,
    pub type_ : String,
}
#[derive(Debug,Hash,PartialEq, Eq,Clone)]

pub struct SigMethod {
    pub from: String,
    pub to: String,
    pub parent : String,
    pub type_ : String,
    pub args: Vec<String>,
}
#[derive(Debug,Hash,PartialEq, Eq)]
pub enum SigType<'a> {
    Class(Cow<'a,SigClass>),
    Field(Cow<'a,SigField>),
    Method(Cow<'a,SigMethod>),
}

impl SigType<'_> {
    pub fn get_from(&self) -> &str {
        match self {
            SigType::Class(c) => &c.from,
            SigType::Field(f) => &f.from,
            SigType::Method(m) => &m.from,
        }
    }
    pub fn get_to(&self) -> &str {
        match self {
            SigType::Class(c) => &c.to,
            SigType::Field(f) => &f.to,
            SigType::Method(m) => &m.to,
        }
    }
}
#[derive(Debug,Hash,Clone)]
pub enum SigMod<'a> {
    Mod(String,Vec<SigMod<'a>>),
    class(Cow<'a,SigClass>)
}
impl Display for SigMod<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigMod::Mod(n, _) => f.write_str(format!("module:{}",n).as_str()),
            SigMod::class(c) => f.write_str(format!("class:{}",c.to).as_str()),
        }
    }
}

impl Default for SigMod<'_> {
    fn default() -> Self {
        SigMod::Mod(String::from_str("root").unwrap(),Vec::new())
    }
}

#[derive(Debug,Default)]
pub struct SigMappings<'a> {
    pub classes: Vec<Cow<'a,SigClass>>,
    pub mods : SigMod<'a>,
    pub sig_to_x: HashMap<String, SigType<'a>>
} 