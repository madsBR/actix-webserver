use vcg_auction::vcg_base_types::{Price};
use serde::{Deserialize,Serialize,Deserializer};
use std::{fmt::Display};

use std::fmt::Debug;
use regex::Regex;
use lazy_static::lazy_static;


pub type InputRawPairing =(usize,Option<usize>,usize);

fn is_valid_hexadecimal(input: &str) -> bool {

    lazy_static! {        
        static ref RE: Regex = Regex::new(r"^(#)?[0-9a-fA-F]+$").unwrap();
    }    
    RE.is_match(input)
}


#[derive(Debug,Serialize,Deserialize,PartialEq,Eq,Default,Clone,PartialOrd, Ord)]
pub struct PlayerExt{
    pub id : usize,
    pub name : String
}

impl PlayerExt{
    pub fn new(pl_id : usize, name : &str) -> Self{
        Self { id: pl_id, name: name.into() }
    }
}

#[derive(Debug,PartialEq, Eq,Clone,Serialize,Deserialize,Default,PartialOrd, Ord)]
pub struct GoodExt{
    pub id :usize,
    pub name : String,
    pub color : Color,
}


#[derive(Debug,Serialize,Deserialize,PartialEq,Eq,Default,Clone)]
pub struct GoodWPriceExt{
    pub good : GoodExt,
    pub price : Price
}



#[derive(Debug,Serialize,PartialEq,Eq,Default,Clone,PartialOrd, Ord,Deserialize)]
pub struct Color {
    pub str: String,
}

impl TryFrom<&str> for Color{
    type Error = String;
    fn try_from(hexa_decimal: &str) -> Result<Self, Self::Error> {
        if is_valid_hexadecimal(hexa_decimal){
            Ok(Color { str: hexa_decimal.to_string() })
        }

        else{
            let mut err_msg = "this string is not valid hexadecimal".to_string();
            err_msg.push_str(hexa_decimal);
            Err(err_msg)
        }
    }
}

/*
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        let str_value: &str = Deserialize::deserialize(deserializer)?;
        if is_valid_hexadecimal(str_value){
            Ok(Color {
                str: str_value.to_owned(),
            })
        }
        else{
            Err(serde::de::Error::custom(format!("The received hexadecimal {} is invalid",{str_value})))
        }
    }
}
 */



impl Color{

     pub fn from_rgb(r : u8, g : u8, b : u8) -> Self{
        let z = 256 * (b as u32) + 16 * (g as u32) + r as u32;
        Self{str : format!("#{:06x}",z).to_uppercase()}
     }

     pub fn to_rgb(&self) -> (u8,u8,u8){
        let r = u8::from_str_radix(&self.str[1..3],16).unwrap();
        let g = u8::from_str_radix(&self.str[3..5],16).unwrap();
        let b = u8::from_str_radix(&self.str[5..7],16).unwrap();
        (r,g,b)
        
     }
    
    // pub fn into_u32(&self) -> u32{
    //     256 * (self.b as u32) + 16 * (self.g as u32) + self.r as u32
    // }

    // pub fn get_hex_code(&self) ->String{

    //     format!("#{:06x}",self.into_u32()).to_uppercase()
    // }


    pub fn get_matching_font_col(&self) -> Self{
        let (r,g,b) = self.to_rgb();
        if (r as f32 *0.299 + g as f32 *0.587 + b as f32 * 0.114) > 186.{
            "#000000".try_into().unwrap()
        } else{
            "#FFFFFF".try_into().unwrap()
        }
    }
}



#[derive(Debug,Serialize,Deserialize,PartialEq,Eq,Default,Clone)]
pub struct OutputPairing{
    pub pl : PlayerExt,
    pub good_color_price : Option<GoodWPriceExt>,    
}





#[derive(Deserialize,Serialize,Clone, Copy,PartialEq, Eq)]
pub struct ID(u128);

impl ID{
    pub fn new_random() -> Self{
        Self(fastrand::u128(1<<127..))
    }
}



impl Display for ID{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("0x{:x}",self.0))
    }
}



impl Debug for ID{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("0x{:x}",self.0))
    }
}






