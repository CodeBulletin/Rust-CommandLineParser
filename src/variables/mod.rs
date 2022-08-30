pub enum ExpectedVar {
    INT(String),
    UINT(String),
    FLOAT(String),
    STRING(String),
    VECINT(String),
    VECUINT(String),
    VECFLOAT(String),
    VECSTRING(String)
}
impl std::fmt::Debug for ExpectedVar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExpectedVar::INT(name) => write!(f, "INT({})", name),
            ExpectedVar::UINT(name) => write!(f, "UINT({})", name),
            ExpectedVar::FLOAT(name) => write!(f, "FLOAT({})", name),
            ExpectedVar::STRING(name) => write!(f, "STRING({})", name),
            ExpectedVar::VECINT(name) => write!(f, "[...INT({})...]", name),
            ExpectedVar::VECUINT(name) => write!(f, "[...UINT({})...]", name),
            ExpectedVar::VECFLOAT(name) => write!(f, "[...FLOAT({})...]", name),
            ExpectedVar::VECSTRING(name) => write!(f, "[...STRING({})...]", name),
        }
    }
}

#[derive(Debug)]
pub enum OutputVar {
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