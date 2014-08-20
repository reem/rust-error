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

// Oh DST we wait for thee.
pub trait ErrorRefExt<'a> {
    fn is<O: 'static>(self) -> bool;
    fn downcast<O: 'static>(self) -> Option<&'a O>;
}

impl<'a> ErrorRefExt<'a> for &'a Error {
    fn is<O: 'static>(self) -> bool {
        self.type_id() == TypeId::of::<O>()
    }

    fn downcast<O: 'static>(self) -> Option<&'a O> {
        // Copied from std::any::Any
        if self.is::<O>() {
            unsafe {
                // Get the raw representation of the trait object
                let to: raw::TraitObject = mem::transmute_copy(&self);

                // Extract the data pointer
                Some(mem::transmute(to.data))
            }
        } else {
            None
        }
    }
}

// Copied from std::any::Any.
trait ErrorPrivate {
    fn type_id(&self) -> TypeId;
}

impl<T: 'static> ErrorPrivate for T {
    fn type_id(&self) -> TypeId { TypeId::of::<T>() }
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

