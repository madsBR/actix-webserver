#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    unused_import_braces,
    dead_code,
    clippy::redundant_field_names,
)]

use actix_web::web::{Data, ServiceConfig,scope};
use actix_web::{middleware as mw,get,post, web::{self,ReqData, Json,Redirect,redirect}, App, HttpResponse, HttpServer, Responder};
use image::{ImageFormat,load_from_memory_with_format};
use actix_files as fs;
use app_plugin::AppPlugin;

use async_trait::async_trait;
use homepage::HPConfig;
use vcg::VcgAppConfig;
use log::{log_enabled,info};
use std::env;
use std::fmt::format;
use env_logger::{Builder, Target};
use app_plugin::logger::configure_log;

//    let dirs = path:: 

// async fn dl_euler_plate() -> std::io::Result<()>{
//     let bytes_dl = reqwest::get(
//             "https://projecteuler.net/profile/madsBR.png"
//         ).await.ok().unwrap() 
//         .bytes()
//         .await.ok().unwrap();
//     let byte_vec = bytes_dl.as_ref();
//     let img = load_from_memory_with_format(&byte_vec,ImageFormat::Png).ok().unwrap();
//     img.save("euler.png");
//     Ok(())
// }


#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}


trait MyTrait {
    fn my_function(&self);
}


macro_rules! add_plugins_to_app {
    ($app:expr, $($list_ty:ty),*) => {
        $(
               $app = $app.configure(<$list_ty>::config_w_files); // Create an instance of the type  
        )*
    };
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    configure_log();
    info!("Initializing web server");
    HttpServer::new(|| {
        let mut app = 
        App::new()
        .wrap(mw::NormalizePath::new(mw::TrailingSlash::Trim))
        .service(web::redirect("/",format!("/{}/{}",HPConfig::SCOPE,HPConfig::ROOT_REDIR)));          //MAIN PAGE If just going to www.madsraad.com/*
        add_plugins_to_app!(app,
            HPConfig,
            VcgAppConfig
        );        
        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
