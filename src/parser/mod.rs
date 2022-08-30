use std::{collections::HashMap};
use crate::variables::{ExpectedVar, OutputVar, CLPErrorKind};

pub enum ArgsSetting {
    NONE,
    ALL,
    Args(Vec<ExpectedVar>)
}

pub enum KwargType {
    Important([String; 3]),
    Optional([String; 3])
}

pub struct KwargSettings {
    pub keyvalues: HashMap<String, KwargType>
}

pub struct CommandLineParser {
    pub allow_more: bool,
    pub args: ArgsSetting,
    pub kwargs: KwargSettings
}

fn match_arg(arguments: &Vec<ExpectedVar>, index: usize, arg: &String, vars: &mut HashMap<String, OutputVar>) -> Option<CLPErrorKind> {
    //todo: parsing for vectors
    match &arguments[index] {
        ExpectedVar::INT(name) => {
            let value: Result<i128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                OutputVar::Int(value)
            );
        },
        ExpectedVar::UINT(name) => {
            let value: Result<u128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as unsigned integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                OutputVar::UInt(value)
            );
        }
        ExpectedVar::FLOAT(name) => {
            let value: Result<f64, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as float", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                OutputVar::Float(value)
            );
        },
        ExpectedVar::STRING(name) => {
            vars.insert(
                name.clone(),
                OutputVar::String(arg.clone())
            );
        },
        ExpectedVar::VECINT(name) => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], arg)));
            }
            if arg.chars().nth(0).unwrap() == '[' && arg.chars().nth(num_chars - 1).unwrap() == ']' {
                let mut v: Vec<i128> = Vec::new();
                for i in arg[1..num_chars-1].split(",") {
                    let value: Result<i128, _> = i.parse();
                    if let Err(_) = value {
                        return Some(CLPErrorKind::ParseError(format!("Expected INT got {:?} in arg {}", i, arg)));
                    }
                    v.push(value.unwrap());
                }
                vars.insert(
                    name.clone(),
                    OutputVar::VecInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        },
        ExpectedVar::VECUINT(name) => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], arg)));
            }
            if arg.chars().nth(0).unwrap() == '[' && arg.chars().nth(num_chars - 1).unwrap() == ']' {
                let mut v: Vec<u128> = Vec::new();
                for i in arg[1..num_chars-1].split(",") {
                    let value: Result<u128, _> = i.parse();
                    if let Err(_) = value {
                        return Some(CLPErrorKind::ParseError(format!("Expected UINT got {:?} in arg {}", i, arg)));
                    }
                    v.push(value.unwrap());
                }
                vars.insert(
                    name.clone(),
                    OutputVar::VecUInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        }
        _ => {}
    }
    return None;
}

impl CommandLineParser {
    pub fn parse(&self, args: &Vec<String>) -> Result<HashMap<String, OutputVar>, CLPErrorKind> {
        let mut vars: HashMap<String, OutputVar> = HashMap::new();

        let mut iter = args.iter();

        vars.insert(
            "SELF".to_string(),
            OutputVar::String(iter.next().unwrap().clone())
        );

        match &self.args {
            ArgsSetting::Args(arguments) => {
                let mut index: usize = 0;
                for arg in iter {
                    let toggle = self.kwargs.keyvalues.contains_key(arg);
                    if toggle && index < arguments.len() {
                        return Err(CLPErrorKind::ArgsError(format!("Expected {:?} got {}", arguments[index], arg)));
                    }
                    else if !toggle && index < arguments.len() {
                        let result = match_arg(arguments, index, arg, &mut vars);
                        if let Some(err) = result {
                            return Err(err);
                        }
                    }
                    else if !toggle && index >= arguments.len() && !self.allow_more {
                        return Err(CLPErrorKind::Error(format!("Unexpected input {:?}", arg)));
                    }
                    index += 1;
                }
                if index < arguments.len() {
                    return Err(CLPErrorKind::ArgsError(format!("Didn't get input for {:?}", arguments[index])));
                }
            }
            _ => {}
        }

        return Ok(vars);
    }
}