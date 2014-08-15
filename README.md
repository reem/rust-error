# Error

> An extensible, typesafe error for everyone.

## Motivation

Rust is in need of an extensible solution to error types - this is one attempt.

This crate provides the `Error` trait, which defines a simple interface for
interacting with Errors. Through the use of `rust-convertible`, this crate
provides a flexible API for effectively allowing an "open-enum" of error
options by allowing runtime conversion between the `RawError` type and
specific error types which can house further information.

The `RawError` type contains a phantom type parameter which is used to indicate
the true type of the error, but can easily be parametrized over for generic
handlers and generic throwers. `RawError::is` allows handlers to check for
error types that they know they can handle, and `RawError::to`, provided by
`convertible`, allows handlers to convert the `RawError` to an error variant
that they can do work with.

Additionally, `RawError` contains a `Box<Any>` field for arbitrary extensions
to the error type. I considered using `AnyMap` and/or `rust-plugin` for this
purpose but since the data needs to be closely associated with this specific
error I figured a `Box<Any>` field is more general.

## Overview

`RawError` is the standard error representation exposed by this crate. Error
throwers should return `Result<T, RawError<TheirErrorType>>` and error handlers
should take a generic `RawError` and check its type, or, if they are called in
places where their callers can ensure the phantom type of the `RawError`, they
can receive `RawError<SpecificErrorMarker>`.

These handlers can then check if the `RawError` is of a type they can handle
through `RawError::is`, which is a simple `TypeId` comparison, and can then
attempt to convert the error to a non-raw representation using
`RawError::convert`, which has semantics defined in `rust-convertible`.

To define a concrete error type which can be converted to from `RawError`,
implement `Convertible<RawError>` and `Error<YourMarkerType>` for your error
type.

`RawError` also provides error chaining through the `cause` field, which
contains a `Option<Box<AbstractError>>`, where `None` indicates that this
error is the original error. `AbstractError` is just a trait which has
accessor methods for all of the fields of `RawError`. It exists purely
as a way to erase the type parameter of `RawError` so it can be properly
nested.

## Example


```rust
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

#[test] fn test_generic() {
    fn produce_parse_error() -> RawError<ParseErrorMarker> {
        RawError {
            description: None, details: None,
            extensions: None, cause: None
        }
    }

    fn generic_handler<T: 'static>(raw: RawError<T>) {
        match raw.to::<ParseError>() {
            Some(parse) => assert_eq!(parse, ParseError { location: 7u }),
            None => fail!("Unhandle-able error.")
        }
    }

    generic_handler(produce_parse_error())
}
```

