//#![deny(missing_doc)]
#![deny(warnings)]
#![feature(macro_rules)]

//! A generic, extendable Error type.

extern crate typeable;

use std::fmt::{mod, Show, Formatter};
use std::{raw, mem};
use std::intrinsics::TypeId;

use typeable::Typeable;

pub trait Error: Show + Send + Typeable {
    fn name(&self) -> &'static str;

    fn description(&self) -> Option<&str> { None }

    fn cause(&self) -> Option<&Error> { None }
}

// Oh DST we wait for thee.
pub trait ErrorRefExt<'a> {
    fn is<O: Error>(self) -> bool;
    fn downcast<O: Error>(self) -> Option<&'a O>;
}

impl<'a> ErrorRefExt<'a> for &'a Error {
    fn is<O: Error>(self) -> bool { self.get_type() == TypeId::of::<O>() }

    fn downcast<O: Error>(self) -> Option<&'a O> {
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

impl Show for Box<Error> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { (**self).fmt(f) }
}

impl Error for String {
    fn name(&self) -> &'static str { "String Error" }
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
        fn produce_parse_error() -> Box<Error> {
            box ParseError { location: 7u }
        }

        fn generic_handler(raw: Box<Error>) {
            (match_error! { raw,
                parse: ParseError => {
                    assert_eq!(*parse, ParseError { location: 7u })
                }
            }).unwrap()
        }

        generic_handler(produce_parse_error())
    }
}

