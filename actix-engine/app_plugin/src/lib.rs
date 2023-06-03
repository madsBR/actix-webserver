use std::marker::PhantomData;
use async_trait::async_trait; //should be temporary until async fn is stable
use actix_web::web::{Data, ServiceConfig,scope};


#[async_trait] // Should be temporary until async fn is stable
pub trait AppPlugin {
    const  SCOPE: &'static str; 
    async fn scheduled_process(&self);
    fn config(cfg: &mut ServiceConfig);    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
