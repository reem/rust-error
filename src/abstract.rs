use super::RawError;
use std::fmt::{Show, Formatter, FormatError};
use std::any::Any;

pub trait AbstractError: Show {
    fn description(&self) -> &Option<&'static str>;
    fn details(&self) -> &Option<String>;
    fn cause(&self) -> &Option<Box<AbstractError>>;

    fn extensions(&self) -> &Option<Box<Any>>;
    fn extensions_mut(&mut self) -> &mut Option<Box<Any>>;
}

impl<T> AbstractError for RawError<T> {
    fn description(&self) -> &Option<&'static str> { &self.description }
    fn details(&self) -> &Option<String> { &self.details }
    fn cause(&self) -> &Option<Box<AbstractError>> { &self.cause }

    fn extensions(&self) -> &Option<Box<Any>> { &self.extensions }
    fn extensions_mut(&mut self) -> &mut Option<Box<Any>> { &mut self.extensions }
}

impl Show for Box<AbstractError> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> { self.fmt(f) }
}

