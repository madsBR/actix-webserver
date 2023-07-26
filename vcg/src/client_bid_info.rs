
use vcg_auction::vcg_base_types::{Price,Good,Player, Pairing, GoodWPrice};
use serde::{Deserialize,Serialize,Deserializer};
use tinyvec::TinyVec;
use std::fmt::Debug;
use regex::Regex;
use lazy_static::lazy_static;
use crate::ext_types::{Color,BidPostBackContent,PlayerExt,GoodExt,GoodWPriceExt,OutputPairing,ID};
use std::{time::{Instant,Duration}, fmt::Display, vec,error};

pub struct ContentMetaData{
    pub players : TinyVec<[PlayerExt;5]>,
    pub goods : TinyVec<[GoodExt;10]>,
    pub bid_nr : usize
}

impl ContentMetaData{
    pub fn from_content(content : &BidPostBackContent) -> Result<Self,String>{        
        let mut pl_ids : Vec<usize> = Vec::new();
        let mut good_ids : Vec<usize> = Vec::new();        
        let mut bid_counter : usize = 0;
        for (pl_ext,good_ext) in content.bid_pairings.iter().filter_map(|(x,y,_)| if y.is_some(){Some((*x,y.unwrap()))} else {None} ){
            pl_ids.push(pl_ext);
            good_ids.push(good_ext);
            bid_counter += 1;
        }
        pl_ids.sort();
        pl_ids.dedup();
        good_ids.sort();
        good_ids.dedup();

        let mut players : TinyVec<[PlayerExt;5]> = TinyVec::with_capacity(pl_ids.len());
        let mut goods : TinyVec<[GoodExt;10]> = TinyVec::with_capacity(good_ids.len());
        for (ind,pl) in pl_ids.iter().enumerate(){
            if let Some(pl_ext) = content.pls.iter().find(|ext| ext.id == *pl){
                players.push(pl_ext.clone());                
            }
            else{
                let x = format!("A bid came from urecognized player {}",pl);
                return(Err(x));
            }
        }
        for (ind,good) in good_ids.iter().enumerate(){
            if let Some(good_ext) = content.goods.iter().find(|ext| ext.id == *good){
                goods.push(good_ext.clone());                
            }
            else{
                let x = format!("A bid came for an urecognized good {}",good);
                return(Err(x)); 
            }
        }
 
        Ok(Self { players: players, goods: goods,bid_nr : bid_counter })

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

pub struct ClientBidInfo{
    pub id : ID,
    pub bid_buffer : Vec<(Player,Good,Price)>,
    pub created_at : Instant,
    pub metadata : ContentMetaData,
}


impl TryFrom<BidPostBackContent> for ClientBidInfo{
    type Error = String;
    fn try_from(content: BidPostBackContent) -> Result<Self,Self::Error> {
        let metadata_mb = ContentMetaData::from_content(&content);
        match metadata_mb{
            Ok(metadata) => {
                
                let instant = Instant::now();
                let bid_pair_builder = BidPairingBuilder::new(&metadata, content.bid_pairings);
                let cli_bid_info = ClientBidInfo { 
                    id: content.id.unwrap_or_else(ID::new_random),
                    bid_buffer: bid_pair_builder.parse_bid_pairings(),
                    created_at: instant,
                    metadata : metadata,
                };
                return(Ok(cli_bid_info))
            }
            Err(str) => {
                return(Err(str));
            }

        }
    }
}

impl ClientBidInfo{

}

struct BidPairingBuilder<'a>{            
    metadt : &'a ContentMetaData,
    result : Vec<(Player,Good,Price)>,
    content_bid_pairs : Vec<(usize,Option<usize>,usize)>,
    pl_ind : usize,
    good_ind : usize,
}



impl<'a> BidPairingBuilder<'a>{
    fn new(metadt : &'a ContentMetaData,content_bid_pairs : Vec<(usize,Option<usize>,usize)>) -> Self{
        Self { metadt: metadt, result: Vec::with_capacity(content_bid_pairs.len()), content_bid_pairs: content_bid_pairs, pl_ind: 0, good_ind: 0 }
    }

    fn parse_bid_pairings(mut self) -> Vec<(Player,Good,Price)>{
        self.content_bid_pairs.sort();
        for index in 0..self.content_bid_pairs.len() {
            if let (pl_id, Some(good_id), pr) = self.content_bid_pairs[index] {
                self.update_pl_good(pl_id, good_id);
                self.result.push((self.pl_ind.into(),self.good_ind.into(),pr.into()));      
            }
        }
        self.result
    }    

    fn update_pl_good(&mut self,pl_id : usize, good_id : usize){
        if(self.metadt.players[self.pl_ind].id != pl_id){
            self.good_ind = 0;
            self.pl_ind+=1;
        }
        while(self.metadt.players[self.pl_ind].id != pl_id){
            self.pl_ind+=1;
        }

        while(self.good_ind < self.metadt.goods.len()){
            if self.metadt.goods[self.good_ind].id != good_id{
                self.good_ind+=1;
            } else{
                break
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::{ext_types::{GoodExt, PlayerExt, BidPostBackContent}, test_data::test_utils::check_vec};
    use tinyvec::{TinyVec,tiny_vec};
    use super::*;
    use crate::test_data::test_utils::{get_test_data_valid,get_test_data_bad_pl,get_test_data_bad_good};
    #[test]
    fn test_create_metadata_bad_pl(){
    let (id,pls,goods,bids) = get_test_data_bad_good();
    let content = BidPostBackContent{
        id : id,
        player_nr : pls.len() as u64,
        pls: pls,
        goods : goods,
        bid_pairings : bids
    };
    let metadata = ContentMetaData::from_content(&content);
    assert!(metadata.is_err());
    }
    fn test_create_metadata_bad_good(){
        let (id,pls,goods,bids) = get_test_data_bad_good();
        let content = BidPostBackContent{
            id : id,
            player_nr : pls.len() as u64,
            pls: pls,
            goods : goods,
            bid_pairings : bids
        };
        let metadata = ContentMetaData::from_content(&content);
        assert!(metadata.is_err());
    }



    fn test_create_metadata_valid(){
        let (id,pls,goods,bids) = get_test_data_valid();
        let content = BidPostBackContent{
            id : id,
            player_nr : pls.len() as u64,
            pls: pls,
            goods : goods,
            bid_pairings : bids
        };
        let metadata = ContentMetaData::from_content(&content);
        assert!(metadata.is_ok());
    }

    
    

    #[test]
    fn test_create_bid_info(){
        let (id,pls,goods,bids) = get_test_data_valid();
        let content = BidPostBackContent{
            id : id,
            player_nr : pls.len() as u64,
            pls: pls,
            goods : goods,
            bid_pairings : bids
        };
        let client_bid_info_mb = ClientBidInfo::try_from(content);
        assert!(client_bid_info_mb.is_ok());
    let client_bid_info = client_bid_info_mb.unwrap(); 

    let metadata = client_bid_info.metadata;
    let mut x = client_bid_info.bid_buffer.clone();
    x.sort();
    let exp_vec_pre = vec![
        (0,Some(0),4),(0,Some(1),6),(0,Some(2),7),
        (1,Some(0),4),(1,Some(1),5),(1,Some(2),6),
        (2,Some(0),3),(2,Some(1),5),(2,Some(2),8),
        (3,Some(0),4),(3,Some(1),5),(3,Some(2),6),
        
    ];

    let exp_vec : Vec<(Player,Good,Price)> = exp_vec_pre.into_iter().map(|(x,y,z)| (Player{val : x},Good{val : y.unwrap()},Price{val : z})).collect();
    check_vec(x, exp_vec);

    }
    
}