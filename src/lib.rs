mod parser;
mod variables;
pub use parser::{CommandLineParser, ArgsSetting, KwargSettings, KwargType};
pub use variables::{CLPOutputType, CLPExpectedType, CLPErrorKind};

#[cfg(test)]
mod test {
    use crate::{CommandLineParser, ArgsSetting, KwargSettings, KwargType};
    use crate::{CLPOutputType, CLPExpectedType, CLPErrorKind};
    use CLPExpectedType::{INT, FLOAT, UINT, STRING};

    #[test]
    #[should_panic]
    fn test_args_mismatch() {
        let map = std::collections::HashMap::<String, KwargType>::new();
        let parser = CommandLineParser {
            allow_more: false,
            args: ArgsSetting::Args(vec![INT("a".to_string()), UINT("b".to_string())]),
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
            args: ArgsSetting::Args(vec![INT("a".to_string()), UINT("b".to_string())]),
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
    // fn test_Over
}