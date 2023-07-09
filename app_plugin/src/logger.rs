use log::{LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::env;
pub fn configure_log() {
    let level = if env::var("BRANCH").is_ok() {LevelFilter::Info} else{ LevelFilter::Debug};
    let file_appender = FileAppender::builder().build("log/app.log").unwrap();    
    let config = Config::builder()
        .appender(Appender::builder().build("file_appender", Box::new(file_appender)))
        .build(Root::builder().appender("file_appender").build(level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    
}