use itertools::Itertools;
use ndarray::{Array1,s, Axis, ArrayView2};
use crate::vcg_base_types::{Player,Good,VCGOutput, Price};
use crate::iterator_as::{IteratorAsTr,IntoIteratorAsTr};



#[derive(Debug)]
pub struct VCG_Computer_Out{
   pub best_bid_sum : Price,
   pub best_pairings : Vec<(Player,Good)>,
}


impl<'a> From<VCG_Computer<'a>> for VCG_Computer_Out{
    fn from(vcg_computer: VCG_Computer) -> Self {
        Self { best_bid_sum: vcg_computer.best_bid_sum.into(), best_pairings: vcg_computer.best_pairings.into_iter().enumerate().take_while(|(_ind,x)| x.is_some()).map(|(x,y)| (x.into(),y.unwrap())).collect_vec() }        
    }
}

pub struct VCG_Computer<'a>{
    pub last_player : usize,
    pub nr_goods : usize,
    lagged_bid_sum : usize,
    bids : ArrayView2<'a,usize>,
    masks : ArrayView2<'a,usize>,
    mask_stack : Array1<usize>,
    pub best_bid_sum : usize,
    pub best_pairings : [Option<Good>;Player::MAX_PLAYERS],
    lagged_pairing_status : [Good;Player::MAX_PLAYERS],
}





impl<'a> VCG_Computer<'a>{
    pub fn new(nr_players : usize, nr_goods : usize,masks : ArrayView2<'a,usize>,bids : ArrayView2<'a,usize>) -> Self{
        let last_player = nr_players -1;
        let lagged_pairing_status : [Good;Player::MAX_PLAYERS]= [Good{val : 0};Player::MAX_PLAYERS];
        let mask_stack = Array1::zeros(nr_goods);
        let best_pairings = [None;Player::MAX_PLAYERS];
        //println!("constructed with mask_stack {:?}, masks {:?} and current pairing status {:?} bids are {:?}",mask_stack,masks,lagged_pairing_status.iter().map(|x| x.val).collect_vec(),bids);

        Self{best_pairings: best_pairings , nr_goods : nr_goods, lagged_pairing_status , mask_stack : mask_stack , masks : masks, bids : bids, lagged_bid_sum : 0, best_bid_sum : 0 , last_player : last_player}
    }


    fn next_unmasked_good_for_player(&self,pl : &Player) -> Option<Good>{
        (self.lagged_pairing_status[pl.val].val + 1..self.nr_goods).zip(self.mask_stack.slice(s![self.lagged_pairing_status[pl.val].val +1..]).iter()).find(
            |(_good,mask_val)| **mask_val==0).map(|(good_val,_)|Good{val : good_val})
    }


    fn first_unmasked_good(&self) -> Good{
        (0..self.nr_goods).zip(self.mask_stack.iter()).find(
            |(_good,mask_val)| **mask_val==0).map(|(good_val,_)|Good{val : good_val}).unwrap()
    }

    fn reset(&mut self){
        self.mask_stack.assign(&self.masks.slice(s![..self.last_player,..]).sum_axis(Axis(0)));        
        self.lagged_pairing_status.iter_mut().enumerate().take(self.last_player).map(|(i,good)| {*good = Good{val : i};}).last();
        self.best_pairings.fill(None);
        self.lagged_bid_sum = self.bids.slice(s![0..self.last_player,..]).diag().iter().sum();
    }


    fn is_stack_top(&self,pl : Player) -> Option<Good>
    //is stack top if not none, in which case Some(good) is next good for stack top
    {
        if let Some(next_good) = self.next_unmasked_good_for_player(&pl){
            println!("get_stack_top:: Found stack top {} and next good {:?}", usize::from(pl),next_good);
            return Some(next_good);
        } else{
            None
        }
    }

    fn decrement_masks_and_bids_to_stack_top(&mut self) -> Option<(usize,Good)>{
        //Decrement TO w/o including Stack top
        println!("Decre_masks:::mask stack is {}",self.mask_stack);
        for pl in (0..self.last_player).rev(){
            if let Some(next_good) = self.is_stack_top(pl.into()){
                println!("Decre_masks::found top stack : player {} ",{pl});
               
                return Some((pl,next_good))
            } else{
                println!("Decre_masks::removing for player {} ",pl);
                self.remove_masks_and_bid_on_stack(self.good_of_pl(pl.into()), &pl.into())            
            }
            
        }   
        println!("Decre_masks:: ENDED mask stack is {}",self.mask_stack);      
        None
    }

    fn good_of_pl(&self, pl : Player) -> Good {
        self.lagged_pairing_status[pl.val]
    }

    fn increment_masks_and_bids_and_update_goods(&mut self,stack_top : usize){
        println!("incr masks :: started with {}. mask stack is {}",stack_top,self.mask_stack);
        for pl in stack_top + 1..self.last_player{
            let first_available_good = self.next_unmasked_good_for_player(&pl.into()).unwrap_or_else(|| self.first_unmasked_good());
            self.lagged_pairing_status[pl]  = first_available_good;

            self.put_masks_and_bid_on_stack(first_available_good, &pl.into());
            println!(" incr masks ::: pl {} has new good{} mask stack is {}", pl,first_available_good.val,self.mask_stack);

        }
        println!("incr masks ::done");         
    }



    fn assign_good_to_stack_top(&mut self,stack_top : usize,good : Good){
        //assumes stack top is not at end

        self.remove_masks_and_bid_on_stack(self.good_of_pl(stack_top.into()), &stack_top.into());
        self.lagged_pairing_status[stack_top] = good;
        self.put_masks_and_bid_on_stack(good, &stack_top.into());
        println!("inc_stack_top :: incrementing pl {} to good {}. Lagged bid sum is now {}",stack_top,self.lagged_pairing_status[stack_top].val,self.lagged_bid_sum);
    }

    fn run_through_last_player(&mut self){
  //      println!("running through last player");
        for (good,(bid,mask_val)) in self.bids.slice(s![self.last_player,..]).iter().zip(self.mask_stack.iter()).enumerate(){
      
            if *mask_val == 0{
         //       println!("found legit good {}",good);
                if bid + self.lagged_bid_sum>self.best_bid_sum{
                    self.best_bid_sum = bid + self.lagged_bid_sum;
                    self.best_pairings[self.last_player] = Some(good.into());

                    (0..self.last_player).map(|pl| { self.best_pairings[pl] = Some(self.lagged_pairing_status[pl])}).last();
                }
            } 
     //       println!("AFT good is = {} bid is {}, best_bid is {}, current bid {} and best bid parings is{:?}",good,bid,self.best_bid_sum,self.lagged_bid_sum,self.best_pairings.iter().map(|x| *x).collect_vec());
       //     println!("current is is {:?}",self.lagged_pairing_status.iter().map(|x| x.val).collect_vec());
        }
    }

                

    fn compute_2p_players(&mut self){
        println!("-----------------STARTING-----------------");
        println!("{}",self.bids);
        self.mask_stack +=  &self.masks.slice(s![0,..]);
        self.lagged_bid_sum += self.bids[(0,0)];
        
        self.assign_good_to_stack_top(0, Good { val: 0 });       
        self.increment_masks_and_bids_and_update_goods(0);
        self.run_through_last_player();
        
        while let Some((stack_top,next_good)) = self.decrement_masks_and_bids_to_stack_top(){            
            println!("start of loop: stack top is now {}, mask stack is {} and status is {:?}",stack_top,self.mask_stack,self.lagged_pairing_status.iter().map(|x| usize::from(*x)).collect_vec());            
            self.assign_good_to_stack_top(stack_top, next_good); 
            self.increment_masks_and_bids_and_update_goods(stack_top);
            println!("Bef Run_through : mask stack is {} and status is {:?}",self.mask_stack,self.lagged_pairing_status.iter().map(|x| usize::from(*x)).collect_vec());
            self.run_through_last_player();
            println!("ending loop: max is now {}, stack top was{} and mask stack is {} with best being {:?}",self.best_bid_sum,stack_top,self.mask_stack,self.best_pairings.iter().map(|x| if x.is_some(){usize::from(x.unwrap())} else {0}).collect_vec());
        }
        println!("ENDED MAX FINDING");

    }

    fn compute_1_player(&mut self){
        let (good_ind,best_bid) = self.bids.iter().enumerate().fold((usize::MAX,0usize), | (mxind,mx),(ind,elem)| if mx<*elem {(ind,*elem)} else {(mxind,mx)});
        self.best_bid_sum = best_bid;
        self.best_pairings[0] = Some(Good{val : good_ind});
    }

    pub fn compute(&mut self){
        if self.last_player == 0{
            self.compute_1_player();
        }
        else {
            self.compute_2p_players();
        }
    }


    pub fn into_out_with_player_mapping(self,pls : &Vec<Player>) -> VCG_Computer_Out{
        VCG_Computer_Out {
            best_bid_sum : self.best_bid_sum.into(),
            best_pairings : self.best_pairings.into_iter().zip(pls).filter(|(good,_pl)| good.is_some()).map(|(good,pl)| (*pl,good.unwrap())).collect_vec() 
        }
    }

    pub fn compute_into_out(mut self) -> VCG_Computer_Out{
        self.compute_2p_players();
        self.into()
    }

    fn put_masks_and_bid_on_stack(&mut self,good : Good, pl : &Player){

        self.mask_stack +=  &self.masks.slice(s![good.val,..]);
        self.lagged_bid_sum += self.bids[(pl.val,good.val)];
        println!("put_masks :: incremented pl {:?} to good {:?}, bid sum is currently {}",pl,good,self.lagged_bid_sum)

    }
    fn remove_masks_and_bid_on_stack(&mut self,good : Good, pl : &Player){
        println!("remove_masks:: about to subtrack {} from {}",self.masks.slice(s![good.val,..]),self.mask_stack);
        self.mask_stack -=  &self.masks.slice(s![good.val,..]);
        self.lagged_bid_sum -= self.bids[(pl.val,good.val)];
        println!("remove_masks:: subtracted good {} from pl {}. mask: {} now bid sum is down to {}",usize::from(good),usize::from(*pl) as usize,self.mask_stack, self.lagged_bid_sum);
    
    }
}




#[cfg(test)]
mod vcg_compute_tests {
    use crate::vcg_compute::{*};
    #[test]
    fn test_base_vcg_wo_mask() {
        let nr_players = 3usize; let nr_goods = 4usize;
        let masks = Array2::from_diag(&Array1::ones(nr_goods));
        let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
        [4,1,9,0,
         5,1,8,5,
         5,1,4,0]).unwrap();
        let mut vcg_comp = VCG_Computer::new(nr_players, nr_goods, masks.view(), bids.view());
        assert_eq!(vcg_comp.lagged_bid_sum,5);
        assert_eq!(vcg_comp.mask_stack.to_vec(),vec![1,1,0,0]);

        vcg_comp.compute_2p_players();
        assert_eq!(vcg_comp.best_bid_sum , 9usize+5+5);
        assert_eq!(vcg_comp.best_pairings[0], Some(Good{val : 2}));
        assert_eq!(vcg_comp.best_pairings[1], Some(Good{val : 3}));
        assert_eq!(vcg_comp.best_pairings[2], Some(Good{val : 0}));
    }   

    #[test]
    fn test_base_vcg_wo_mask_good_eq_pl() {
        let nr_players = 3usize; let nr_goods = 3usize;
        let masks = Array2::from_diag(&Array1::ones(nr_goods));
        let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
        [2,6,0,
         5,6,8,
         5,1,4]).unwrap();
        let mut vcg_comp = VCG_Computer::new(nr_players, nr_goods, masks.view(), bids.view());
        assert_eq!(vcg_comp.lagged_bid_sum,2+6);
        assert_eq!(vcg_comp.mask_stack.to_vec(),vec![1,1,0]);

        vcg_comp.compute_2p_players();
        assert_eq!(vcg_comp.best_bid_sum , 6+8+5);
        assert_eq!(vcg_comp.best_pairings[0], Some(Good{val : 1}));
        assert_eq!(vcg_comp.best_pairings[1], Some(Good{val : 2}));
        assert_eq!(vcg_comp.best_pairings[2], Some(Good{val : 0}));
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
        let mut vcg_comp = VCG_Computer::new(nr_players, nr_goods, masks.view(), bids.view());

        vcg_comp.compute_2p_players();
        assert_eq!(vcg_comp.best_bid_sum , 7);
        assert_eq!(vcg_comp.best_pairings[0], Some(Good{val : 1}));
        assert_eq!(vcg_comp.best_pairings[1], Some(Good{val : 5}));

    }
}