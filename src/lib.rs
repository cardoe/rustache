#![deny(missing_docs)]
#![recursion_limit = "1024"]

//! The main crate for the Rustache library.
//!
//! Rustache is a flexible template engine for Rust.
#[macro_use]
extern crate error_chain;
extern crate rustc_serialize;

use std::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::string;

pub use build::{HashBuilder, VecBuilder};
pub use rustache::Render;

/// Alias for Result<T, `RustacheError`>
pub use errors::*;

// Represents the possible types that passed in data may take on
#[doc(hidden)]
#[derive(Debug)]
pub enum Data<'a> {
    String(string::String),
    Bool(bool),
    Integer(i32),
    Float(f64),
    Vector(self::Vector<'a>),
    Hash(self::Hash<'a>),
    Lambda(RefCell<self::Lambda<'a>>),
}

/// Alias for mustache data vectors
pub type Vector<'a> = Vec<Data<'a>>;
/// Alias for mustache data hashes
pub type Hash<'a> = HashMap<string::String, Data<'a>>;
/// Alias for a Lambda functions to transform data
pub type Lambda<'a> = &'a mut FnMut(string::String) -> string::String;

impl<'a, 'b> From<&'b str> for Data<'a> {
    fn from(v: &'b str) -> Data<'a> {
        Data::String(v.to_owned())
    }
}

impl<'a> From<string::String> for Data<'a> {
    fn from(v: string::String) -> Data<'a> {
        Data::String(v)
    }
}

impl<'a> From<bool> for Data<'a> {
    fn from(v: bool) -> Data<'a> {
        Data::Bool(v)
    }
}

impl<'a> From<i32> for Data<'a> {
    fn from(v: i32) -> Data<'a> {
        Data::Integer(v)
    }
}

impl<'a> From<f64> for Data<'a> {
    fn from(v: f64) -> Data<'a> {
        Data::Float(v)
    }
}

impl<'a> From<self::Vector<'a>> for Data<'a> {
    fn from(v: self::Vector<'a>) -> Data<'a> {
        Data::Vector(v)
    }
}

impl<'a> From<self::Hash<'a>> for Data<'a> {
    fn from(v: self::Hash<'a>) -> Data<'a> {
        Data::Hash(v)
    }
}

impl<'a> From<self::Lambda<'a>> for Data<'a> {
    fn from(v: self::Lambda<'a>) -> Data<'a> {
        Data::Lambda(RefCell::new(v))
    }
}

// |String|: 'a -> String : F Above

// Implementing custom PartialEq for Data
impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Data::String(ref val0), &Data::String(ref val1)) => val0 == val1,
            (&Data::Bool(ref val0), &Data::Bool(ref val1)) => val0 == val1,
            (&Data::Integer(ref val0), &Data::Integer(ref val1)) => val0 == val1,
            (&Data::Float(ref val0), &Data::Float(ref val1)) => val0 == val1,
            (&Data::Vector(ref val0), &Data::Vector(ref val1)) => val0 == val1,
            (&Data::Hash(ref val0), &Data::Hash(ref val1)) => val0 == val1,
            (&Data::Lambda(_), &Data::Lambda(_)) => panic!("Can't compare closures"),
            (_, _) => false,
        }
    }
}

// Implementing custom Show for Lambda
impl <'a> fmt::Debug for Lambda<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lambda(...)")
    }
}

// Internal Modules
mod errors;
mod rustache;
mod compiler;
mod parser;
mod build;
mod template;
