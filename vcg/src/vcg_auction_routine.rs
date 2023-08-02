
use serde::{Serialize};

use crate::{ext_types::*, vcg_auction_owner::VCG_Auction_Owner};
use crate::client_bid_info::ClientBidInfo;
use log::{debug};

use crate::vcg_auction_postprocessor::VCGPostProcessor;









pub fn vcg_routine(bid_info : ClientBidInfo) -> VCGOutputContent{
    let id = bid_info.id;
    let vcg_auction = VCG_Auction_Owner::from_bid_info(&bid_info);
    debug!("buffer is {:?}",vcg_auction.bids);
    debug!("masks are {:?}",vcg_auction.masks);    
    let output = vcg_auction.perform_vcg();

    let output_buffer = bid_info.metadata.pairings_int_to_ext(&mut output.into_buffer());
    
    let output_buff_processed = VCGPostProcessor::new(true,output_buffer,&bid_info).process();
    VCGOutputContent{id, output : output_buff_processed}
    
}


#[derive(Serialize)]
pub struct VCGOutputContent{
    pub id : ID,
    pub output : Vec<OutputPairing>,
}

#[cfg(test)]
mod vcg_auction_tests {
    use itertools::Itertools;
    use tinyvec::{tiny_vec, TinyVec};
    use vcg_auction::iterator_as::IntoIteratorAsTr;
    use vcg_auction::vcg_base_types::{Player,Good,Price, Pairing};
    use std::vec::Vec;
    use std::vec;
    use crate::ext_types::{self, ID, PlayerExt, GoodExt,Color};
    use crate::bid_post_back_content::BidPostBackContent;
    use crate::test_data::test_utils::{get_test_data_bad_good, check_vec, get_test_data_valid,generate_test_data};
    use crate::vcg_auction_routine;

    use super::*;

    #[test]
    fn it_works() {
        use crate::test_data::test_utils::{get_test_data_valid,};
        let _x = Pairing::new(5.into(), 3.into(), 2.into());
        let good = GoodExt{ id : 3,name : "some good".to_string(), color : Color { str: "FF00AB".to_string() }};
        let x = OutputPairing{ pl: PlayerExt::new(3, "Joe") ,good_color_price : Some(GoodWPriceExt{good,price : 5.into()}) };
        let vec = vec![x];
        let vcgout = VCGOutputContent{ id : ID::new_random(),output : vec};
        let z = serde_json::to_string(&vcgout);        
    }

    pub fn is_sorted<T: IntoIterator>(t: T) -> bool
    where<T as IntoIterator>::Item: std::cmp::PartialOrd,
    {
        let mut iter = t.into_iter();
        if let Some(first) = iter.next() 
        {
            iter.try_fold(first, |previous, current| 
            {
                (previous <= current).then_some(current)
            })
            .is_some()
        } else {
            true
        }
    }

    
    #[test]
    fn is_sorted_works() {
        assert!(is_sorted(vec![2,3,4,5,5,6,7,8,9]));
        assert!(!is_sorted(vec![3,2,4,5,6,7,8,9]));
        assert!(!is_sorted(vec![2,3,4,5,6,1,8,9]));

    }



    #[test]
    fn test_auction_goods() {
        let (id,pls,good,pairs) = get_test_data_valid();
        let content = BidPostBackContent{
            id,
            player_nr : pls.len() as u64,
            goods: good,
            pls,
            bid_pairings : pairs,
        };

        let (client_bid_info,_bids) = content.validate_and_unpack().unwrap();
        let output = vcg_auction_routine::vcg_routine(client_bid_info);
        let output_pl_ids : Vec<(usize,Option<usize>)> = output.output.iter().map(|x| {
            match &x.good_color_price {
                Some(good_w_pr) => (x.pl.id,Some(good_w_pr.good.id)),
                None => (x.pl.id,None),
            }
        }).collect();
        let exp_vec : Vec<(usize,Option<usize>)> = vec![
            (1,Some(3)),
            (3,Some(4)),
            (4,Some(2)),
        ];
        let exp_leftovers = vec![(2,Some(1)),(2,Some(5)),(2,Some(6))];
        assert_eq!(output_pl_ids.len(),4);
        assert!(exp_vec.iter().all(|exp_elem|output_pl_ids.contains(exp_elem)));
        assert!(exp_leftovers.iter().any(|exp_elem|output_pl_ids.contains(exp_elem)));
        
    }

    
    #[test]
    fn test_auction1() {
        
        let (inp,out_exp) = generate_test_data(
            (1..=4).collect(),
            (3..=6).collect(),
            vec![
                4,2,8,9,
                2,5,3,6,
                0,0,9,7,
                0,7,6,9,
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
        let (client_bid_info,_bids) = content.validate_and_unpack().unwrap();
        let output = vcg_auction_routine::vcg_routine(client_bid_info);        
        check_vec(output.output, out_exp)
    }
   

    #[test]
    fn test_auction2() {
        let (inp,out_exp) = generate_test_data(
            (1..=4).collect(),
            (3..=7).collect(),
            vec![
                4,2,6,4,8,
                5,7,3,2,0,
                7,6,0,2,0,
                5,7,0,4,0,
            ],
            vec![
                (1,Some(7),0),
                (2,Some(4),3),
                (3,Some(3),1),
                (4,Some(6),0),

            ],
             false);
        

        let content = BidPostBackContent{
            id : inp.0,
            player_nr : inp.1.len() as u64,
            goods: inp.2,
            pls : inp.1,
            bid_pairings : inp.3,
        };           
        let (client_bid_info,_bids) = content.validate_and_unpack().unwrap();
        let output = vcg_auction_routine::vcg_routine(client_bid_info);        
        check_vec(output.output, out_exp)
    }


    #[test]
    fn test_auction_order_bids() {
        let (inp,out_exp) = generate_test_data(
            (1..=4).rev().collect(),
            (3..=7).rev().collect(),
            vec![
                4,2,6,4,8,
                5,7,3,2,0,
                7,6,0,2,0,
                5,7,0,4,0,
            ],
            vec![
                (1,Some(4),0),
                (2,Some(7),1),
                (3,Some(6),3),
                (4,Some(3),0),

            ],
             false);
        

        let content = BidPostBackContent{
            id : inp.0,
            player_nr : inp.1.len() as u64,
            goods: inp.2,
            pls : inp.1,
            bid_pairings : inp.3,
        };           
        let (client_bid_info,bids) = content.validate_and_unpack().unwrap();
        assert!(is_sorted(bids.as_ref()));
        assert!(is_sorted(&client_bid_info.bid_buffer));
        let output = vcg_auction_routine::vcg_routine(client_bid_info);        
        check_vec(output.output, out_exp)
    }



    #[test]
    fn test_auction3() {
        let (inp,out_exp) = generate_test_data(
            (1..=4).collect(),
            (3..=7).collect(),
            vec![
                6,4,4,2,8,
                3,2,5,7,0,
                0,2,7,6,0,
                0,4,5,7,0,
            ],
            vec![
                (1,Some(7),0),
                (2,Some(6),3),
                (3,Some(5),1),
                (4,Some(4),0),

            ],
             false);
        

        let content = BidPostBackContent{
            id : inp.0,
            player_nr : inp.1.len() as u64,
            goods: inp.2,
            pls : inp.1,
            bid_pairings : inp.3,
        };           
        let (client_bid_info,_bids) = content.validate_and_unpack().unwrap();
        let output = vcg_auction_routine::vcg_routine(client_bid_info);        
        check_vec(output.output, out_exp)
    }

    #[test]
    fn test_auction4() {
        let (inp,out_exp) = generate_test_data(
            (0..=3).map(|x| 3*x).collect(),
            (0..=4).map(|x| 2*x).collect(),
            vec![
                6,4,4,2,8,
                3,2,5,7,0,
                0,2,7,6,0,
                0,4,5,7,0,
            ],
            vec![
                (0,Some(8),0),
                (3,Some(6),3),
                (6,Some(4),1),
                (9,Some(2),0),

            ],
             false);
        

        let content = BidPostBackContent{
            id : inp.0,
            player_nr : inp.1.len() as u64,
            goods: inp.2,
            pls : inp.1,
            bid_pairings : inp.3,
        };           
        let (client_bid_info,_bids) = content.validate_and_unpack().unwrap();
        let output = vcg_auction_routine::vcg_routine(client_bid_info);        
        check_vec(output.output, out_exp)
    }


}