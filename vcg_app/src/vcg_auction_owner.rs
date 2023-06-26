use ndarray::{Array2};
use vcg_auction::vcg_auction::VCG_Auction;
use vcg_auction::vcg_base_types::VCGOutput;
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
}

