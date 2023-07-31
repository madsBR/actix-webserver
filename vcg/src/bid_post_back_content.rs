use crate::ext_types::{GoodExt, PlayerExt, ID};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tinyvec::TinyVec;


#[derive(Deserialize, Serialize, Debug)]
pub struct BidPostBackContent {
    pub id: Option<ID>,
    pub player_nr: u64,
    pub pls: TinyVec<[PlayerExt; 5]>,
    pub goods: TinyVec<[GoodExt; 10]>,
    pub bid_pairings: Vec<(usize, Option<usize>, usize)>,
}

impl BidPostBackContent {}
