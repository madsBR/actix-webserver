use actix_web::web::{Data, ServiceConfig,scope};
use actix_web::{get,post, web::{self,ReqData, Json,Redirect,redirect}, App, HttpResponse, HttpServer, Responder};
use image::{ImageFormat,load_from_memory_with_format};
use actix_files as fs;
use app_plugin::AppPlugin;
use async_trait::async_trait;
use homepage::HPConfig;
use vcg_app::VcgAppConfig;
use log::{log_enabled,info};
use std::env;
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



#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    configure_log();
    info!("Initializing web server");
    HttpServer::new(|| {
        App::new()
            .service(web::scope(HPConfig::SCOPE)
                .configure(HPConfig::config_w_files)
            ).service(web::scope(VcgAppConfig::SCOPE)
            .configure(VcgAppConfig::config_w_files)
        )

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
