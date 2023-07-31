use ndarray::{Array1,Array2};
use vcg_auction::vcg_auction::VCG_Auction;
use vcg_auction::vcg_base_types::VCGOutput;

use crate::client_bid_info::ClientBidInfo;
pub struct VCG_Auction_Owner{
    pub bids : Array2<usize>,
    pub masks: Array2<usize>,
}

impl VCG_Auction_Owner {
    pub fn perform_vcg(&self) -> VCGOutput{
        let mut vcg_auction = VCG_Auction::new(self.masks.view(), self.bids.view());
        let res = vcg_auction.run();
        res
    }
    
    pub fn new(bids : Array2<usize>, masks : Array2<usize>)-> Self{
        VCG_Auction_Owner { bids: bids, masks: masks }
    }

    pub fn from_bid_info<'a>(bid_info : &'a ClientBidInfo) -> Self{
        let nr_goods = bid_info.metadata.goods_active.len();
        let nr_players = bid_info.metadata.players_active.len();
        let masks = Array2::<usize>::from_diag(&Array1::ones(nr_goods));
        let mut bids_buffer_to_array = vec![0;nr_players * nr_goods];
        for (pl,good,pr) in bid_info.bid_buffer.iter(){
            bids_buffer_to_array[nr_goods * usize::from(*pl) + usize::from(*good)] = (*pr).into();
        }       
        let bids = Array2::<usize>::from_shape_vec((nr_players,nr_goods), bids_buffer_to_array).unwrap();
        VCG_Auction_Owner::new(bids, masks)  
    }
}

