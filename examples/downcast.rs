#![allow(staged_unstable)]

#[macro_use(match_error)]
extern crate error;

use std::error::Error as StdError;
use error::Error;

#[derive(Show, PartialEq, Copy)]
pub struct ParseError {
    location: uint,
}

impl StdError for ParseError {
    fn description(&self) -> &str { "Parse Error" }
}

fn produce_parse_error() -> Box<Error> {
    Box::new(ParseError { location: 7u })
}

fn generic_handler(raw: Box<Error>) {
    (match_error! { &*raw,
        parse => ParseError: {
            assert_eq!(*parse, ParseError { location: 7u })
        }
    }).unwrap()
}

fn main() {
    generic_handler(produce_parse_error())
}

