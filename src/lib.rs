#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

use std::any::{Any, AnyRefExt};
use std::fmt::Show;
use std::{raw, mem};
use std::intrinsics::TypeId;

pub trait Error: Show + Any + ErrorPrivate {
    fn name(&self) -> &'static str;

    fn description(&self) -> Option<String> {
        None
    }

    fn cause(&self) -> Option<Box<Error>> {
        None
    }

    fn abstract(self) -> Box<Error> { box self as Box<Error> }
}

impl<T: 'static> RawError<T> {
    pub fn new(description: &'static str) -> RawError<T> {
        RawError {
            description: Some(description),
            details: None,
            extensions: None,
            cause: None
        }
    }

    pub fn wrap<R: 'static>(description: &'static str, sub: RawError<R>) -> RawError<T> {
        RawError {
            description: Some(description),
            details: None,
            extensions: None,
            cause: Some(sub.abstract())
        }
    }

    pub fn wrap_with_details<R: 'static>(description: &'static str,
                                         details: String, sub: RawError<R>) -> RawError<T> {
        RawError {
            description: Some(description),
            details: Some(details),
            extensions: None,
            cause: Some(sub.abstract())
        }
    }

    pub fn with_details(description: &'static str, details: String) -> RawError<T> {
        RawError {
            description: Some(description),
            details: Some(details),
            extensions: None,
            cause: None
        }
    }

    pub fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }

    pub fn abstract(self) -> Box<AbstractError> { box self as Box<AbstractError> }
}

pub trait Error<T: 'static>: Convertible<RawError<T>> {
    fn as_raw(&self) -> RawError<T>;

    fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }
}

impl<O: 'static, E: Error<O>> Convertible<E> for RawError<O> {
    fn convert(err: &E) -> Option<RawError<O>> { Some(err.as_raw()) }
}

impl<T> Show for RawError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "RawError {{ description: {}, details: {}, cause: {} }}", &self.description, &self.details, &self.cause)
    }
}

#[cfg(test)]
mod test {
    use super::{RawError, Error};
    use convertible::{Convertible, Raw};

    #[deriving(Show)]
    pub struct ParseErrorMarker;

    #[deriving(Show, PartialEq)]
    pub struct ParseError {
        location: uint,
    }

    impl<T: 'static> Convertible<RawError<T>> for ParseError {
        fn convert(err: &RawError<T>) -> Option<ParseError> {
            if err.is::<ParseErrorMarker>() {
                Some(ParseError {
                    location: 7u,
                })
            } else {
                None
            }
        }
    }

    impl Error<ParseErrorMarker> for ParseError {
        fn as_raw(&self) -> RawError<ParseErrorMarker> {
            RawError {
                description: Some("Parse-Error"),
                details: None,
                extensions: None,
                cause: None
            }
        }
    }

    #[test] fn test_convert_from_raw() {
        let raw: RawError<ParseErrorMarker> = RawError {
            description: None,
            details: None,
            extensions: None,
            cause: None
        };

        let parse = raw.to::<ParseError>().unwrap();

        assert_eq!(parse, ParseError { location: 7u });
    }

    #[test] fn test_convert_to_raw() {
        let parse = ParseError { location: 10u };
        assert_eq!(parse.to::<RawError<ParseErrorMarker>>().unwrap().description, Some("Parse-Error"));
    }

    #[test] fn test_generic() {
        fn produce_parse_error() -> RawError<ParseErrorMarker> {
            RawError {
                description: None,
                details: None,
                extensions: None,
                cause: None
            }
        }

        fn generic_handler<T: 'static>(raw: RawError<T>) {
            let parse = raw.to::<ParseError>().unwrap();
            assert_eq!(parse, ParseError { location: 7u });
        }

        generic_handler(produce_parse_error())
    }
}

