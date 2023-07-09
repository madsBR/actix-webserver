use std::fmt;
use askama::Template;
use crate::HOMEPAGE;


#[derive(Template)]
#[template(path="resume.html",escape="none")]
pub struct CVTemplate {
    scope : &'static str
}

impl CVTemplate{
    pub fn new(&) -> Self{
        CVTemplate{}
    }
}
