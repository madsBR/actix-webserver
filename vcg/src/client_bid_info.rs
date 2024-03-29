use crate::bid_post_back_content::BidPostBackContent;
use crate::ext_types::{GoodExt, GoodWPriceExt, OutputPairing, PlayerExt, ID, InputRawPairing};
use std::{time::{Instant}};
use tinyvec::TinyVec;
use vcg_auction::vcg_base_types::{Good, GoodWPrice, Pairing, Player, Price};
use std::rc::Rc;
pub struct ContentMetaData {
    pub players_active: TinyVec<[PlayerExt; 5]>,
    pub goods_active: TinyVec<[GoodExt; 10]>,
    pub bid_nr: usize,
    pub players_total: TinyVec<[PlayerExt; 5]>,
    pub goods_total : TinyVec<[GoodExt; 10]>,
}


impl ContentMetaData {
    pub fn from_content_vals(bid_pairings : &[InputRawPairing],cont_pls : TinyVec<[PlayerExt;5]>,cont_goods : TinyVec<[GoodExt;10]>) -> Result<Self, String> {
        let mut pl_ids: Vec<usize> = Vec::new();
        let mut good_ids: Vec<usize> = Vec::new();
        let mut bid_counter: usize = 0;
        let iter = bid_pairings.iter().filter_map(|(x, y, _)| {
            if y.is_some() {
                Some((*x, y.unwrap()))
            } else {
                None
            }
        });
        for (pl_ext, good_ext) in iter{        
            pl_ids.push(pl_ext);
            good_ids.push(good_ext);
            bid_counter += 1;
        }        
        pl_ids.sort();
        pl_ids.dedup();
        good_ids.sort();
        good_ids.dedup();

        let mut players: TinyVec<[PlayerExt; 5]> = TinyVec::with_capacity(pl_ids.len());
        let mut goods: TinyVec<[GoodExt; 10]> = TinyVec::with_capacity(good_ids.len());
        for pl in pl_ids.iter() {
            if let Some(pl_ext) = cont_pls.iter().find(|ext| ext.id == *pl) {
                players.push(pl_ext.clone());
            } else {
                let x = format!("A bid came from urecognized player {}", pl);
                return Err(x);
            }
        }
        for good in good_ids.iter() {
            if let Some(good_ext) = cont_goods.iter().find(|ext| ext.id == *good) {
                goods.push(good_ext.clone());
            } else {
                let x = format!("A bid came for an urecognized good {}", good);
                return Err(x);
            }
        }

        Ok(Self {
            players_active: players,
            goods_active: goods,
            bid_nr: bid_counter,
            players_total : cont_pls, 
            goods_total : cont_goods,
        })
    }

    #[inline]
    pub fn player_int_to_ext(&self, pl: Player) -> &PlayerExt {
        &self.players_active[usize::from(pl)]
    }

    #[inline]
    pub fn good_int_to_ext(&self, good: Good) -> &GoodExt {
        &self.goods_active[usize::from(good)]
    }

    #[inline]
    pub fn some_pairing_int_to_ext(&self, pl: Player, good_w_price: GoodWPrice) -> OutputPairing {
        let good_ext = self.good_int_to_ext(good_w_price.good);

        OutputPairing {
            pl: self.player_int_to_ext(pl).clone(),
            good_color_price: Some(GoodWPriceExt {
                good: good_ext.clone(),
                price: good_w_price.price,
            }),
        }
    }

    #[inline]
    pub fn pairing_int_to_ext(&self, pair: Pairing) -> OutputPairing {
        let Pairing {
            pl,
            bought_good,
        } = pair;
        let _pl_ext = self.player_int_to_ext(pl);
        match bought_good {
            Some(GoodWPrice {
                good,
                price,
            }) => self.some_pairing_int_to_ext(
                pl,
                GoodWPrice {
                    good,
                    price,
                },
            ),
            None => OutputPairing {
                pl: self.player_int_to_ext(pl).clone(),
                good_color_price: None,
            },
        }
    }

    pub fn pairings_int_to_ext(&self, vec: &mut [Pairing]) -> Vec<OutputPairing> {
        vec.iter().map(|x| self.pairing_int_to_ext(*x)).collect()
    }
}

pub struct ClientBidInfo {
    pub id: ID,
    pub bid_buffer: Vec<(Player, Good, Price)>,
    pub created_at: Instant,
    pub metadata: ContentMetaData,
}

impl ClientBidInfo{
    pub fn new(metadata :ContentMetaData, bids :  &[InputRawPairing],id : Option<ID>) -> Self{
        let instant = Instant::now();
        let bid_pair_builder = BidPairingBuilder::new(&metadata, bids);
        let cli_bid_info = ClientBidInfo {
            id: id.unwrap_or_else(ID::new_random),
            bid_buffer: bid_pair_builder.parse_bid_pairings(),
            created_at: instant,
            metadata,
        };
        cli_bid_info
    }
}

impl ClientBidInfo {}


impl BidPostBackContent{
    pub fn validate_and_unpack(self) -> Result<(ClientBidInfo, Rc<Vec<InputRawPairing>>),String>{
        let BidPostBackContent{id,player_nr:_player_nr,goods,pls,mut bid_pairings} = self;
        bid_pairings.sort();
        let bids = Rc::new(bid_pairings);
        let metadata_mb = ContentMetaData::from_content_vals(&bids,pls,goods);     
        match metadata_mb{
            Ok(metadata) =>{
                let client_bid_info = ClientBidInfo::new(metadata, &bids, id);
                Ok((client_bid_info,bids))
            }
            Err(str) => Err(str)
        }
    }
}

struct BidPairingBuilder<'a> {
    metadt: &'a ContentMetaData,
    result: Vec<(Player, Good, Price)>,
    content_bid_pairs: &'a [InputRawPairing],

    pl_ind: usize,
    good_ind: usize,
}

impl<'a> BidPairingBuilder<'a> {
    fn new(
        metadt: &'a ContentMetaData,
        content_bid_pairs: &'a [InputRawPairing]
    ) -> Self {
        Self {
            metadt,
            result: Vec::with_capacity(content_bid_pairs.len()),
            content_bid_pairs,
            pl_ind: 0,
            good_ind: 0,
        }
    }

    fn parse_bid_pairings(mut self) -> Vec<(Player, Good, Price)> {
        for index in 0..self.content_bid_pairs.len() {
            if let (pl_id, Some(good_id), pr) = self.content_bid_pairs[index] {
                self.update_pl_good(pl_id, good_id);
                self.result
                    .push((self.pl_ind.into(), self.good_ind.into(), pr.into()));
            }
        }
        self.result
    }

    fn update_pl_good(&mut self, pl_id: usize, good_id: usize) {
        if self.metadt.players_active[self.pl_ind].id != pl_id {
            self.good_ind = 0;
            self.pl_ind += 1;
        }
        while self.metadt.players_active[self.pl_ind].id != pl_id {
            self.pl_ind += 1;
        }

        while self.good_ind < self.metadt.goods_active.len() {
            if self.metadt.goods_active[self.good_ind].id != good_id {
                self.good_ind += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bid_post_back_content::BidPostBackContent;
    use crate::test_data::test_utils::{
        get_test_data_bad_good, get_test_data_valid,
    };
    use crate::{test_data::test_utils::check_vec};
    #[test]
    fn test_create_metadata_bad_pl() {
        let (id, pls, goods, bids) = get_test_data_bad_good();
        let content = BidPostBackContent {
            id,
            player_nr: pls.len() as u64,
            pls,
            goods,
            bid_pairings: bids,
        };
        let metadata = ContentMetaData::from_content_vals(&content.bid_pairings,content.pls,content.goods);
        assert!(metadata.is_err());
    }

    #[test]
    fn test_create_metadata_bad_good() {
        let (id, pls, goods, bids) = get_test_data_bad_good();
        let content = BidPostBackContent {
            id,
            player_nr: pls.len() as u64,
            pls,
            goods,
            bid_pairings: bids,
        };
        let metadata = ContentMetaData::from_content_vals(&content.bid_pairings,content.pls,content.goods);
        assert!(metadata.is_err());
    }

    #[test]
    fn test_create_metadata_valid() {
        let (id, pls, goods, bids) = get_test_data_valid();
        let content = BidPostBackContent {
            id,
            player_nr: pls.len() as u64,
            pls,
            goods,
            bid_pairings: bids,
        };

        let metadata = ContentMetaData::from_content_vals(&content.bid_pairings,content.pls,content.goods);
        assert!(metadata.is_ok());
    }

    #[test]
    fn test_create_bid_info() {
        let (id, pls, goods, bids) = get_test_data_valid();
        let content = BidPostBackContent {
            id,
            player_nr: pls.len() as u64,
            pls,
            goods,
            bid_pairings: bids,
        };
        let unpacked_content = content.validate_and_unpack();
        assert!(unpacked_content.is_ok());
        let (client_bid_info,_) = unpacked_content.unwrap();

        let mut x = client_bid_info.bid_buffer;
        x.sort();
        let exp_vec_pre = vec![
            (0, Some(0), 4),
            (0, Some(1), 6),
            (0, Some(2), 7),
            (1, Some(0), 3),
            (1, Some(1), 5),
            (1, Some(2), 6),
            (2, Some(0), 3),
            (2, Some(1), 5),
            (2, Some(2), 8),
            (3, Some(0), 4),
            (3, Some(1), 5),
            (3, Some(2), 6),
        ];

        let exp_vec: Vec<(Player, Good, Price)> = exp_vec_pre
            .into_iter()
            .map(|(x, y, z)| {
                (
                    Player { val: x },
                    Good { val: y.unwrap() },
                    Price { val: z },
                )
            })
            .collect();
        check_vec(x, exp_vec);
    }
}
