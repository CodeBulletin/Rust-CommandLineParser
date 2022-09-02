pub mod types;

use std::{collections::HashMap};
use types::{ArgsSettings, ArgTypes, KwargSettings, CLPResult, CLPVar, CLPErrorKind};

pub struct CommandLineParser {
    pub allow_more: bool,
    pub args: ArgsSettings,
    pub kwargs: KwargSettings
}

fn match_arg(argument: &ArgTypes, var_name: &String, arg: &String, vars: &mut HashMap<String, CLPVar>) -> Option<CLPErrorKind> {
    //todo: parsing for vectors
    match &argument {
        ArgTypes::INT => {
            let value: Result<i128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPVar::Int(value)
            );
        },
        ArgTypes::UINT => {
            let value: Result<u128, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as unsigned integer", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPVar::UInt(value)
            );
        }
        ArgTypes::FLOAT => {
            let value: Result<f64, _> = arg.parse();
            if let Err(_) = value {
                return Some(CLPErrorKind::ParseError(format!("unable to parse {:?} as float", arg))); 
            }
            let value = value.unwrap();
            vars.insert(
                var_name.clone(),
                CLPVar::Float(value)
            );
        },
        ArgTypes::STRING => {
            vars.insert(
                var_name.clone(),
                CLPVar::String(arg.clone())
            );
        },
        ArgTypes::VECINT => {
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
                    CLPVar::VecInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        },
        ArgTypes::VECUINT => {
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
                    CLPVar::VecUInt(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        }
        ArgTypes::VECFLOAT => {
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
                    CLPVar::VecFloat(v)
                );
            }
            else {
                return Some(CLPErrorKind::ArgsError(format!("Expected {:?} got {:?}", argument, *arg)));
            }
        }
        ArgTypes::VECSTRING => {
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
                    CLPVar::VecString(v)
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
    pub fn new(allow_more: bool, args: ArgsSettings, kwargs:KwargSettings) -> CommandLineParser {
        CommandLineParser {
            allow_more,
            args,
            kwargs
        }
    }
    pub fn parse(&self, args: &Vec<String>) -> Result<CLPResult, CLPErrorKind> {
            
        let mut iter = args.iter();
            
        let mut vars: HashMap<String, CLPVar> = HashMap::new();
        let mut default: Vec<String> = Vec::new();
        let arg0 = iter.next().unwrap().clone(); 
        let get_default = self.allow_more;

        match &self.args {
            ArgsSettings::Args(arguments) => {
                let mut index: usize = 0;
                while let Some(arg) = iter.next() {
                    let toggle = self.kwargs.keyvalues.contains_key(arg);
                    if toggle {
                        if index < arguments.len() {
                            return Err(CLPErrorKind::ArgsError(format!("Expected {:?} got {}", arguments[index], arg)));
                        } else {
                            todo!()
                        }
                    } else {
                        if index < arguments.len() {
                            let result = match_arg(&arguments[index].object_type, &arguments[index].name, arg, &mut vars);
                            if let Some(err) = result {
                                return Err(err);
                            }
                        } else if get_default {
                            default.push(arg.clone());
                        } else {
                            return Err(CLPErrorKind::Error(format!("Unexpected input {}", arg)));
                        }
                    }
                    index += 1;
                }
                if index < arguments.len() {
                    return Err(CLPErrorKind::ArgsError(format!("Didn't get input for {:?}", arguments[index])));
                }
            }
            ArgsSettings::ALL(expected) => {
                let mut index: usize = 0;
                let mut inlist = true;
                for arg in iter {
                    let toggle = self.kwargs.keyvalues.contains_key(arg);
                    if toggle {
                        inlist = false;
                    }
                    if inlist {
                        let name = expected.name.clone() + format!("{}", index).as_str();
                        let result = match_arg(&expected.object_type, &name, arg, &mut vars);
                        if let Some(err) = result {
                            return Err(err);
                        }
                    } else {
                        if !toggle && get_default {
                            default.push(arg.clone());
                        } else if !toggle {
                            return Err(CLPErrorKind::Error(format!("Unexpected input {}", arg)));
                        } else {
                            todo!();
                        }
                    }
                    index += 1;
                }
            },
            ArgsSettings::NONE => {
                while let Some(arg) = iter.next() {
                    let toggle = self.kwargs.keyvalues.contains_key(arg);
                    if toggle {
                        todo!();
                    } else {
                        if get_default {
                            default.push(arg.clone());
                        } else {
                            return Err(CLPErrorKind::Error(format!("Unexpected input {}", arg)));
                        }
                    }
                }
            }
        }

        return Ok(CLPResult {
            arg0,
            default,
            args: vars
        });
    }
}