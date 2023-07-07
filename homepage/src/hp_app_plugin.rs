pub use app_plugin::AppPlugin;
use async_trait::async_trait;
use actix_web::{get,web::{self,Data, ServiceConfig,scope},  HttpResponse,  Responder};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use std::{time::Duration, io::Read};
use crate::HOMEPAGE;
use crate::index_template::HPTemplate;
use askama::Template;
use askama::Error as AskaErr;


#[get("/index")]
async fn index() -> impl Responder {
    let page = HPTemplate::new().render();
//    let page : Result<&str,usize> = Ok("hej");
    let response = match page{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    };
    response
}




// this function could be located in a different module
fn config1(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

pub struct HPConfig{}

#[async_trait]
impl AppPlugin for HPConfig {
    const SCOPE : &'static str = "homepage";
    async fn scheduled_process(&self){}
    fn config(cfg: &mut ServiceConfig){
        config1(cfg);
    }
}