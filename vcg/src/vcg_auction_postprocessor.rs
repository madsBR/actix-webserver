
use itertools::Itertools;
use crate::{client_bid_info::ClientBidInfo, ext_types::{OutputPairing, GoodExt, PlayerExt, GoodWPriceExt}};
use rand::prelude::*;


pub struct VCGPostProcessor<'a>{
    distribute_leftovers : bool,
    vcg_output : Vec<OutputPairing>,
    cli_info : &'a ClientBidInfo,
}

impl<'a> VCGPostProcessor<'a>{

    pub fn new(distribute_leftovers : bool, vcg_output : Vec<OutputPairing>, cli_info : &'a ClientBidInfo) -> Self{
        VCGPostProcessor{
            distribute_leftovers,
            vcg_output,
            cli_info,
        }
    }

    fn assign_left_overs(&mut self){
        let leftover_pls_nr = self.cli_info.metadata.players_total.len()- self.vcg_output.len();
        let leftover_goods_nr = self.cli_info.metadata.players_total.len()- self.vcg_output.len();
        let mut assigns_remaining = leftover_goods_nr.min(leftover_pls_nr); 
        let mut leftover_goods = self.get_leftover_goods();
        println!("leftover goods are {:?}",leftover_goods);
        leftover_goods.shuffle(&mut thread_rng());
        let mut inp_pls = self.cli_info.metadata.players_total.iter();
        while assigns_remaining>0 {
            let pl = inp_pls.next().unwrap();
            if !self.vcg_output.iter().any(|x| &x.pl == pl){
                self.assign(pl, &leftover_goods.pop().unwrap(), &mut assigns_remaining);
            }
            
        }
    }
    
    fn assign(&mut self, pl : &PlayerExt, good: &GoodExt,assign_counter : &mut usize){
        println!("assigning {:?} to {:?}",pl,good);
        self.vcg_output.push(OutputPairing { pl: pl.clone(), good_color_price: Some(GoodWPriceExt{good : good.clone(), price : 0.into()}) });
        *assign_counter-=1;
    }

    pub fn process(mut self)-> Vec<OutputPairing>{
        self.assign_left_overs();
        self.vcg_output
    }

    fn pairs_has_good(good_ext : &GoodExt,vec : &[OutputPairing]) -> bool{
        vec.iter().any(|x| x.good_color_price.as_ref().unwrap().good.id == good_ext.id)
    }

    fn get_leftover_goods(&self) -> Vec<GoodExt>{
        let mut leftover_goods = self.cli_info.metadata.goods_total.iter().filter(|x|!Self::pairs_has_good(x,&self.vcg_output)).cloned().collect_vec();
                
        leftover_goods.shuffle(&mut thread_rng());
        leftover_goods
    }
}

//is_some_and(|b_good| b_good.good.id == good_in.id)





#[cfg(test)]
mod vcg_auction_tests {
    use tinyvec::{tiny_vec, TinyVec};
    use vcg_auction::iterator_as::IntoIteratorAsTr;
    use vcg_auction::vcg_base_types::{Player,Good,Price, Pairing};
    use std::vec::Vec;
    use std::vec;
    use crate::ext_types::{self, ID, PlayerExt, GoodExt,Color, OutputPairing};
    use crate::bid_post_back_content::BidPostBackContent;
    use crate::client_bid_info::ClientBidInfo;
    use crate::test_data::test_utils::{get_test_data_bad_good, check_vec, get_test_data_valid,generate_test_data};
    use crate::vcg_auction_postprocessor::VCGPostProcessor;
    #[test]
    fn test_postprocess() {
        
        let (inp,out_bef_proc) = generate_test_data(
            (0..=4).collect(),
            (2..=6).collect(),
            vec![
                4,2,8,9,1,
                2,5,3,6,2,
                0,0,9,7,3,
                0,7,6,9,4,
                3,5,2,1,5,
            ],
            vec![
                (1,Some(3),0),
                (2,Some(4),3),
                (3,Some(5),4), 
                (4,Some(6),5),

            ],
             false);
        


        
        let content = BidPostBackContent{
            id : inp.0,
            player_nr : inp.1.len() as u64,
            goods: inp.2,
            pls : inp.1,
            bid_pairings : inp.3,
        };           
        let client_bid_info = ClientBidInfo::try_from(content).unwrap();
        let post_proc : Vec<OutputPairing> = VCGPostProcessor { distribute_leftovers: true, vcg_output: out_bef_proc.clone(), cli_info: &client_bid_info }.process();
    

        assert!(out_bef_proc.iter().all(|elem_bef_proc|post_proc.contains(elem_bef_proc)));
        assert!(post_proc.iter().any(|pair| pair.pl.id == 0 && pair.good_color_price.as_ref().is_some_and(|x| x.good.id == 2)));


    }
   

}
