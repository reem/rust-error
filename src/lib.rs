#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]
#![feature(macro_rules)]

//! A generic, extendable Error type.

use std::any::{Any, AnyRefExt};
use std::fmt::{Show, Formatter, FormatError};
use std::{raw, mem};
use std::intrinsics::TypeId;

pub trait Error: Show + Any + Send + ErrorPrivate {
    fn name(&self) -> &'static str;

    fn description(&self) -> Option<&str> { None }

    fn cause(&self) -> Option<&Error + Send> { None }

    fn unwrap(self) -> Option<Box<Error + Send>> { None }

    fn abstract(self) -> Box<Error + Send> { box self as Box<Error + Send> }
}

// Oh DST we wait for thee.
pub trait ErrorRefExt<'a> {
    fn is<O: 'static>(self) -> bool;
    fn downcast<O: 'static + Error>(self) -> Option<&'a O>;
}

impl<'a> ErrorRefExt<'a> for &'a Error + Send {
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

impl Show for Box<Error + Send> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> { self.fmt(f) }
}

impl Error for String {
    fn name(&self) -> &'static str { "String-Only Error" }
    fn description(&self) -> Option<&str> { Some(self.as_slice()) }
}

#[macro_export]
macro_rules! match_error {
    ($m:expr, $i1:pat: $t1:ty => $e1:expr) => {{
        let tmp = $m;
        match tmp.downcast::<$t1>() {
            Some($i1) => Some($e1),
            None => None,
        }
    }};

    ($m:expr, $i1:pat: $t1:ty => $e1:expr, $($i:pat: $t:ty => $e:expr),+) => {{
        let tmp = $m;
        match tmp.downcast::<$t1>() {
            Some($i1) => Some($e1),
            None => match_error!(tmp, $($i: $t => $e),*),
        }
    }};
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
        fn produce_parse_error() -> Box<Error + Send> {
            ParseError { location: 7u }.abstract()
        }

        fn generic_handler(raw: Box<Error + Send>) {
            (match_error! { raw,
                parse: ParseError => {
                    assert_eq!(*parse, ParseError { location: 7u })
                }
            }).unwrap()
        }

        generic_handler(produce_parse_error())
    }
}

