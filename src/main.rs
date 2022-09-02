use std::collections::HashMap;
use clparser::{CommandLineParser, KwargSettings, ArgsSettings, KwargTypes, CLPErrorKind, CLPInput};
use clparser::ArgTypes::{STRING};

fn main() {
    let mut map = HashMap::<String, KwargTypes>::new();
    map.insert("--int".to_string(), KwargTypes::Important([
        "Integer".to_string(),
        "<int>".to_string(),
        "unexpected value {} expected an <int>".to_string()
    ]));
    let parser = CommandLineParser {
        allow_more: true,
        args: ArgsSettings::ALL(
            CLPInput {
                object_type: STRING,
                name: "V".to_string()
            }),
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
