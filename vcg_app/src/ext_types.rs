use vcg_auction::vcg_base_types::{Price,Good,Player, VCGOutput,Pairing, GoodWPrice};
use serde::{Deserialize,Serialize,Deserializer, ser::SerializeStruct};
use std::{time::{Instant,Duration}, fmt::Display, vec};
use tinyvec::TinyVec;
use std::fmt::Debug;
use regex::Regex;
use lazy_static::lazy_static;
use serde_json::Value;


fn is_valid_hexadecimal(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(#)?[0-9a-fA-F]+$").unwrap();
    }    
    RE.is_match(input)
}


#[derive(Debug,Serialize,Deserialize,PartialEq,Eq,Default,Clone)]
pub struct PlayerExt{
    pub id : usize,
    pub name : String
}

impl PlayerExt{
    pub fn new(pl_id : usize, name : &str) -> Self{
        Self { id: pl_id, name: name.into() }
    }
}

#[derive(Debug,PartialEq, Eq,Clone,Serialize,Deserialize,Default)]
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



#[derive(Debug,Serialize,PartialEq,Eq,Default,Clone)]
pub struct Color {
    pub str: String,
}

impl TryFrom<String> for Color{
    type Error = String;
    fn try_from(hexa_decimal: String) -> Result<Self, Self::Error> {
        if is_valid_hexadecimal(&hexa_decimal){
            Ok(Color { str: hexa_decimal })
        }

        else{
            let mut err_msg = "this string is not valid hexadecimal".to_string();
            err_msg.push_str(&hexa_decimal);
            Err(err_msg)
        }
    }
}


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
            "#000000".to_owned().try_into().unwrap()
        } else{
            "#FFFFFF".to_owned().try_into().unwrap()
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
        Self{0 : fastrand::u128(1<<127..)}
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




#[derive(Deserialize, Serialize,Debug)]
pub struct BidPostBackContent {
    pub id : Option<ID>,
    pub player_nr : u64,
    pub pls : TinyVec<[PlayerExt;5]>,
    pub goods : TinyVec<[GoodExt;10]>,
    pub bid_pairings: Vec<(usize,usize,usize)>,
}


pub struct ContentMetaData{
    pub players : TinyVec<[PlayerExt;5]>,
    pub goods : TinyVec<[GoodExt;10]>,
}

impl ContentMetaData{
    pub fn from_content(content : &BidPostBackContent) -> Self{
        let mut players = TinyVec::<[PlayerExt;5]>::from_iter(content.pls.iter().cloned());
        let mut goods = TinyVec::<[GoodExt;10]>::from_iter(content.goods.iter().cloned());
        Self { players: players, goods: goods }
    }
    #[inline]
    pub fn player_int_to_ext<'a>(&'a self,pl : Player) -> &'a PlayerExt{
        &self.players[usize::from(pl)]
    }

    #[inline]
    pub fn good_int_to_ext<'a>(&'a self,good : Good) -> &'a GoodExt{
        &self.goods[usize::from(good)]
    }

    #[inline]
    pub fn some_pairing_int_to_ext(&self, pl : Player,good_w_price : GoodWPrice) -> OutputPairing {
        let good_ext = self.good_int_to_ext(good_w_price.good);

        OutputPairing { pl: self.player_int_to_ext(pl).clone(), good_color_price: Some(
            GoodWPriceExt {
                good: good_ext.clone(),
                price: good_w_price.price 
            }
        )}                
    }

    #[inline]
    pub fn pairing_int_to_ext(&self,pair : Pairing) -> OutputPairing{
        let Pairing{pl : pl,bought_good : bought_good} = pair;
        let pl_ext =self.player_int_to_ext(pl);
        match bought_good {
            Some(GoodWPrice{good : good, price : price }) => self.some_pairing_int_to_ext(pl, GoodWPrice { good: good, price: price }),
            None => OutputPairing { pl: self.player_int_to_ext(pl).clone(), good_color_price : None}
        }
    }

    pub fn pairings_int_to_ext(&self,vec : &mut Vec<Pairing>) -> Vec<OutputPairing>{
        vec.iter().map(|x| self.pairing_int_to_ext(*x)).collect()
    }
}




