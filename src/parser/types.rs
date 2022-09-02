use std::collections::HashMap;


//arguments
pub enum ArgTypes {
    INT,
    UINT,
    FLOAT,
    STRING,
    VECINT,
    VECUINT,
    VECFLOAT,
    VECSTRING
}

pub enum ArgsSettings {
    NONE,
    ALL(CLPInput),
    Args(Vec<CLPInput>)
}

impl std::fmt::Debug for ArgTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgTypes::INT => write!(f, "INT"),
            ArgTypes::UINT => write!(f, "UINT"),
            ArgTypes::FLOAT => write!(f, "FLOAT"),
            ArgTypes::STRING => write!(f, "STRING"),
            ArgTypes::VECINT => write!(f, "[...INT...]"),
            ArgTypes::VECUINT => write!(f, "[...UINT...]"),
            ArgTypes::VECFLOAT => write!(f, "[...FLOAT...]"),
            ArgTypes::VECSTRING => write!(f, "[...STRING...]"),
        }
    }
}

//kwargs
pub struct KwargSettings {
    pub keyvalues: HashMap<String, KwargTypes>
}

pub enum KwargTypes {
    Important([String; 3]),
    Optional([String; 3]),
    Toggle([String; 1])
}

//Result and input
pub struct CLPInput {
    pub object_type: ArgTypes,
    pub name: String
}

impl std::fmt::Debug for CLPInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}({})", self.object_type, self.name)
    }
}

#[derive(Debug)]
pub enum CLPVar {
    Int(i128),
    UInt(u128),
    Float(f64),
    String(String),
    VecInt(Vec<i128>),
    VecUInt(Vec<u128>),
    VecFloat(Vec<f64>),
    VecString(Vec<String>)
}

#[derive(Debug)]
pub struct CLPResult {
    pub arg0: String,
    pub default: Vec<String>,
    pub args: HashMap<String, CLPVar>
}

//Error Kind
#[derive(Debug)]
pub enum CLPErrorKind {
    Error(String),
    ParseError(String),
    KwargError(String),
    ArgsError(String)
}