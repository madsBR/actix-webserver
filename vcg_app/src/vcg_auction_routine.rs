use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize,Serialize};
use std::{time::{Instant,Duration}, fmt::Display, vec};
use std::fmt::Debug;
use vcg_auction::vcg_base_types::{Price,Good,Player, VCGOutput,Pairing, GoodWPrice};
use ndarray::{Array1,Array2};
use crate::vcg_auction_owner::VCG_Auction_Owner;
use crate::ext_types::*;
use log::{debug,log};
use env_logger::{Builder, Target};

trait Game{
    const MaxPlayerNr : usize;
    const MaxGoodNr : usize;  
}

struct Other{}
struct Gaia{}



impl Game for Other{
    const MaxGoodNr : usize = 16;
    const MaxPlayerNr : usize = 8;
}

impl Game for Gaia{
    const MaxGoodNr : usize = 8;
    const MaxPlayerNr : usize = 8;
}





pub struct ClientBidInfo{
    id : ID,
    bid_buffer : Vec<(Player,Good,Price)>,
    created_at : Instant,
    metadata : ContentMetaData,
}


impl From<BidPostBackContent> for ClientBidInfo{
    fn from(content: BidPostBackContent) -> Self {
        let metadata = ContentMetaData::from_content(&content);
        let instant = Instant::now();
        let bid_buffer = content.bid_pairings;
        ClientBidInfo { 
            id: content.id.unwrap_or_else(ID::new_random),
            bid_buffer: bid_buffer.into_iter().filter_map(|bid_pairing| BidPostBackContent::parse_bid_pairing(bid_pairing)).collect(),
            created_at: instant,
            metadata : metadata,
        }
    }
}



impl ClientBidInfo{
    pub fn construct_vcg_auction(&self) -> VCG_Auction_Owner{
        let nr_goods = self.metadata.goods.len();
        let nr_players = self.metadata.players.len();
        let masks = Array2::<usize>::from_diag(&Array1::ones(nr_goods));
        let mut bids_buffer_to_array = vec![0;nr_players * nr_goods];
        for (pl,good,pr) in self.bid_buffer.iter(){
            bids_buffer_to_array[nr_goods * usize::from(*pl) + usize::from(*good)] = (*pr).into();
        }       
        let bids = Array2::<usize>::from_shape_vec((nr_players,nr_goods), bids_buffer_to_array).unwrap();
        VCG_Auction_Owner::new(bids, masks)  
    }
}





pub fn vcg_routine(content : BidPostBackContent) -> VCGOutputContent{
    let bid_info = ClientBidInfo::from(content);
    debug!("buffer is {:?}",bid_info.bid_buffer);
    
    let vcg_auction = bid_info.construct_vcg_auction();
    debug!("buffer is {:?}",vcg_auction.bids);
    debug!("masks are {:?}",vcg_auction.masks);
    
    let output = vcg_auction.perform_vcg();
    let mut buf = output.into_buffer();
    debug!("buffer is now {:?}",buf);
    let output_buffer = bid_info.metadata.pairings_int_to_ext(&mut buf);
    return VCGOutputContent{id : bid_info.id, output : output_buffer};
}

#[derive(Serialize)]
pub struct VCGOutputContent{
    pub id : ID,
    pub output : Vec<OutputPairing>,
}


#[cfg(test)]
mod tests {
    use vcg_auction::vcg_base_types::Pairing;
    use crate::{vcg_auction_routine::{OutputPairing, GoodWPriceExt, PlayerExt,Color}, ext_types::GoodExt};

    use super::{VCGOutputContent, ID,Serialize};
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
