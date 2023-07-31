
use actix_web::web::{ServiceConfig};
use actix_web::{get,post, HttpResponse, Responder};



use app_plugin::AppPlugin;
use async_trait::async_trait;

use crate::bid_post_back_content::BidPostBackContent;

use crate::vcg_auction_routine::{vcg_routine};
use crate::client_bid_info::ClientBidInfo;





use crate::result_page::{VCGResultTemplate};
use askama::Template;



use crate::index_template::IndexTemplate;


const SCOPE : &str = "vcg";

pub struct VcgAppConfig{}
const ROOT_REDIR : &str = "app";
#[get("/app")]
async fn app() -> impl Responder {
    let page = IndexTemplate::new(SCOPE).render();    
    
    match page{
     Ok(page) => HttpResponse::Ok().body(page),
    _ => HttpResponse::InternalServerError().into(),
    }
}




/*
TO ENSURE FOLLOWING:
If player in bid pool then player in pls vector.
MAYBE: If player in pls vector then pls should get assigned a good as if plaer bidded "less than 0"
*/
#[post("/app/submit_bids")]
async fn submit_bids(content : String) -> impl Responder {
    let content: BidPostBackContent = serde_json::from_str(&content).unwrap();
    log::debug!("received bids {:?}",content);
    match ClientBidInfo::try_from(content){
        Ok(cli_bid_info) => {    
            log::debug!("succesfully formatted postback into client bid info");
        let resp_content = vcg_routine(cli_bid_info);
            let page_result = VCGResultTemplate::from(&resp_content).render();
            let response = match page_result{
            Ok(page) => HttpResponse::Ok().body(page),
            _ => HttpResponse::InternalServerError().into(),
            };
            log::debug!("responding {:?}",response);
            response
        }
        Err(str) => {
            log::debug!("Failed to translate postback content to client bid info");
            HttpResponse::BadRequest().body(str)
        }
    }
}




#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body(format!("pong from {}",VcgAppConfig::SCOPE))
}


#[async_trait]
impl AppPlugin for VcgAppConfig {
    
    const SCOPE : &'static str = SCOPE;
    const ROOT_REDIR : &'static str = ROOT_REDIR;
    async fn scheduled_process(&self){}
    fn config(cfg : &mut ServiceConfig ){
        cfg
        .service(ping)
        .service(app)
        .service(submit_bids);
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
    use crate::ext_types::{GoodExt, PlayerExt,Color};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct ColorWrap {
        col: Color
    }
    use regex::Regex;

    use super::*;

    #[test]
    fn deser_col() {
        let json_string = r#"{"col": "ABC123"}"#;
        let color_wrap: ColorWrap = serde_json::from_str(json_string).unwrap();
        println!("{:?}", color_wrap);
    }

    #[test]
    fn deser_good() {
        let good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".try_into().unwrap() };
        let json = r#"
        {
            "id": 24,
            "name": "Example Good",
            "color": "ABC123"
        }
        "#;
        let v : GoodExt = serde_json::from_str(json).unwrap();
        assert_eq!(v.color.str,"ABC123");
        let good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".try_into().unwrap() };
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
