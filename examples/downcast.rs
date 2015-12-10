#[macro_use(match_error)]
extern crate error;

use std::error::Error as StdError;
use std::fmt::Error as FmtError;
use error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParseError {
    location: usize,
}

impl StdError for ParseError {
    fn description(&self) -> &str { "Parse Error" }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.description().fmt(f)
    }
}

fn produce_parse_error() -> Box<Error + Send> {
    Box::new(ParseError { location: 7 })
}

fn generic_handler(raw: Box<Error + Send>) {
    (match_error! { &*raw,
        parse => ParseError: {
            assert_eq!(*parse, ParseError { location: 7 })
        }
    }).unwrap()
}

fn main() {
    generic_handler(produce_parse_error())
}

