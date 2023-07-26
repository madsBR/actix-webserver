use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize,Serialize};
use std::{time::{Instant,Duration}, fmt::Display, vec};
use std::fmt::Debug;
use vcg_auction::vcg_base_types::{Price,Good,Player, VCGOutput,Pairing, GoodWPrice};
use ndarray::{Array1,Array2};
use crate::{ext_types::*, vcg_auction_owner::VCG_Auction_Owner};
use crate::client_bid_info::ClientBidInfo;
use log::{debug,log};
use env_logger::{Builder, Target};











pub fn vcg_routine(bid_info : ClientBidInfo) -> VCGOutputContent{
    let id = bid_info.id;
    let vcg_auction = VCG_Auction_Owner::from_bid_info(&bid_info);
    debug!("buffer is {:?}",vcg_auction.bids);
    debug!("masks are {:?}",vcg_auction.masks);    
    let output = vcg_auction.perform_vcg();
    let mut buf = output.into_buffer();
    debug!("buffer is now {:?}",buf);
    let output_buffer = bid_info.metadata.pairings_int_to_ext(&mut buf);
    return VCGOutputContent{id : id, output : output_buffer};
}


#[derive(Serialize)]
pub struct VCGOutputContent{
    pub id : ID,
    pub output : Vec<OutputPairing>,
}


#[cfg(test)]
mod vcg_auction_tests {
    use tinyvec::{tiny_vec, TinyVec};
    use vcg_auction::iterator_as::IntoIteratorAsTr;
    use vcg_auction::vcg_base_types::{Player,Good,Price, Pairing};

    use crate::ext_types::{self, ID, PlayerExt, GoodExt,Color};
    use crate::ext_types::BidPostBackContent;
    use crate::vcg_auction_routine;

    use super::*;

    #[test]
    fn it_works() {
        let x = Pairing::new(5.into(), 3.into(), 2.into());
        let good = GoodExt{ id : 3,name : "some good".to_string(), color : Color { str: "FF00AB".to_string() }};
        let x = OutputPairing{ pl: PlayerExt::new(3, "Joe") ,good_color_price : Some(GoodWPriceExt{good : good,price : 5.into()}) };
        let vec = vec![x];
        let vcgout = VCGOutputContent{ id : ID::new_random(),output : vec};
        let z = serde_json::to_string(&vcgout);        
        println!("{}",z.unwrap())
    }


}