
use actix_web::web::{ServiceConfig};
use actix_web::{get,post, HttpResponse, Responder};



use app_plugin::AppPlugin;
use async_trait::async_trait;

use crate::bid_post_back_content::BidPostBackContent;

use crate::vcg_auction_routine::{vcg_routine};
use crate::client_bid_info::ClientBidInfo;
use crate::scope::SCOPE;




use crate::result_page::{VCGResultTemplate};
use askama::Template;



use crate::index_template::IndexTemplate;



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
    println!("content receieved : {}",content);
    let content: BidPostBackContent = serde_json::from_str(&content).unwrap();
    log::debug!("received bids {:?}",content);
    match ClientBidInfo::try_from(content){
        Ok(cli_bid_info) => {    
            log::debug!("succesfully formatted postback into client bid info");
        let resp_content = vcg_routine(cli_bid_info);
            let page_result = VCGResultTemplate::new().render();
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

    use regex::Regex;

    use super::*;

    #[test]
    fn deser_col() {
        let json_string = r#"{"str": "ABC123"}"#;
        let color_wrap: Color = serde_json::from_str(json_string).unwrap();
        assert_eq!(color_wrap.str,"ABC123");
    }

    #[test]
    fn deser_good() {
        let _good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".try_into().unwrap() };
        let json = r#"
        {
            "id": 24,
            "name": "Example Good",
            "color": {"str": "ABC123"}
        }
        "#;
        let v : GoodExt = serde_json::from_str(json).unwrap();
        assert_eq!(v.color.str,"ABC123");
        let _good : GoodExt = GoodExt { id: 2, name: "hej".to_string(), color:  "#00FA00".try_into().unwrap() };
        let json = r#"
        {
            "id": 24,
            "name": "Example Good",
            "color": {"str": "ABC123"}
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
    fn parse_content_json(){
        let content_json = r##"{
            "id":8295960347037019,
            "player_nr":2,
            "pls":[{"id":0,"name":"a"},{"id":1,"name":"b"}],
            "goods":[{
                "id":0,"name":"Firaks","color":{"str":"#808B96"}},
                {"id":1,"name":"Ivits","color":{"str":"#FF0000"}},
                {"id":2,"name":"Terran","color":{"str":"#0000FF"}},
                {"id":3,"name":"Xenon","color":{"str":"#FFFF00"}},
                {"id":4,"name":"Geoden","color":{"str":"#F39c12"}},
                {"id":5,"name":"Itars","color":{"str":"#FFFFFF"}},
                {"id":6,"name":"none","color":{"str":"#FFFFFF"}}],
                "bid_pairings":[[0,0,2],[0,1,4],[1,0,3],[1,1,5]]}"##;
    let content: Result<BidPostBackContent,serde_json::Error> = serde_json::from_str(content_json);
    assert!(content.is_ok(),"there was an error in reading json {}. ERROR received: {}",content_json,content.err().unwrap().to_string());
            
    }
    #[test]
    fn test_regex(){
        let RE: Regex = Regex::new(r"^(#)?[0-9a-fA-F]+$").unwrap();
        assert!(RE.is_match("#3aBf33"));
    }
}
