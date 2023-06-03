mod client_buffer;
use actix_web::dev::Service;
use actix_web::web::{Data, ServiceConfig,scope};
use actix_web::{get,post, web::{self,ReqData, Json,Redirect,redirect}, App, HttpResponse, HttpServer, Responder};
use image::{ImageFormat,load_from_memory_with_format};
use actix_files as fs;
use app_plugin::AppPlugin;
use async_trait::async_trait;


pub struct VcgAppConfig{}

#[async_trait]
impl AppPlugin for VcgAppConfig {
    const SCOPE : &'static str = "vcg_app";
    async fn scheduled_process(&self){}
    fn config(cfg : &mut ServiceConfig ){}
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
