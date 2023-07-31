use async_trait::async_trait; //should be temporary until async fn is stable
use actix_web::web::{ServiceConfig,scope, redirect};
pub mod logger;
use actix_files as fs;




#[async_trait] // Should be temporary until async fn is stable
pub trait AppPlugin {
    const  SCOPE: &'static str; 
    const  ROOT_REDIR: &'static str; 
    async fn scheduled_process(&self);
    fn config(cfg: &mut ServiceConfig);

    fn config_w_files<'a>( cfg: &'a mut ServiceConfig){
        cfg
        .service(redirect(format!("/{}",Self::SCOPE),format!("/{}/{}",Self::SCOPE,Self::ROOT_REDIR)))
        .service(
             scope(format!("/{}",Self::SCOPE).as_str())
            .configure(Self::config_w_files_internal)
        )
        ;
    }

    fn config_w_files_internal<'a>( cfg: &'a mut ServiceConfig){
        cfg.service(fs::Files::new("/static/styles", format!("./static/{}/styles",Self::SCOPE))
        .show_files_listing()
        .use_last_modified(true))
        .service(
        fs::Files::new("/static/templates", format!("./static/{}/templates",Self::SCOPE))
        .show_files_listing()
        .use_last_modified(true))
        .service(fs::Files::new("/static/js", format!("./static/{}/js",Self::SCOPE))
        .show_files_listing()
        .use_last_modified(true))
        .service(fs::Files::new("/static/imgs", format!("./static/{}/imgs",Self::SCOPE))
        .show_files_listing()
        .use_last_modified(true))
        .service(fs::Files::new("/static/base_styles", format!("./static/{}/styles","base"))
        .show_files_listing()
        .use_last_modified(true))
        .service(fs::Files::new("/static/base_js", format!("./static/{}/js","base"))
        .show_files_listing()
        .use_last_modified(true))
        .service(fs::Files::new("/static/base_imgs", format!("./static/{}/imgs","base"))
        .show_files_listing()
        .use_last_modified(true));
        Self::config(cfg);
    }

    fn redir_scoped<'a> (cfg: &'a mut ServiceConfig,from : &str, to : &str) -> &'a mut ServiceConfig{
        cfg.service(redirect(format!("/{}/{}",Self::SCOPE,from), format!("/{}/{}",Self::SCOPE,to)));            
        cfg
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
