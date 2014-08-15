#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

extern crate convertible;

use convertible::{Raw, Convertible};

pub struct Phantom<T>;

pub struct RawError {
    pub kind: &'static str,
    pub description: Option<&'static str>,
    pub details: Option<String>
}

impl RawError {
    pub fn is<E: Error>(&self) -> bool { Error::is(self, Phantom::<E>) }
}

pub trait Error: Convertible<RawError> {
    fn as_raw(&self) -> RawError;
    fn is(raw: &RawError, _: Phantom<Self>) -> bool {
        raw.to::<Self>().is_some()
    }
}

impl<E: Error> Convertible<E> for RawError {
    fn convert(err: &E) -> Option<RawError> { Some(err.as_raw()) }
}

