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


#[derive(Template)]
#[template(path="post-piece.html",escape="none")]
pub struct PostPieceTemplate {
    scope : &'static str
}

impl PostPieceTemplate{
    pub fn new(scope : &'static str) -> Self{
        PostPieceTemplate{scope : scope}
    }
}
 