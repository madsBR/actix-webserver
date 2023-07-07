use std::fmt;
use askama::Template;


#[derive(Template)]
#[template(path="index.html",escape="none")]
pub struct IndexTemplate {}

impl IndexTemplate{
    pub fn new() -> Self{
        IndexTemplate{}
    }
}
 