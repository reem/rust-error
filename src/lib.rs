#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

use std::any::{Any, AnyRefExt};
use std::fmt::{Show, Formatter, FormatError};
use std::{raw, mem};
use std::intrinsics::TypeId;

pub trait Error: Show + Any + ErrorPrivate {
    fn name(&self) -> &'static str;

    fn description(&self) -> Option<&str> { None }

    fn cause(&self) -> Option<&Error> { None }

    fn unwrap(self) -> Option<Box<Error>> { None }

    fn abstract(self) -> Box<Error> { box self as Box<Error> }
}

// Oh DST we wait for thee.
pub trait ErrorRefExt<'a> {
    fn is<O: 'static>(self) -> bool;
    fn downcast<O: 'static + Error>(self) -> Option<&'a O>;
}

impl<'a> ErrorRefExt<'a> for &'a Error {
    fn is<O: 'static>(self) -> bool {
        self.type_id() == TypeId::of::<O>()
    }

    fn downcast<O: 'static + Error>(self) -> Option<&'a O> {
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

impl Show for Box<Error> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> { self.fmt(f) }
}

#[cfg(test)]
mod test {
    use super::{Error, ErrorRefExt};

    #[deriving(Show, PartialEq)]
    pub struct ParseError {
        location: uint,
    }

    impl Error for ParseError {
        fn name(&self) -> &'static str { "Parse Error" }
    }

    #[test] fn test_generic() {
        fn produce_parse_error() -> Box<Error> {
            ParseError { location: 7u }.abstract()
        }

        fn generic_handler(raw: Box<Error>) {
            let parse = raw.downcast::<ParseError>().unwrap();
            assert_eq!(*parse, ParseError { location: 7u });
        }

        generic_handler(produce_parse_error())
    }
}

