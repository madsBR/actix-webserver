#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    unused_import_braces,
    dead_code,
    clippy::redundant_redundant_fields,
)]

pub mod routes;
mod vcg_auction_routine;
pub use routes::VcgAppConfig;
pub mod vcg_input_analyzer;
mod vcg_auction_owner;
mod result_page;
mod ext_types;
mod index_template;
mod client_bid_info;
mod bid_post_back_content;
mod vcg_auction_postprocessor;

#[cfg(test)]
mod test_data;
