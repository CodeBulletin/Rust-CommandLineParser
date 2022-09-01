use std::{collections::HashMap};
use crate::variables::{CLPExpectedType, CLPOutputType, CLPErrorKind, CLPType};

pub enum ArgsSetting {
    NONE,
    ALL(CLPType),
    Args(Vec<CLPType>)
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

fn match_arg(argument: &CLPExpectedType, var_name: &String, arg: &String, vars: &mut HashMap<String, CLPOutputType>) -> Option<CLPErrorKind> {
    //todo: parsing for vectors
    match &argument {
        CLPExpectedType::INT => {
            let value: Result<i128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPOutputType::Int(value)
            );
        },
        CLPExpectedType::UINT => {
            let value: Result<u128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as unsigned integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPOutputType::UInt(value)
            );
        }
        CLPExpectedType::FLOAT => {
            let value: Result<f64, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as float", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPOutputType::Float(value)
            );
        },
        CLPExpectedType::STRING => {
            vars.insert(
                var_name.clone(),
                CLPOutputType::String(arg.clone())
            );
        },
        CLPExpectedType::VECINT => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, arg)));
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
                    var_name.clone(),
                    CLPOutputType::VecInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        },
        CLPExpectedType::VECUINT => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, arg)));
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
                    var_name.clone(),
                    CLPOutputType::VecUInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        }
        CLPExpectedType::VECFLOAT => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, arg)));
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
                    var_name.clone(),
                    CLPOutputType::VecFloat(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        }
        CLPExpectedType::VECSTRING => {
            let num_chars = arg.chars().count();
            if num_chars < 2 {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, arg)));
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
                        return Some(CLPErrorKind::ParseError(format!("expected , got {} at the end of the argument {}", arg.chars().nth(id).unwrap(), arg)));
                    } else if !isfirst {
                        isfirst = true;
                        id += 1;
                        continue;
                    }
                    if arg.chars().nth(id).unwrap() == '\'' && isfirst {
                        id += 1;
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
                    var_name.clone(),
                    CLPOutputType::VecString(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
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
                        let result = match_arg(&arguments[index].object_type, &arguments[index].name, arg, &mut vars);
                        if let Some(err) = result {
                            return Err(err);
                        }
                    }
                    else if !toggle && index >= arguments.len() && !self.allow_more {
                        return Err(CLPErrorKind::Error(format!("Unexpected input {}", arg)));
                    }
                    else if toggle {
                        todo!();
                    }
                    index += 1;
                }
                if index < arguments.len() {
                    return Err(CLPErrorKind::ArgsError(format!("Didn't get input for {:?}", arguments[index])));
                }
            }
            ArgsSetting::ALL(expected) => {
                let mut index: usize = 0;
                let mut inlist = true;
                for arg in iter {
                    let toggle = self.kwargs.keyvalues.contains_key(arg);
                    if toggle {
                        inlist = false;
                    }
                    if inlist {
                        let name = expected.name.clone() + format!("{}", index).as_str();
                        match_arg(&expected.object_type, &name, arg, &mut vars);
                    } else {
                        todo!();
                    }
                    index += 1;
                }
            }
            _ => {}
        }

        return Ok(vars);
    }
}