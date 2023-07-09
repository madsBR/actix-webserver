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
 