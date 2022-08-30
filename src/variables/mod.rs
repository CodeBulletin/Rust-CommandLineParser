pub enum CLPExpectedType {
    INT(String),
    UINT(String),
    FLOAT(String),
    STRING(String),
    VECINT(String),
    VECUINT(String),
    VECFLOAT(String),
    VECSTRING(String)
}
impl std::fmt::Debug for CLPExpectedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CLPExpectedType::INT(name) => write!(f, "INT({})", name),
            CLPExpectedType::UINT(name) => write!(f, "UINT({})", name),
            CLPExpectedType::FLOAT(name) => write!(f, "FLOAT({})", name),
            CLPExpectedType::STRING(name) => write!(f, "STRING({})", name),
            CLPExpectedType::VECINT(name) => write!(f, "[...INT({})...]", name),
            CLPExpectedType::VECUINT(name) => write!(f, "[...UINT({})...]", name),
            CLPExpectedType::VECFLOAT(name) => write!(f, "[...FLOAT({})...]", name),
            CLPExpectedType::VECSTRING(name) => write!(f, "[...STRING({})...]", name),
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