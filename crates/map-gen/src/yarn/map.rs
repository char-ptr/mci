use std::{
    borrow::Cow, cell::RefCell, collections::HashMap, fmt::Debug, fs::File, io::Read,
    iter::Peekable, path::PathBuf, rc::Rc, str::Chars, sync::Arc,
};

use logos::{Lexer, Logos};
use parking_lot::RwLock;

#[derive(Debug)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) scope: Vec<ModuleOrClass>,
}
#[derive(Debug)]
pub struct Method {
    pub(crate) map_data: Mapping,
    pub(crate) arguments: Vec<Mapping>,
    pub(crate) type_signature: Signatures,
}
#[derive(Debug)]
pub struct Field {
    pub(crate) map_data: Mapping,
    pub(crate) type_signature: Signatures,
}
pub struct Class {
    pub(crate) map_data: Mapping,
    pub(crate) methods: Vec<Method>,
    pub(crate) fields: Vec<Field>,
    pub(crate) inner_classes: Vec<Class>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {{\n\tfields : {:#?}\n\tmethods : {:#?}\n\tinner_classes : {:#?}}}",
            self.map_data.to, self.fields, self.methods, self.inner_classes
        ))
    }
}

#[derive(Debug)]

pub struct Mapping {
    pub from: String,
    pub to: String,
}
#[derive(Debug)]

pub enum ModuleOrClass {
    Module(Module),
    Class(Class),
}
#[derive(Debug, Clone)]

pub enum Signatures {
    MethodSig {
        arguments: Vec<SigPart>,
        ret: SigPart,
    },
    FieldSig(SigPart),
}
#[derive(Debug, Clone)]
pub enum SigPart {
    WithArray {
        depth: usize,
        content: SigContentTypes,
    },
    Normal(SigContentTypes),
}

impl SigPart {
    pub fn new_with_depth(depth: usize, content: SigContentTypes) -> Self {
        if depth == 0 {
            Self::Normal(content)
        } else {
            Self::WithArray { depth, content }
        }
    }
    pub fn get_content(&self) -> &SigContentTypes {
        match self {
            SigPart::Normal(c) => c,
            SigPart::WithArray { content, .. } => content,
        }
    }
    pub fn is_class(&self) -> bool {
        match self {
            SigPart::Normal(SigContentTypes::Class(_)) => true,
            SigPart::WithArray { content, .. } => match content {
                SigContentTypes::Class(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            SigPart::Normal(_) => false,
            SigPart::WithArray { .. } => true,
        }
    }
}
impl Signatures {
    pub fn unwrap_field(self) -> SigPart {
        match self {
            Signatures::FieldSig(s) => s,
            _ => panic!("Tried to unwrap a method signature as a field signature"),
        }
    }
    pub fn unwrap_method(self) -> (Vec<SigPart>, SigPart) {
        match self {
            Signatures::MethodSig { arguments, ret } => (arguments, ret),
            _ => panic!("Tried to unwrap a field signature as a method signature"),
        }
    }
    pub fn to_java(&self) -> String {
        match self {
            Signatures::MethodSig { arguments, ret } => {
                let mut args = String::new();
                for arg in arguments {
                    args.push_str(&arg.to_java());
                }
                format!("({}){}", args, ret.to_java())
            }
            Signatures::FieldSig(s) => s.to_java(),
        }
    }
    pub fn to_javas(&self) -> String {
        match self {
            Signatures::MethodSig { arguments, ret } => {
                let mut args = String::new();
                for arg in arguments {
                    args.push_str(&arg.to_java_safer());
                }
                format!("{}{}", args, ret.to_java_safer())
            }
            Signatures::FieldSig(s) => s.to_java_safer(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SigContentTypes {
    Class(String),
    Bool,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void,
}
impl SigContentTypes {
    pub fn is_class(&self) -> bool {
        match self {
            SigContentTypes::Class(_) => true,
            _ => false,
        }
    }
    pub fn to_rust(&self) -> String {
        match self {
            SigContentTypes::Class(s) => s.to_string(),
            SigContentTypes::Bool => "bool".to_string(),
            SigContentTypes::Byte => "i8".to_string(),
            SigContentTypes::Char => "char".to_string(),
            SigContentTypes::Short => "i16".to_string(),
            SigContentTypes::Int => "i32".to_string(),
            SigContentTypes::Long => "i64".to_string(),
            SigContentTypes::Float => "f32".to_string(),
            SigContentTypes::Double => "f64".to_string(),
            SigContentTypes::Void => "()".to_string(),
        }
    }
    pub fn to_java(&self) -> String {
        match self {
            SigContentTypes::Class(s) => format!("L{};", s),
            SigContentTypes::Bool => "Z".to_string(),
            SigContentTypes::Byte => "B".to_string(),
            SigContentTypes::Char => "C".to_string(),
            SigContentTypes::Short => "S".to_string(),
            SigContentTypes::Int => "I".to_string(),
            SigContentTypes::Long => "J".to_string(),
            SigContentTypes::Float => "F".to_string(),
            SigContentTypes::Double => "D".to_string(),
            SigContentTypes::Void => "V".to_string(),
        }
    }
    pub fn to_java_name(&self) -> String {
        match self {
            SigContentTypes::Class(s) => s.to_string(),
            SigContentTypes::Bool => "boolean".to_string(),
            SigContentTypes::Byte => "byte".to_string(),
            SigContentTypes::Char => "char".to_string(),
            SigContentTypes::Short => "short".to_string(),
            SigContentTypes::Int => "int".to_string(),
            SigContentTypes::Long => "long".to_string(),
            SigContentTypes::Float => "float".to_string(),
            SigContentTypes::Double => "double".to_string(),
            SigContentTypes::Void => "void".to_string(),
        }
    }
}
impl SigPart {
    pub fn to_rust(&self) -> String {
        match self {
            SigPart::Normal(s) => s.to_rust(),
            SigPart::WithArray { depth, content } => {
                let s = content.to_rust();
                format!("{}{s}{}", "JArray<".repeat(*depth), ">".repeat(*depth))
            }
        }
    }
    pub fn to_rust_life(&self) -> String {
        match self {
            SigPart::Normal(s) => s.to_rust(),
            SigPart::WithArray { depth, content } => {
                let s = content.to_rust();
                format!("{}{s}{}", "JArray<'a,".repeat(*depth), ">".repeat(*depth))
            }
        }
    }
    pub fn to_rust_custom(&self, cus: &str) -> String {
        match self {
            SigPart::Normal(s) => {
                if s.is_class() {
                    cus.to_string()
                } else {
                    s.to_rust()
                }
            }
            SigPart::WithArray { depth, content } => {
                format!(
                    "{}{}{}",
                    "JArray<".repeat(*depth),
                    if content.is_class() {
                        cus.to_string()
                    } else {
                        content.to_rust()
                    },
                    ">".repeat(*depth)
                )
            }
        }
    }
    pub fn to_rust_custom_life(&self, cus: &str) -> String {
        match self {
            SigPart::Normal(s) => {
                if s.is_class() {
                    cus.to_string()
                } else {
                    s.to_rust()
                }
            }
            SigPart::WithArray { depth, content } => {
                format!(
                    "{}{}{}",
                    "JArray<'a,".repeat(*depth),
                    if content.is_class() {
                        cus.to_string()
                    } else {
                        content.to_rust()
                    },
                    ">".repeat(*depth)
                )
            }
        }
    }
    pub fn to_rust_no_array(&self) -> String {
        match self {
            SigPart::Normal(s) => s.to_rust(),
            SigPart::WithArray { depth, content } => content.to_rust(),
        }
    }
    pub fn to_java(&self) -> String {
        match self {
            SigPart::Normal(s) => s.to_java(),
            SigPart::WithArray { depth, content } => {
                let s = content.to_java();
                format!("{}{s}", "[".repeat(*depth))
            }
        }
    }
    pub fn to_java_safer(&self) -> String {
        match self {
            SigPart::Normal(s) => {
                let s1 = s.to_java_name();
                let s = &s1[s1.len() - 2..];
                s.to_string()
            }
            SigPart::WithArray { depth, content } => {
                let s1 = &content.to_java_name();
                let s = &s1[s1.len() - 2..];
                format!("{}{s}", "a".repeat(*depth))
            }
        }
    }
}

impl Signatures {
    pub fn parse_from_str(sig: &str) -> Self {
        let mut itr = sig.chars().peekable();
        if itr
            .peek()
            .and_then(|x| if *x == '(' { Some(x) } else { None })
            .is_some()
        {
            itr.next();
            let a = Self::parse_method(&mut itr);
            a
        } else {
            Signatures::FieldSig(Self::parse_content(&mut itr))
        }
    }
    fn parse_method<'a>(itr: &mut Peekable<Chars<'a>>) -> Self {
        let mut parts = Vec::new();
        while itr.peek() != Some(&')') {
            let content = Self::parse_content(itr);
            parts.push(content);
        }
        itr.next();
        let ret = Self::parse_content(itr);

        Signatures::MethodSig {
            arguments: parts,
            ret: ret,
        }
    }
    fn parse_content<'a>(itr: &mut Peekable<Chars<'a>>) -> SigPart {
        let mut depth = 0;
        // let mut depth = if itr.peek().and_then(|x| if *x == '[' {Some(x)} else {None}).is_some() {itr.take_while(|x| *x == '[').count()} else {0};
        while itr.peek().is_some_and(|x| *x == '[') {
            itr.next();
            depth += 1;
        }
        if itr
            .peek()
            .and_then(|x| {
                if x.to_ascii_uppercase() == 'L' {
                    Some(x)
                } else {
                    None
                }
            })
            .is_some()
        {
            itr.next();
            let clz = itr.take_while(|x| *x != ';').collect::<String>();
            return SigPart::new_with_depth(depth, SigContentTypes::Class(clz));
        } else {
            let nc = itr.next().and_then(|x| Some(x.to_ascii_uppercase()));
            match nc {
                Some('B') => return SigPart::new_with_depth(depth, SigContentTypes::Byte),
                Some('C') => return SigPart::new_with_depth(depth, SigContentTypes::Char),
                Some('D') => return SigPart::new_with_depth(depth, SigContentTypes::Double),
                Some('F') => return SigPart::new_with_depth(depth, SigContentTypes::Float),
                Some('I') => return SigPart::new_with_depth(depth, SigContentTypes::Int),
                Some('J') => return SigPart::new_with_depth(depth, SigContentTypes::Long),
                Some('S') => return SigPart::new_with_depth(depth, SigContentTypes::Short),
                Some('Z') => return SigPart::new_with_depth(depth, SigContentTypes::Bool),
                Some('V') => return SigPart::new_with_depth(depth, SigContentTypes::Void),
                _ => panic!("Unknown content type: {:?}", nc),
            }
        }
    }
}

#[derive(Debug)]

pub struct Yarn {
    pub(crate) modules: Vec<Module>,
    pub(crate) lookup: HashMap<String, String>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            scope: Vec::new(),
        }
    }
}

impl Mapping {
    pub fn get_safe_name(&self) -> String {
        let mut name = self.to.trim().to_string();
        if name.is_empty() {
            name = {
                let mut name = self.from.clone();
                let mut name = name.split('/').last().unwrap().to_string();
                name.retain(|c| c.is_ascii_alphanumeric() || c == '_');
                name
            };
            println!("Warning: {} has no name, using {} instead", self.from, name);
        }
        let mut name = name.split('/').last().unwrap().to_string();
        name.retain(|c| c.is_alphanumeric() || c == '_');
        name
    }
}
impl ModuleOrClass {
    pub fn get_name(&self) -> &str {
        match self {
            ModuleOrClass::Module(m) => m.name.as_ref(),

            ModuleOrClass::Class(c) => c.map_data.to.as_ref(),
        }
    }
}

impl From<Module> for ModuleOrClass {
    fn from(m: Module) -> Self {
        ModuleOrClass::Module(m)
    }
}
impl From<Class> for ModuleOrClass {
    fn from(m: Class) -> Self {
        ModuleOrClass::Class(m)
    }
}

impl Class {
    pub fn from_token(token: &ClassIdToken) -> Self {
        Self {
            fields: Vec::new(),
            methods: Vec::new(),
            map_data: Mapping {
                from: token.0.clone(),
                to: token.1.clone(),
            },
            inner_classes: Vec::new(),
        }
    }
}
impl Method {
    pub fn from_token(token: &MethodToken) -> Self {
        if token.2.is_empty() {
            Self {
                arguments: Vec::new(),
                map_data: Mapping {
                    from: token.0.clone(),
                    to: "".to_string(),
                },
                type_signature: Signatures::parse_from_str(&token.1),
            }
        } else {
            Self {
                arguments: Vec::new(),
                map_data: Mapping {
                    from: token.0.clone(),
                    to: token.1.clone(),
                },
                type_signature: Signatures::parse_from_str(&token.2),
            }
        }
    }
}
impl Field {
    pub fn from_token(token: &FieldToken) -> Self {
        Self {
            map_data: Mapping {
                from: token.0.clone(),
                to: token.1.clone(),
            },
            type_signature: Signatures::parse_from_str(&token.2),
        }
    }
}

pub fn parse_arg(lex: &mut Lexer<YarnTokens>) -> Result<ArgumentToken, ()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;
    let arg_num: usize = spaces.next().unwrap().parse().or(Err(()))?;
    let arg_name = spaces.next().unwrap_or("");

    Ok(ArgumentToken(arg_num, arg_name.to_string()))
}
pub fn parse_class_id(lex: &mut Lexer<YarnTokens>) -> Result<ClassIdToken, ()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;
    let class_id = spaces.next().ok_or(())?;
    let class_name = spaces.next().unwrap_or("");

    Ok(ClassIdToken(class_id.to_string(), class_name.to_string()))
}
pub fn parse_method(lex: &mut Lexer<YarnTokens>) -> Result<MethodToken, ()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;

    let method_id = spaces.next().ok_or(())?;
    let method_name = spaces.next().unwrap_or("");
    let method_sig = spaces.next().unwrap_or("");

    Ok(MethodToken(
        method_id.to_string(),
        method_name.to_string(),
        method_sig.to_string(),
    ))
}
pub fn parse_field(lex: &mut Lexer<YarnTokens>) -> Result<FieldToken, ()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;
    let field_id = spaces.next().ok_or(())?;
    let field_name = spaces.next().unwrap_or("");
    let field_sig = spaces.next().unwrap_or("");

    Ok(FieldToken(
        field_id.to_string(),
        field_name.to_string(),
        field_sig.to_string(),
    ))
}
#[derive(Debug, PartialEq)]
pub struct ArgumentToken(usize, String);

#[derive(Debug, PartialEq)]
pub struct ClassIdToken(String, String);
#[derive(Debug, PartialEq)]
pub struct MethodToken(String, String, String);
#[derive(Debug, PartialEq)]
pub struct FieldToken(String, String, String);

#[derive(Logos, Debug, PartialEq)]
pub enum YarnTokens {
    #[regex(r#"CLASS [a-zA-z0-9/_$]+ ?[a-zA-z0-9/_$]*"#,callback = parse_class_id)]
    Class(ClassIdToken),
    #[regex(r#"CLASS class_\d+ \S+"#, callback = parse_class_id)]
    ClassId(ClassIdToken),
    #[regex(r#"METHOD (method_\d+ ?)?( ?\S+ ?)? ?\(((\[*)([ZBCSIJFD]|L[A-Za-z0-9/_$]+;))*\)(\[*)([ZBCSIJFDV]|L[A-Za-z0-9/_$]+;)"#, callback = parse_method)]
    Method(MethodToken),
    #[regex(r#"ARG \d( \S+)?"#, callback = parse_arg )]
    Arg(ArgumentToken),
    #[regex(r#"FIELD field_\d+ (\S+ )?(\[*)([ZBCSIJFD]|L[A-Za-z0-9/_$]+;)"#, callback = parse_field)]
    Field(FieldToken),
    #[regex(r"\t+", callback = |lex| lex.slice().chars().count())]
    Tab(usize),

    #[regex("\n")]
    NewLine,

    #[regex(r#"COMMENT.*"#, |lex| lex.slice()[7..].to_string())]
    Comment(String),
    #[regex(r#"(method|field|class)_\d+"#, callback= |lex| lex.slice().to_string())]
    Identifier(String),
    // #[error]
    #[regex(r"[ \r\f]+", logos::skip)]
    Error,
}

#[derive(Debug)]
pub enum YarnParseError {
    RootClassNotFound,
    LexingError,
    ImpossibleArgument,
}

enum OwnedOrBorrowed<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}
impl<'a, T> OwnedOrBorrowed<'a, T> {
    fn to_borrowed(&self) -> &T {
        match &self {
            OwnedOrBorrowed::Owned(t) => t,
            OwnedOrBorrowed::Borrowed(t) => t,
        }
    }
}
impl Yarn {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn resolve_sig(sig: &str) {}

    pub fn run_str(&mut self, s: &str) -> Result<Class, YarnParseError> {
        let tokens = YarnTokens::lexer(&s);

        for token in tokens {
            let Ok(token) = token else {
                return Err(YarnParseError::LexingError);
            };
            match token {
                YarnTokens::Class(_) => todo!(),
                YarnTokens::ClassId(_) => todo!(),
                YarnTokens::Method(_) => todo!(),
                YarnTokens::Arg(_) => todo!(),
                YarnTokens::Field(_) => todo!(),
                YarnTokens::Tab(_) => todo!(),
                YarnTokens::NewLine => todo!(),
                YarnTokens::Comment(_) => todo!(),
                YarnTokens::Identifier(_) => todo!(),
                YarnTokens::Error => todo!(),
            }
        }
    }

    pub fn run_file(&mut self, path: &PathBuf) -> Result<Class, YarnParseError> {
        let mut file = File::open(path).unwrap();
        let mut fstr = String::new();
        file.read_to_string(&mut fstr);
        self.run_str(&fstr)
    }
    pub fn run_directory(&mut self, path: PathBuf) -> Result<Module, YarnParseError> {
        let mut new_mod = Module::new(path.file_name().unwrap().to_str().unwrap().to_string());
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let module_out = self.run_directory(path.clone());
                match module_out {
                    Ok(x) => new_mod.scope.push(ModuleOrClass::Module(x)),
                    Err(r) => {
                        println!("{} failed = {:?}", path.display(), r);
                    }
                }
            } else {
                match self.run_file(&path) {
                    Ok(x) => {
                        println!("{} success", &path.display());
                        new_mod.scope.push(ModuleOrClass::Class(x));
                    }
                    Err(r) => {
                        println!("{} failed = {:?}", &path.display(), r);
                    }
                };
            }
        }
        Ok(new_mod)
    }
}

mod tests {
    use std::{fs::File, io::Write, path::PathBuf};

    use super::Yarn;
    // #[test]
    // fn test_lexer() {
    //     let mut yarn_instance = Yarn::new();

    //     yarn_instance.run_str(include_str!("../../../mc-mappings/mappings/mappings/net/minecraft/advancement/Advancement.mapping")).expect("bruh");
    // }

    #[test]
    fn test_dir_run() {
        let mut yarn_instance = Yarn::new();
        if !std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .ends_with("gen")
        {
            std::env::set_current_dir("./crates/map-gen").unwrap();
        }
        println!(
            "env = {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        );

        yarn_instance.run_directory(PathBuf::from("../../mappings/yarn-maps/mappings/net"));
        println!("done");
        let mut wf = File::create("test.ron").unwrap();
        let data = format!("{:#?}", yarn_instance.modules);
        println!("got data");
        println!("lookup table = {:#?}", yarn_instance.lookup);
        write!(wf, "{}", data).unwrap();
        // println!("{:#?}",yarn_instance.modules);
    }
}
