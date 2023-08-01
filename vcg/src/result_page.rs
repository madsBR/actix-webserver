use askama::Template;
use crate::{vcg_auction_routine::{VCGOutputContent}, ext_types::{OutputPairing, GoodWPriceExt}};
use crate::scope::SCOPE;
#[derive(Template)]
#[template(path="result_page.html",escape="none")]
pub struct VCGResultTemplate {
   scope : &'static str
}

impl VCGResultTemplate{
   pub fn new() -> Self{
      Self { scope: SCOPE }
   }
}

/* 
#[derive(Template)]
#[template(path="row.txt")]
struct Row<'a>  {
   name: &'a str,
   good: &'a str,
   price : usize,
   bg_color : &'a str,
   font_color : String,
}

impl<'a> From<&'a OutputPairing> for Row<'a>{
   fn from(value: &'a OutputPairing) -> Self {
      match &value.good_color_price
      {
         Some(GoodWPriceExt{good, price}) => Row { name: &value.pl.name, good: &good.name, price: (*price).into(), bg_color: &good.color.str , font_color : good.color.get_matching_font_col().str},
         None                                                            => Row{ name : &value.pl.name, good : "none", price : 0,  bg_color : "#FFFFFF", font_color : "#000000".to_owned()}         
      }
   }
}



*/