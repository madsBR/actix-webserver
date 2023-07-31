
use askama::Template;


#[derive(Template)]
#[template(path="index.html",escape="none")]
pub struct IndexTemplate {
    scope : &'static str
}

impl IndexTemplate{
    pub fn new(scope : &'static str) -> Self{
        IndexTemplate{scope}
    }
}
 