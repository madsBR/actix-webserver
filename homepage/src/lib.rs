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


mod index_template;
pub mod hp_app_plugin;
mod cv_template;
pub use hp_app_plugin::HPConfig;

const  HOMEPAGE: &str = "homepage";