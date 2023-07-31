use std::time::{Duration,Instant};
use ndarray::{Array2};
use crate::{vcg_auction::VCG_Auction, vcg_base_types::Player};


pub struct Profiler{
    pub masks : Array2<usize>,
    pub bids : Array2<usize>
}
type ResultContainer = Vec<(Duration,Player)>;



impl Profiler{

    fn run_an_auction(&self,auction : &mut VCG_Auction) -> (Duration,Player){
        let time = Instant::now();
        let out = auction.run();
        let duration = time.elapsed();        
        (duration,out[0].pl)
    }
    fn create_auction<'a>(&self) -> VCG_Auction{
        let vcg_comp = VCG_Auction::new(self.masks.view(), self.bids.view());
        vcg_comp
    }
    pub fn run_reps(&self,reps : usize){
        let mut result = ResultContainer::with_capacity(reps);
        for i in 0..reps{
            let mut auction = self.create_auction();
            result.push(self.run_an_auction(&mut auction));
            if i %1000 == 0{
                self.print_res(&result);
            }
        }
        self.print_res(&result);
    }

    fn print_res(&self, result : &ResultContainer){
        let (sum_time,sum_pl) = result.iter().fold((0.,0), 
        |(accm,accp),(dur,pl)| (accm + dur.as_secs_f64() * 1000.,accp + usize::from(*pl))
    );
    let (avg_time,avg_pl) = (sum_time / (result.len() as f64),sum_pl as f64 / (result.len() as f64));
    println!("based on {} iterations, it has taken on average {} milli sec and dummy pl is {}",result.len(),avg_time,avg_pl);
    }
    
}