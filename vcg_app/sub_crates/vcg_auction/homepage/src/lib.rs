pub use app_plugin::AppPlugin;
use async_trait::async_trait;
use actix_web::{get,web::{self,Data, ServiceConfig,scope},  HttpResponse,  Responder};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use std::{time::Duration, io::Read};


const  HOMEPAGE: &str = "homepage";



#[get("/index")]
async fn index() -> impl Responder {
    let path: PathBuf = format!("static/{}/templates/index.html",HOMEPAGE).into();
    let mut  file_content = NamedFile::open(path).ok().unwrap();
    let mut buffer = String::new();
    file_content.read_to_string(&mut buffer).ok();
    HttpResponse::Ok().body(buffer)
}




// this function could be located in a different module
fn config1(cfg: &mut web::ServiceConfig) {
    println!("serving from ./static/{}/styles",HOMEPAGE);
    cfg.service(fs::Files::new("/static/styles", format!("./static/{}/styles",HOMEPAGE))
                .show_files_listing()
                .use_last_modified(true)
            )
    .service(
        fs::Files::new("/static/templates", format!("./static/{}/templates",HOMEPAGE))
        .show_files_listing()
        .use_last_modified(true))
    .service(
        fs::Files::new("/static/imgs", format!("./static/{}/imgs",HOMEPAGE))
        .show_files_listing()
        .use_last_modified(true))
    .service(index);
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