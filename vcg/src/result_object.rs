use crate::ext_types::{OutputPairing,InputRawPairing};
use serde::{Deserialize,Serialize,Deserializer};


#[derive(Debug,Serialize,Deserialize,PartialEq,Eq,Clone)]
pub struct ResultObject{
    pub html : String,
    pub auction_result : Vec<OutputPairing>,
    pub input_matrix : Vec<InputRawPairing>,
}