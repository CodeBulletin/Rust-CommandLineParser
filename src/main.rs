use std::collections::HashMap;
use clparser::{CommandLineParser, KwargSettings, ArgsSetting, KwargType, CLPErrorKind};
use clparser::ExpectedVar::{INT, FLOAT, UINT, STRING, VECINT, VECUINT};

fn main() {
    let mut map = HashMap::<String, KwargType>::new();
    map.insert("--int".to_string(), KwargType::Important([
        "Integer".to_string(),
        "<int>".to_string(),
        "unexpected value {} expected an <int>".to_string()
    ]));
    map.insert("--float".to_string(), KwargType::Important([
        "Float".to_string(),
        "<float>".to_string(),
        "unexpected value {} expected an <float>".to_string()
    ]));

    let parser = CommandLineParser {
        allow_more: false,
        args: ArgsSetting::Args(vec![
            INT("a".to_string()),
            UINT("b".to_string()),
            FLOAT("c".to_string()),
            STRING("d".to_string()),
            VECINT("e".to_string()),
            VECUINT("f".to_string()),
        ]),
        kwargs: KwargSettings {
            keyvalues: map
        }
    };

    let args: Vec<String> = std::env::args().collect();
    let variables = parser.parse(&args);

    match variables {
        Err(err) => {
            match err {
                CLPErrorKind::Error(value) => println!("{}", value),
                CLPErrorKind::ArgsError(value) => println!("{}", value),
                CLPErrorKind::KwargError(value) => println!("{}", value),
                CLPErrorKind::ParseError(value) => println!("{}", value),
            }
        },
        Ok(value) => {
            println!("{:#?}", value);
        }
    }
}
