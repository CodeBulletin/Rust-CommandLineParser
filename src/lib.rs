mod parser;
mod variables;
pub use parser::{CommandLineParser, ArgsSetting, KwargSettings, KwargType};
pub use variables::{CLPOutputType, CLPExpectedType, CLPErrorKind, CLPType};

#[cfg(test)]
mod test {
    use crate::{CommandLineParser, ArgsSetting, KwargSettings, KwargType};
    use crate::{CLPExpectedType, CLPType};
    use CLPExpectedType::{INT, UINT, VECSTRING};

    #[test]
    #[should_panic]
    fn test_args_mismatch() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: false,
            args: ArgsSetting::Args(vec![
                CLPType {
                    object_type: INT,
                    name: "a".to_string()
                },
                CLPType {
                    object_type: UINT,
                    name: "b".to_string()
                },
            ]),
            kwargs: KwargSettings {
                keyvalues: map
            }
        };
        let args: Vec<String> = vec![
            "0".to_string(),
            "128".to_string(),
            "-128".to_string()
        ];
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }
    }

    #[test]
    #[should_panic]
    fn test_under() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: false,
            args: ArgsSetting::Args(vec![
                CLPType {
                    object_type: INT,
                    name: "a".to_string()
                },
                CLPType {
                    object_type: UINT,
                    name: "b".to_string()
                },
            ]),
            kwargs: KwargSettings {
                keyvalues: map
            }
        };
        let args: Vec<String> = vec![
            "0".to_string(),
            "128".to_string(),
        ];
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }
    }
    
    #[test]
    fn test_over_args_allow_more() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: true,
            args: ArgsSetting::Args(vec![
                CLPType {
                    object_type: INT,
                    name: "a".to_string()
                },
                CLPType {
                    object_type: UINT,
                    name: "b".to_string()
                },
            ]),
            kwargs: KwargSettings {
                keyvalues: map
            }
        };
        let args: Vec<String> = vec![
            "/target".to_string(),
            "7".to_string(),
            "9".to_string(),
            "11".to_string(),
        ];
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }
    }

    
    #[test]
    #[should_panic]
    fn test_over_args_allow_more_false() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: false,
            args: ArgsSetting::Args(vec![
                CLPType {
                    object_type: INT,
                    name: "a".to_string()
                },
                CLPType {
                    object_type: UINT,
                    name: "b".to_string()
                },
            ]),
            kwargs: KwargSettings {
                keyvalues: map
            }
        };
        let args: Vec<String> = vec![
            "/target".to_string(),
            "7".to_string(),
            "9".to_string(),
            "11".to_string(),
        ];
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }
    }

    #[test]
    fn test_vecstring() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: false,
            args: ArgsSetting::Args(vec![
                CLPType {
                    object_type: VECSTRING,
                    name: "a".to_string()
                },
            ]),
            kwargs: KwargSettings {
                keyvalues: map
            }
        };


        let args: Vec<String> = vec![
            "/target".to_string(),
            "".to_string()
        ];

        let vars = parser.parse(&args);
        if let Err(_) = vars {
        } else {
            panic!("Empty String passed");
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "xyz".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(_) = vars {
        } else {
            panic!("'xyz' String passed");
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "[]".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "[,]".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(_) = vars {
        } else {
            panic!("should have gotten \"expected ' got ',' at the end of the argument [,]\"");
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "['',]".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "['/'/'',]".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(name) = vars {
            panic!("{:?}", name);
        }

        let args: Vec<String> = vec![
            "/target".to_string(),
            "['\\'',]".to_string()
        ];
        
        let vars = parser.parse(&args);
        if let Err(_) = vars {
        }
        else {
            panic!("should have gotten \"expected , got ' at the end of the argument ['\\'',]\"");
        }
    }
}