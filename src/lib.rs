#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

extern crate convertible;

use convertible::Convertible;
use std::intrinsics::TypeId;

pub struct Phantom<T>;

pub struct RawError<Marker> {
    pub description: Option<&'static str>,
    pub details: Option<String>
}

impl<T: 'static> RawError<T> {
    pub fn is<O: 'static, E: Error<O>>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<O>() }
}

pub trait Error<T: 'static>: Convertible<RawError<T>> {
    fn as_raw(&self) -> RawError<T>;

    fn is<R: 'static>(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<R>()
    }

    fn marker(Phantom<Self>) -> T;
}

impl<O: 'static, E: Error<O>> Convertible<E> for RawError<O> {
    fn convert(err: &E) -> Option<RawError<O>> { Some(err.as_raw()) }
}

