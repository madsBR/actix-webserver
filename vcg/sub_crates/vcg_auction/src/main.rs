

use ndarray::{Array1,Array2,Axis,ArrayView2};
use vcg_auction::auction_profiler;

fn main() {
    let nr_players = 3usize; let nr_goods = 4usize;
    let masks = Array2::<usize>::from_diag(&Array1::ones(nr_goods));
    let bids = Array2::from_shape_vec((nr_players,nr_goods),vec!
    [4usize,1,9,0,
        5,1,8,5,
        5,1,4,0]).unwrap();
    let profiler = auction_profiler::Profiler{
        bids : bids,
        masks : masks};
    profiler.run_reps(10000)
        
}