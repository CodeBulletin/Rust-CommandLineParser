use std::collections::HashMap;
use clparser::{CommandLineParser, KwargSettings, ArgsSettings, KwargTypes, CLPErrorKind, CLPInput};
use clparser::ArgTypes::{STRING, VECSTRING, VECFLOAT};

fn main() {
    let mut map = HashMap::<String, KwargTypes>::new();
    map.insert("--int".to_string(), KwargTypes::Important([
        "Integer".to_string(),
        "<int>".to_string(),
        "unexpected value {} expected an <int>".to_string()
    ]));
    let parser = CommandLineParser::new(
        false,
        ArgsSettings::Args(vec![
            CLPInput {
                object_type: STRING,
                name: "int".to_string()
            },
            CLPInput {
                object_type: VECSTRING,
                name: "int".to_string()
            },
            CLPInput {
                object_type: VECFLOAT,
                name: "int".to_string()
            }
        ]),
        KwargSettings {
            keyvalues: map
        });
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
