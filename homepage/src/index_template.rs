#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    unused_import_braces,
    dead_code
)]


use std::fmt;
use askama::Template;
use crate::HOMEPAGE;


#[derive(Template)]
#[template(path="index.html",escape="none")]
pub struct HPTemplate {
    scope : &'static str
}

impl HPTemplate{
    pub fn new(scope : &'static str) -> Self{
        HPTemplate{scope : scope}
    }
}
 