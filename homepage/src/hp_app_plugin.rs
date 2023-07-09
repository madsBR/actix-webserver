pub use app_plugin::AppPlugin;
use async_trait::async_trait;
use actix_web::{get,web::{self,Data, ServiceConfig,scope,Redirect},  HttpResponse,  Responder};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use std::{time::Duration, io::Read};
use crate::HOMEPAGE;
use crate::index_template::HPTemplate;
use crate::cv_template::CVTemplate;
use askama::Template;
use askama::Error as AskaErr;




#[get("/intro")]
async fn front_page() -> impl Responder {
    let page = HPTemplate::new().render();
//    let page : Result<&str,usize> = Ok("hej");
    let response = match page{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    };
    response
}


#[get("/resume")]
async fn resume() -> impl Responder {
    let page = CVTemplate::new().render();
//    let page : Result<&str,usize> = Ok("hej");
    let response = match page{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    };
    response
}




pub struct HPConfig{}

#[async_trait]
impl AppPlugin for HPConfig {
    const SCOPE : &'static str = "homepage";
    async fn scheduled_process(&self){}
    fn config(cfg: &mut ServiceConfig){
        cfg
        .service(web::redirect("/index",format!("/intro")))
//        .service(web::redirect("/index",format!("/{}/",Self::SCOPE)))
//        .service(web::redirect("",format!("/{}/index",Self::SCOPE)))
        .service(front_page)
        .service(resume)
        ;
    }
}