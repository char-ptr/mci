use std::{collections::HashMap, path::PathBuf, fs::File, io::Read, rc::Rc, cell::RefCell, sync::Arc};

use logos::{Logos, Lexer};
use parking_lot::RwLock;

#[derive(Debug)]
pub struct Module{
    name: String,
    scope: Vec<ModuleOrClass>
} 
#[derive(Debug)]
pub struct Method {
    map_data : Mapping,
    arguments: Vec<Mapping>,
    type_signature: String
}
#[derive(Debug)]
pub struct Field {
    map_data : Mapping,
    type_signature: String
}
#[derive(Debug)]
pub struct Class {
    map_data : Mapping,
    methods: Vec<Method>,
    fields: Vec<Field>,
    inner_classes: Vec<Arc<RwLock<Class>>>,
}
#[derive(Debug)]

pub struct Mapping{pub from: String, pub to: String}
#[derive(Debug)]

pub enum ModuleOrClass {
    Module(Module),
    Class(Class)
}
#[derive(Debug)]

pub struct Signature {

}
#[derive(Debug)]

pub struct Yarn<'a> {
    modules: Vec<Module>,
    lookup: HashMap<String, &'a ModuleOrClass>
}


// const GENERAL_TYPE_MATCHER : &'static str = r#"(\[*)([ZBCSIJFD]|L[A-Za-z0-9/_$]+)"#;
// const RETURN_TYPE_MATCHER : &'static str = r#"\(((\[*)([ZBCSIJFD]|L[A-Za-z0-9/_$]+);)+\)(\[*)([ZBCSIJFDV]|L[A-Za-z0-9/_$]+)"#;


impl Class {
    pub fn from_token(token : &ClassIdToken) -> Self {
        Self{
            fields: Vec::new(),
            methods: Vec::new(),
            map_data: Mapping{
                from: token.0.clone(),
                to: token.1.clone()
            },
            inner_classes: Vec::new()
        }
    }
}
impl Method {
    pub fn from_token(token : &MethodToken) -> Self {
        Self{
            arguments: Vec::new(),
            map_data: Mapping{
                from: token.0.clone(),
                to: token.1.clone()
            },
            type_signature: token.2.clone()
        }
    }
}
impl Field {
    pub fn from_token(token : &FieldToken) -> Self {
        Self{
            map_data: Mapping{
                from: token.0.clone(),
                to: token.1.clone()
            },
            type_signature: token.2.clone()
        }
    }
}

pub fn parse_arg(lex: &mut Lexer<YarnTokens>) -> Result<ArgumentToken,()> {
    let mut spaces = lex.slice().split(' ');


    let _ = spaces.next().ok_or(())?;
    let arg_num: usize = spaces.next().unwrap().parse().or(Err(()))?;
    let arg_name = spaces.next().unwrap_or("");


    Ok(ArgumentToken(arg_num,arg_name.to_string()))
}
pub fn parse_class_id(lex: &mut Lexer<YarnTokens>) -> Result<ClassIdToken,()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;
    let class_id = spaces.next().ok_or(())?;
    let class_name = spaces.next().unwrap_or("");


    Ok(ClassIdToken(class_id.to_string(),class_name.to_string()))
}
pub fn parse_method(lex: &mut Lexer<YarnTokens>) -> Result<MethodToken,()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;

    let method_id = spaces.next().ok_or(())?;
    let method_name = spaces.next().unwrap_or("");
    let method_sig = spaces.next().unwrap_or("");


    Ok(MethodToken(method_id.to_string(),method_name.to_string(),method_sig.to_string()))
}
pub fn parse_field(lex: &mut Lexer<YarnTokens>) -> Result<FieldToken,()> {
    let mut spaces = lex.slice().split(' ');

    let _ = spaces.next().ok_or(())?;
    let field_id = spaces.next().ok_or(())?;
    let field_name = spaces.next().unwrap_or("");
    let field_sig = spaces.next().unwrap_or("");


    Ok(FieldToken(field_id.to_string(),field_name.to_string(),field_sig.to_string()))
}
#[derive(Debug,PartialEq)]
pub struct ArgumentToken(usize,String);

#[derive(Debug,PartialEq)]
pub struct ClassIdToken(String,String);
#[derive(Debug,PartialEq)]
pub struct MethodToken(String,String,String);
#[derive(Debug,PartialEq)]
pub struct FieldToken(String,String,String);

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
    #[error]
    #[regex(r"[ \r\f]+",logos::skip)]
    Error,
}

#[derive(Debug)]
pub enum YarnParseError {
    RootClassNotFound,
    LexingError
}

impl<'a> Yarn<'a> {

    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            lookup: HashMap::new()
        }
    }

    pub fn run_str(&mut self,s :&str) -> Result<(),YarnParseError> {
        let tokens = YarnTokens::lexer(&s);

        let mut tokens = tokens.peekable();

        let mut root_class = Arc::new(RwLock::new(match tokens.peek() {
            Some(token) => {
                match token {
                    YarnTokens::Class(id) => Class::from_token(id),
                    _ => return Err(YarnParseError::RootClassNotFound)
                }
            },
            None => return Err(YarnParseError::LexingError),
        }));

        let mut stack = vec![root_class.clone()];

        tokens.next();
        let last_token = 0;

        for tok in tokens {
            match tok {
                YarnTokens::Class(x) => {
                    let clzz = Class::from_token(&x);
                    stack.last_mut().unwrap().write().inner_classes.push(Arc::new(RwLock::new(clzz)));
                },
                YarnTokens::ClassId(x) => {
                    let clzz = Class::from_token(&x);
                    stack.last_mut().unwrap().write().inner_classes.push(Arc::new(RwLock::new(clzz)));

                },
                YarnTokens::Method(m) => {
                    stack.last_mut().unwrap().write().methods.push(Method::from_token(&m));
                },
                YarnTokens::Arg(a) => {
                    stack.last_mut().unwrap().write().methods.last_mut().unwrap().arguments.push(Mapping { from: a.0.to_string(), to: a.1 });
                },
                YarnTokens::Field(f) => {
                    stack.last_mut().unwrap().write().fields.push(Field::from_token(&f));
                },
                YarnTokens::Tab(new) => {
                    let stk = &mut stack;
                    if new > last_token {
                        let last = {
                            let q = stk.last().unwrap();
                            match q.write().inner_classes.last(){
                                Some(x) => x,
                                None => &q
                            }.clone()
                        };
                        stk.push(last);
                    } else {
                        for _ in 0..(last_token - new) {
                            stk.pop();
                        }
                    }
                },
                // YarnTokens::NewLine => todo!(),
                // YarnTokens::Comment(_) => todo!(),
                // YarnTokens::Identifier(_) => todo!(),
                _ => {

                }
                YarnTokens::Error => return Err(YarnParseError::LexingError),
            }
        }

        println!("{:#?}",root_class);

        Ok(())
    }

    pub fn run_file(&mut self,path:PathBuf) {
        let mut file = File::open(path).unwrap();
        let mut fstr = String::new();
        file.read_to_string(&mut fstr);
        
        self.run_str(&fstr);
    }

}


#[test]
fn test_lexer() {
    let mut yarn_instance = Yarn::new();

    yarn_instance.run_str(include_str!("../../mc-mappings/mappings/mappings/net/minecraft/advancement/Advancement.mapping")).expect("bruh");


}