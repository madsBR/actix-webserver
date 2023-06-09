use actix_web::dev::Service;
use actix_web::web::{Data, ServiceConfig,scope};
use actix_web::{get,post, web::{self,ReqData, Json,Redirect,redirect}, App, HttpResponse, HttpServer, Responder};
use image::{ImageFormat,load_from_memory_with_format};
use actix_files as fs;
use actix_files::{NamedFile};
use std::io::Read;
use app_plugin::AppPlugin;
use async_trait::async_trait;
use std::path::PathBuf;
use crate::ext_types::{BidPostBackContent,Color};
use crate::vcg_auction_routine::{self, vcg_routine};
use serde::{Deserialize,Serialize};
use vcg_auction::vcg_base_types::{VCGOutput, Pairing};
use vcg_auction_routine::{ClientBidInfo,VCGOutputContent};
use std::env;
use regex::Regex;
use crate::result_page::{VCGResultTemplate};
use askama::Template;
use askama::Error as AskaErr;
use log::debug;
use crate::index_template::IndexTemplate;


const SCOPE : &'static str = "vcg_app";
pub struct VcgAppConfig{}


#[get("/index")]
async fn index() -> impl Responder {
    let page = IndexTemplate::new(SCOPE).render();
    let response = match page{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    };
    response
}





#[post("/index/submit_bids")]
async fn submit_bids(content : String) -> impl Responder {
    let content: BidPostBackContent = serde_json::from_str(&content).unwrap();
    log::debug!("received bids {:?}",content);
    let resp_content = vcg_routine(content);
    let page_result = VCGResultTemplate::from(&resp_content).render();
    let response = match page_result{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    };
    log::debug!("responding {:?}",response);
    response
}




#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body(format!("pong from {}",VcgAppConfig::SCOPE))
}


#[async_trait]
impl AppPlugin for VcgAppConfig {
    
    const SCOPE : &'static str = SCOPE;
    async fn scheduled_process(&self){}
    fn config(cfg : &mut ServiceConfig ){
        cfg
        .service(ping)
        .service(index)
        .service(submit_bids)
        ;
    }
}


    // cfg.service(fs::Files::new("/static/styles", format!("./static/{}/styles",HOMEPAGE))
    //             .show_files_listing()
    //             .use_last_modified(true)
    //         )
    // .service(
    //     fs::Files::new("/static/templates", format!("./static/{}/templates",HOMEPAGE))
    //     .show_files_listing()
    //     .use_last_modified(true))
    // .service(
    //     fs::Files::new("/static/imgs", format!("./static/{}/imgs",HOMEPAGE))
    //     .show_files_listing()
    //     .use_last_modified(true))
//     .service(index);




#[cfg(test)]
mod tests {
    use crate::ext_types::{GoodExt, PlayerExt};


    #[derive(Debug, Deserialize)]
    pub struct ColorWrap {
        col: Color
    }

    use super::*;

    #[test]
    fn deser_col() {
        let json_string = r#"{"col": "ABC123"}"#;
        let color_wrap: ColorWrap = serde_json::from_str(json_string).unwrap();
        println!("{:?}", color_wrap);
    }

    #[test]
    fn deser_good() {
        let good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".to_string().try_into().unwrap() };
        let json = r#"
        {
            "id": 24,
            "name": "Example Good",
            "color": "ABC123"
        }
        "#;
        let v : GoodExt = serde_json::from_str(json).unwrap();
        assert_eq!(v.color.str,"ABC123");
        let good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".to_string().try_into().unwrap() };
        let json = r#"
        {
            "id": 24,
            "name": "Example Good",
            "color": "ABC123"
        }
        "#;
        let v : GoodExt = serde_json::from_str(json).unwrap();
        assert_eq!(v.color.str,"ABC123");

        let json_pl = r#"
        {
            "id": 24,
            "name": "Example Name"
        }
        "#;
        let v : PlayerExt = serde_json::from_str(json_pl).unwrap();
        assert_eq!(v.name,"Example Name");



    }

    #[test]
    fn test_regex(){
        let RE: Regex = Regex::new(r"^(#)?[0-9a-fA-F]+$").unwrap();
        assert!(RE.is_match("#3aBf33"));
    }
}
