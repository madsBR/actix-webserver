use std::fmt;
use askama::Template;
use crate::HOMEPAGE;


#[derive(Template)]
#[template(path="index.html",escape="none")]
pub struct HPTemplate {}

impl HPTemplate{
    pub fn new() -> Self{
        HPTemplate{}
    }
}
 