use core::num;
use std::{collections::HashMap};

use crate::variables::{CLPExpectedType, CLPOutputType, CLPErrorKind};

pub enum ArgsSetting {
    NONE,
    ALL,
    Args(Vec<CLPExpectedType>)
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

fn match_arg(arguments: &Vec<CLPExpectedType>, index: usize, arg: &String, vars: &mut HashMap<String, CLPOutputType>) -> Option<CLPErrorKind> {
    //todo: parsing for vectors
    match &arguments[index] {
        CLPExpectedType::INT(name) => {
            let value: Result<i128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                CLPOutputType::Int(value)
            );
        },
        CLPExpectedType::UINT(name) => {
            let value: Result<u128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as unsigned integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                CLPOutputType::UInt(value)
            );
        }
        CLPExpectedType::FLOAT(name) => {
            let value: Result<f64, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as float", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                name.clone(),
                CLPOutputType::Float(value)
            );
        },
        CLPExpectedType::STRING(name) => {
            vars.insert(
                name.clone(),
                CLPOutputType::String(arg.clone())
            );
        },
        CLPExpectedType::VECINT(name) => {
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
                    CLPOutputType::VecInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        },
        CLPExpectedType::VECUINT(name) => {
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
                    CLPOutputType::VecUInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        }
        CLPExpectedType::VECFLOAT(name) => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], arg)));
            }
            if arg.chars().nth(0).unwrap() == '[' && arg.chars().nth(num_chars - 1).unwrap() == ']' {
                let mut v: Vec<f64> = Vec::new();
                for i in arg[1..num_chars-1].split(",") {
                    let value: Result<f64, _> = i.parse();
                    if let Err(_) = value {
                        return Some(CLPErrorKind::ParseError(format!("Expected UINT got {:?} in arg {}", i, arg)));
                    }
                    v.push(value.unwrap());
                }
                vars.insert(
                    name.clone(),
                    CLPOutputType::VecFloat(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        }
        CLPExpectedType::VECSTRING(name) => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], arg)));
            }
            if arg.chars().nth(0).unwrap() == '[' && arg.chars().nth(num_chars - 1).unwrap() == ']' {
                let mut v: Vec<String> = Vec::new();
                let mut string = String::new();
                let mut id: usize = 1;
                let mut isfirst = true;
                while id < num_chars - 1 {
                    if arg.chars().nth(id).unwrap() == ' ' {
                        id += 1;
                        continue;
                    }
                    if !isfirst && arg.chars().nth(id).unwrap() != ',' {
                        return Some(CLPErrorKind::ParseError(format!("expected , got {:?} at the end of the argument {}", arg.chars().nth(id).unwrap(), arg)));
                    } else if !isfirst {
                        isfirst = true;
                        id += 1;
                        continue;
                    }
                    if arg.chars().nth(id).unwrap() == '\'' && isfirst {
                        id += 1;
                        let mut isopen = true;
                        while id < num_chars - 1 && arg.chars().nth(id).unwrap() != '\'' {
                            if arg.chars().nth(id).unwrap() == '/' {
                                let next_char = arg.chars().nth(id+1);
                                if let Some(char) = next_char {
                                    string.push(char);
                                } else {
                                    return Some(CLPErrorKind::ParseError(format!("unexpected / at the end of the argument {}", arg)));
                                }
                                id += 1;
                            } else {
                                string.push(arg.chars().nth(id).unwrap());
                            }
                            id += 1;
                        }
                        if arg.chars().nth(id).unwrap() != '\'' {
                            return Some(CLPErrorKind::ParseError(format!("expected ' got End Of Argument {}", arg)));
                        }
                        v.push(string.clone());
                        string = String::new();
                        isfirst = false;
                    } else if isfirst {
                        return Some(CLPErrorKind::ParseError(format!("expected ' got {:?} at the end of the argument {}", arg.chars().nth(id).unwrap(), arg)));
                    }
                    id += 1;
                }
                if string != "" {
                    v.push(string.clone());
                }
                vars.insert(
                    name.clone(),
                    CLPOutputType::VecString(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", arguments[index], *arg)));
            }
        }
    }
    return None;
}

impl CommandLineParser {
    pub fn parse(&self, args: &Vec<String>) -> Result<HashMap<String, CLPOutputType>, CLPErrorKind> {
        let mut vars: HashMap<String, CLPOutputType> = HashMap::new();

        let mut iter = args.iter();

        vars.insert(
            "SELF".to_string(),
            CLPOutputType::String(iter.next().unwrap().clone())
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