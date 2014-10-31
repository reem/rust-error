# Error

> An extensible, typesafe error for everyone.

## Motivation

Rust is in need of an extensible solution to error types - this is one attempt.

This crate provides the `Error` trait, which defines a simple interface for
interacting with Errors.

## Overview

Error is compromised of several immutable getters, and is just a shell for
defining a clean, interoperable interface for errors. The real magic of this
crate is in the `ErrorRefExt` trait, which provides the `is` and `downcast`
methods for checking if an `Error` trait object is a specific error.

These methods are very similar to the ones found on `std::any::Any` which
allow for runtime reflection. The benefit of these methods when applied to
Errors is tremendous, as this allows error handlers to accept a generic error
through a `Box<Error>` trait object and then attempt to handle the types of
errors they can before forwarding the error on if they could not handle it
completely.

The primary benefit is that it allows an extensible error system where errors
can not only be easily propagated, but also *handled* across library
boundaries.

## Example

```rust
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
        let parse = raw.downcast::<ParseError>().unwrap();
        assert_eq!(*parse, ParseError { location: 7u });
    }

    generic_handler(produce_parse_error())
}
```

