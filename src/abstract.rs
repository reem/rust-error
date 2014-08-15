use super::RawError;
use std::fmt::{Show, Formatter, FormatError};
use std::any::Any;

pub trait AbstractError: Show {
    fn description(&self) -> &Option<&'static str>;
    fn description_mut(&mut self) -> &mut Option<&'static str>;

    fn details(&self) -> &Option<String>;
    fn details_mut(&mut self) -> &mut Option<String>;

    fn extensions(&self) -> &Option<Box<Any>>;
    fn extensions_mut(&mut self) -> &mut Option<Box<Any>>;

    fn cause(&self) -> &Option<Box<AbstractError>>;
    fn cause_mut(&mut self) -> &mut Option<Box<AbstractError>>;
}

impl<T> AbstractError for RawError<T> {
    fn description(&self) -> &Option<&'static str> { &self.description }
    fn description_mut(&mut self) -> &mut Option<&'static str> { &mut self.description }

    fn details(&self) -> &Option<String> { &self.details }
    fn details_mut(&mut self) -> &mut Option<String> { &mut self.details }

    fn extensions(&self) -> &Option<Box<Any>> { &self.extensions }
    fn extensions_mut(&mut self) -> &mut Option<Box<Any>> { &mut self.extensions }

    fn cause(&self) -> &Option<Box<AbstractError>> { &self.cause }
    fn cause_mut(&mut self) -> &mut Option<Box<AbstractError>> { &mut self.cause }
}

impl Show for Box<AbstractError> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> { self.fmt(f) }
}

