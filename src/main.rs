use actix_web::web::{Data, ServiceConfig,scope};
use actix_web::{get,post, web::{self,ReqData, Json,Redirect,redirect}, App, HttpResponse, HttpServer, Responder};
use image::{ImageFormat,load_from_memory_with_format};
use reqwest::StatusCode;
use std::{time::Duration, io::Read};
use std::path::PathBuf;
use std::path;
use actix_files as fs;
use actix_files::NamedFile;
use std::collections::HashMap;
use std::fmt::format;
use serde::Serialize;
 //   let path: PathBuf = "static/templates/menu.html".parse().unwrap();
    
//    let dirs = path:: 
const  test_scope: &str = "scope1";



#[get("/menu")]
async fn menu() -> impl Responder {
    let path: PathBuf = "static/templates/index.html".parse().unwrap();
    let stuff = NamedFile::open(path).ok().unwrap();
    stuff 
}

#[get("/a")]
async fn hello() -> impl Responder {
    "hello hello"
}

#[get("/index")]
async fn index() -> impl Responder {

    let path: PathBuf = format!("{}/static/templates/index.html",test_scope).into();
    let mut  file_content = NamedFile::open(path).ok().unwrap();
    let mut buffer = String::new();
    file_content.read_to_string(&mut buffer).ok();
    HttpResponse::Ok().body(buffer)
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


async fn dl_euler_plate() -> std::io::Result<()>{
    let bytes_dl = reqwest::get(
            "https://projecteuler.net/profile/madsBR.png"
        ).await.ok().unwrap() 
        .bytes()
        .await.ok().unwrap();
    let byte_vec = bytes_dl.as_ref();
    let img = load_from_memory_with_format(&byte_vec,ImageFormat::Png).ok().unwrap();
    img.save("euler.png");
    Ok(())

}



// this function could be located in a different module
fn config2(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scope2")
        .service(index)        
    );
}

// this function could be located in a different module
fn config1(cfg: &mut web::ServiceConfig) {
    println!("serving from ./{}/static/styles",test_scope);
    cfg
    .service(web::scope(test_scope)
        .service(
            fs::Files::new("/static/styles", format!("./{}/static/styles",test_scope))
                    .show_files_listing()
                    .use_last_modified(true)
                )
        .service(
            fs::Files::new("/static/templates", format!("./{}/static/templates",test_scope))
            .show_files_listing()
            .use_last_modified(true))
            .service(
                fs::Files::new("/static/templates", format!("./{}/static/imgs","base"))
                .show_files_listing()
                .use_last_modified(true))
    
        .service(
            fs::Files::new("/static/imgs", format!("./{}/static/imgs",test_scope))
            .show_files_listing()
            .use_last_modified(true))
        .service(index)
        .service(
            web::resource("/app")
                .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
    ));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config1)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
