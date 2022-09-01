pub struct CLPType {
    pub object_type: CLPExpectedType,
    pub name: String
}

impl std::fmt::Debug for CLPType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}({})", self.object_type, self.name)
    }
}

pub enum CLPExpectedType {
    INT,
    UINT,
    FLOAT,
    STRING,
    VECINT,
    VECUINT,
    VECFLOAT,
    VECSTRING
}

impl std::fmt::Debug for CLPExpectedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CLPExpectedType::INT => write!(f, "INT"),
            CLPExpectedType::UINT => write!(f, "UINT"),
            CLPExpectedType::FLOAT => write!(f, "FLOAT"),
            CLPExpectedType::STRING => write!(f, "STRING"),
            CLPExpectedType::VECINT => write!(f, "[...INT...]"),
            CLPExpectedType::VECUINT => write!(f, "[...UINT...]"),
            CLPExpectedType::VECFLOAT => write!(f, "[...FLOAT...]"),
            CLPExpectedType::VECSTRING => write!(f, "[...STRING...]"),
        }
    }
}

#[derive(Debug)]
pub enum CLPOutputType {
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
pub enum CLPErrorKind {
    Error(String),
    ParseError(String),
    KwargError(String),
    ArgsError(String)
}