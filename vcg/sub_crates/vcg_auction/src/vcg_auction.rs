use crate::vcg_compute::{VCG_Computer,VCG_Computer_Out};
use crate::iterator_as::*;
use crate::vcg_base_types::{Good,Player, VCGOutput, Price, Pairing,GoodWPrice};
use ndarray::{Axis,ArrayView2};
use itertools::Itertools;
use rustc_hash::FxHashMap;



pub struct VCG_Auction<'a>{
    masks : ArrayView2<'a,usize>,
    bids : ArrayView2<'a,usize>,
    nr_goods : usize,
    nr_players : usize,
    auctions : FxHashMap<Vec<Player>,VCG_Computer_Out>,
}




impl<'a> VCG_Auction<'a>{

    pub fn new(masks : ArrayView2<'a,usize>, bids : ArrayView2<'a,usize>) -> Self{
        let nr_goods = bids.ncols();
        let nr_players = bids.nrows();
        Self { masks : masks, bids : bids, nr_goods: nr_goods, nr_players: nr_players, auctions: FxHashMap::default()}
    }

    pub fn run(&mut self)-> VCGOutput{
        let mut output = self.compute_output_pre_alt();
        self.assign_sub_prices(&mut output);
        output

    }

    fn compute_output_pre_alt(&mut self) -> VCGOutput {
        //Computes VCG output before alternative totals has been subtracted
        let sub_nr_players = self.nr_goods.min(self.nr_players);
        for pl_combi in (0..self.nr_players).into_iter_as::<Player>().combinations(sub_nr_players){
            let usize_combi = pl_combi.into_iter_as::<usize>().collect_vec();
            let sub_masks = self.masks;
            let sub_bids = self.bids.select(Axis(0), &usize_combi);
            let pl_combi = usize_combi.into_iter_as().collect_vec();
            self.auctions.insert(pl_combi, VCG_Computer::new(sub_nr_players, self.nr_goods, sub_masks.view(), sub_bids.view()).compute_into_out());

        }
        println!("auctions are{:?}",self.auctions);
        let winner_combi = self.get_best_sub_auction();
        println!("WINNER IS output{:?}",winner_combi);
        winner_combi        
    }   

    fn assign_sub_prices_unsaturated(&self,vcg_output : &mut VCGOutput){
        //if self.nr_goods<self.nr_players
        
        for ind in 0..vcg_output.nr_players(){
            let winning_pl = vcg_output[ind].pl;
            let alt_cost = (0..self.nr_players).into_iter_as::<Player>().filter(|pl| *pl!= winning_pl).combinations(self.nr_goods)
                .map(|alt_combi| self.auctions[&alt_combi].best_bid_sum)
                .max().unwrap();
            vcg_output[ind].bought_good.unwrap().price = alt_cost - vcg_output[ind].bought_good.unwrap().price;
        }
    }

    fn assign_sub_prices_saturated(&self,vcg_output : &mut VCGOutput){
        //if self.nr_goods>=self.nr_players
        for ind in 0..vcg_output.nr_players(){
            let winning_pl = vcg_output[ind].pl;
            let alt_cost = (0..self.nr_players).into_iter_as::<Player>().filter(|pl| *pl!= winning_pl).combinations(self.nr_players - 1).map(
                |alt_combi| {
                    let pls_vec = alt_combi.into_iter_as::<usize>().collect_vec();
                    let sat_sub_masks= self.masks;
                    let sat_sub_bids = self.bids.select(Axis(0), &pls_vec);
                    let mut sub_computer = VCG_Computer::new(self.nr_players-1, self.nr_goods, sat_sub_masks.view(), sat_sub_bids.view());
                    sub_computer.compute();
                    let compute_out = sub_computer.into_out_with_player_mapping(&pls_vec.into_iter_as().collect());
                    //println!("comp out are now{:?}",compute_out);
                    let price = compute_out.best_bid_sum;
                    return price;
            }).max().unwrap();
            vcg_output[ind].bought_good = Some(GoodWPrice{
                good: vcg_output[ind].bought_good.unwrap().good,
                price : alt_cost - vcg_output[ind].bought_good.unwrap().price}
            );            
        }
    }   

    #[inline]
    fn is_saturated(&self) -> bool{
        self.nr_goods>=self.nr_players
    }

    pub fn assign_sub_prices(&self,vcg_output : &mut VCGOutput){
        if self.is_saturated(){
            self.assign_sub_prices_saturated(vcg_output);
        } else{
            self.assign_sub_prices_unsaturated(vcg_output);
        }
        
    }

            /*
             .max_by_key(
               move |alt_combi| {
                let vec_p = (*alt_combi).into_iter_as::<usize>().collect_vec();
                let sat_sub_masks = self.masks.select(Axis(0),&vec_p);
                let sat_sub_bids = self.bids.select(Axis(0), &vec_p);                
                let compute_out = VCG_Computer::new(self.nr_players-1, self.nr_goods, sat_sub_masks.view(), sat_sub_bids.view()).into_out_with_player_mapping(&vec_p.into_iter_as().collect_vec());

                });
            */
     



    fn get_best_sub_auction(&self) -> VCGOutput{
        //returns reference to winners vec in auction hashmap and best vcg_output PRE subtracting alt gains
        let mut winners :Option<&Vec<Player>> = None;
        let mut best_pairings : Option<&Vec<(Player,Good)>> = None;
        let mut max_price :Price = 0.into();
        for (players,auc_res_out) in self.auctions.iter(){
            if auc_res_out.best_bid_sum>=max_price{
                winners = Some(players);
                best_pairings = Some(&auc_res_out.best_pairings);
                max_price = auc_res_out.best_bid_sum;                
            }
        }
        
        let output = VCGOutput::new(
            best_pairings.unwrap().iter().zip(winners.unwrap()).map(
                    |((_,good),wnr)| Pairing::new( *wnr, *good, max_price - self.bids[(usize::from(*wnr),usize::from(*good))].into())
                    ).collect_vec()
                );

            
        output
    }

}

#[cfg(test)]
mod vcg_auction_tests {
    use crate::vcg_auction::{*};
    #[test]
    fn test_base_vcg_wo_mask() {
        let nr_players = 3usize; let nr_goods = 4usize;
        let masks = Array2::from_diag(&Array1::ones(nr_goods));
        let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
        [4,1,9,0,
         5,1,8,5,
         5,1,4,0]).unwrap();
        let mut auction = VCG_Auction::new( masks.view(), bids.view());
        let mut output = auction.compute_output_pre_alt();
        let pre_price = 9+5+5;
        assert_eq!(output[0].bought_good, Some(GoodWPrice{good : Good{val : 2},price : Price{val : pre_price-9}}));
        assert_eq!(output[1].bought_good, Some(GoodWPrice{good : Good{val : 3},price : Price{val : pre_price-5}}));
        assert_eq!(output[2].bought_good, Some(GoodWPrice{good : Good{val : 0},price : Price{val : pre_price-5}}));

    auction.assign_sub_prices(&mut output);
        assert_eq!(output[0].bought_good, Some(GoodWPrice{good : Good{val : 2},price : Price{val : 3}}));
        assert_eq!(output[1].bought_good, Some(GoodWPrice{good : Good{val : 3},price : Price{val : 0}}));
        assert_eq!(output[2].bought_good, Some(GoodWPrice{good : Good{val : 0},price : Price{val : 0}}));


    }   

    #[test]
    fn test_case2() {
        let nr_players = 2usize; let nr_goods = 7usize;
        let masks = Array2::from_diag(&Array1::ones(nr_goods));
        let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
        [
            0,4,0,0,0,3,0,
            0,4,0,0,0,3,0
        ]).unwrap();
        let mut auction = VCG_Auction::new( masks.view(), bids.view());
        let mut output = auction.compute_output_pre_alt();
        auction.assign_sub_prices(&mut output);
        assert_eq!(output[0].bought_good, Some(GoodWPrice{good : Good{val : 1},price : Price{val : 1}}));
        assert_eq!(output[1].bought_good, Some(GoodWPrice{good : Good{val : 5},price : Price{val : 0}}));        
    }

    // #[test]
    // fn test_base_vcg_wo_mask_good_eq_pl() {
    //     let nr_players = 3usize; let nr_goods = 3usize;
    //     let masks = Array2::from_diag(&Array1::ones(nr_goods));
    //     let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
    //     [2,6,0,
    //      5,6,8,
    //      5,1,4]).unwrap();
    //     let mut vcg_comp = VCG_Computer::new(nr_players, nr_goods, masks.view(), bids.view());
    //     assert_eq!(vcg_comp.lagged_bid_sum,2+6);
    //     assert_eq!(vcg_comp.mask_stack.to_vec(),vec![1,1,0]);

    //     vcg_comp.compute_2p_players();
    //     assert_eq!(vcg_comp.best_bid_sum , 6+8+5);
    //     assert_eq!(vcg_comp.best_pairings[0], Some(Good{val : 1}));
    //     assert_eq!(vcg_comp.best_pairings[1], Some(Good{val : 2}));
    //     assert_eq!(vcg_comp.best_pairings[2], Some(Good{val : 0}));
    // }


}