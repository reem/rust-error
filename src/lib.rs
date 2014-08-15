#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

extern crate convertible;

use convertible::Convertible;
use std::intrinsics::TypeId;
use std::any::Any;

#[deriving(Show)]
pub struct RawError<Marker> {
    pub description: Option<&'static str>,
    pub details: Option<String>,
    pub extensions: Option<Box<Any>>
}

impl<T: 'static> RawError<T> {
    pub fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }
}

pub trait Error<T: 'static>: Convertible<RawError<T>> {
    fn as_raw(&self) -> RawError<T>;

    fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }
}

impl<O: 'static, E: Error<O>> Convertible<E> for RawError<O> {
    fn convert(err: &E) -> Option<RawError<O>> { Some(err.as_raw()) }
}

