

#[cfg(test)]
pub mod test_utils {
    use tinyvec::{TinyVec,tiny_vec};
    use crate::ext_types::{ID,PlayerExt,GoodExt,Color, GoodWPriceExt, OutputPairing};
    use std::fmt::{format, Debug};

    type TestInput = (
        Option<ID>,TinyVec<[PlayerExt;5]>,
        TinyVec<[GoodExt;10]>,
        Vec<(usize,Option<usize>,
            usize)>
        );

    type TestOutput = Vec<OutputPairing>;


    pub fn generate_test_data(player_ids: Vec<usize>, good_ids: Vec<usize>, bids: Vec<usize>,output : Vec<(usize,Option<usize>,usize)>,add_nones : bool) -> (TestInput,TestOutput)
    {       
        let mut result_raw: Vec<(usize, Option<usize>, usize)> = Vec::new();
        let m = good_ids.len();
        let n = player_ids.len();
        let bids = &bids;
        
        assert_eq!(bids.len(), m * n, "Length of bids vector does not match M * N");
        
        for (pl_idx, &pl_id) in player_ids.iter().enumerate() {
            for (good_idx, &good_id) in good_ids.iter().enumerate() {
                let bid_idx = m * pl_idx + good_idx;
                result_raw.push((pl_id, Some(good_id), bids[bid_idx]));    
                if add_nones && good_idx == good_ids.len() - pl_idx {
                    result_raw.push((pl_id, None, pl_id % 10));
                }
            }
        }
        
        let players = player_ids.iter().map(|x| PlayerExt{id : *x,name : x.to_string()}).collect();
        let good = good_ids.iter().map(|x| GoodExt{id : *x, name : x.to_string(),color : Color { str: "#000000".to_string() }}).collect();
        let inp : TestInput = (Some(ID::new_random()),players,good,result_raw);

        let outputext : TestOutput = output.iter().map(|(pl,y,pr)| 
            match y {
                Some(good) => {
                    let pl_ext = PlayerExt{id : *pl, name: pl.to_string()};
                    let good_ext = GoodExt { id : *good,name : good.to_string(),color : Color { str: "#000000".to_string() }};
                    let good_w_price = GoodWPriceExt{good: good_ext, price : pr.into()};
                    let out = OutputPairing{pl : pl_ext, good_color_price : Some(good_w_price)};
                    out
                }
                None => {
                    let pl_ext = PlayerExt{id : *pl, name: pl.to_string()};
                    let out = OutputPairing{pl : pl_ext, good_color_price : None};
                    out
                }
            }
        ).collect();
        (inp,outputext)


    }


    pub fn check_vec<T : Eq + Debug>(vec1 : Vec<T>,vec2 : Vec<T>){
        for (ind,(elem1,elem2)) in vec1.iter().zip(vec2.iter()).enumerate(){
            assert!(elem1 == elem2,"vec1 not equal vec2 at index {:?} : {:?} != {:?}",ind,elem1,elem2);
        }
        assert_eq!(vec1.len(),vec2.len(),"all elements checked where equal, but vec left had length {} and vec right had length {}",vec1.len(),vec2.len());

    }


    pub fn get_test_data_valid() -> TestInput{
        let id = Some(ID::new_random());
        let pls : TinyVec<[PlayerExt;5]> = tiny_vec!(
            PlayerExt{id : 1, name : "B".to_string()},
            PlayerExt{id : 2, name : "C".to_string()},
            PlayerExt{id : 3, name : "D".to_string()},
            PlayerExt{ id :4, name : "E".to_string()}

        );
        let goods : TinyVec<[GoodExt;10]> =  tiny_vec!(
            GoodExt{id : 1, name : "Bg".to_string(), color: "#004400".try_into().unwrap()},
            GoodExt{id : 2, name : "Cg".to_string(), color: "#009990".try_into().unwrap()},
            GoodExt{id : 3, name : "Dg".to_string(), color: "#009890".try_into().unwrap()},
            GoodExt{id : 4, name : "Eg".to_string(), color: "#006490".try_into().unwrap()},
            GoodExt{id : 5, name : "Fg".to_string(), color: "#003980".try_into().unwrap()},
            GoodExt{id : 6, name : "Gg".to_string(), color: "#009870".try_into().unwrap()}
        );
        let bidp = vec![
            (1,Some(2),4),(1,Some(3),6),(1,Some(4),7),(1,None,7),
            (2,Some(2),3),(2,Some(3),5),(2,Some(4),6),(2,None,7),
            (3,Some(2),3),(3,Some(3),5),(3,Some(4),8),(3,None,10),            
            (4,Some(2),4),(4,Some(3),5),(4,Some(4),6),(4,None,7),            
        ];
        (id,pls,goods,bidp)
    }

    pub fn get_test_data_bad_pl() ->TestInput{
        let id = Some(ID::new_random());
        let pls : TinyVec<[PlayerExt;5]> = tiny_vec!(
            PlayerExt{ id : 0, name : "A".to_string()},
            PlayerExt{id : 1, name : "B".to_string()},
            PlayerExt{id : 2, name : "C".to_string()},
            PlayerExt{id : 3, name : "D".to_string()}
        );
        let goods : TinyVec<[GoodExt;10]> =  tiny_vec!(
            GoodExt{id : 1, name : "Bg".to_string(), color: "#004400".try_into().unwrap()},
            GoodExt{id : 2, name : "Cg".to_string(), color: "#009990".try_into().unwrap()},
            GoodExt{id : 3, name : "Dg".to_string(), color: "#009890".try_into().unwrap()},
            GoodExt{id : 4, name : "Eg".to_string(), color: "#006490".try_into().unwrap()},
            GoodExt{id : 5, name : "Fg".to_string(), color: "#003980".try_into().unwrap()},
            GoodExt{id : 6, name : "Gg".to_string(), color: "#009870".try_into().unwrap()}
        );
        let bidp = vec![
            (1,Some(2),4),(1,Some(3),6),(1,Some(4),7),(1,None,7),
            (2,Some(2),4),(2,Some(3),5),(2,Some(4),6),(2,None,7),
            (3,Some(2),3),(3,Some(3),5),(3,Some(4),8),(3,None,10),            
            (4,Some(2),4),(4,Some(3),5),(4,Some(4),6),(4,None,7),
            
        ];
        (id,pls,goods,bidp)
    }

    pub fn get_test_data_bad_good() ->TestInput{
        let id = Some(ID::new_random());
        let pls : TinyVec<[PlayerExt;5]> = tiny_vec!(
            PlayerExt{ id : 1, name : "B".to_string()},
            PlayerExt{id : 2, name : "C".to_string()},
            PlayerExt{id : 3, name : "D".to_string()},
            PlayerExt{id : 4, name : "E".to_string()}
        );
        let goods : TinyVec<[GoodExt;10]> =  tiny_vec!(
            GoodExt{id : 1, name : "Bg".to_string(), color: "#004400".try_into().unwrap()},
            GoodExt{id : 2, name : "Cg".to_string(), color: "#009990".try_into().unwrap()},
            GoodExt{id : 4, name : "Eg".to_string(), color: "#006490".try_into().unwrap()},
            GoodExt{id : 5, name : "Fg".to_string(), color: "#003980".try_into().unwrap()},
            GoodExt{id : 6, name : "Gg".to_string(), color: "#009870".try_into().unwrap()}
        );
        let bidp = vec![
            (1,Some(2),4),(1,Some(3),6),(1,Some(4),7),(1,None,7),
            (2,Some(2),4),(2,Some(3),5),(2,Some(4),6),(2,None,7),
            (3,Some(2),3),(3,Some(3),5),(3,Some(4),8),(3,None,10),            
            (4,Some(2),4),(4,Some(3),5),(4,Some(4),6),(4,None,7),
            
        ];
        (id,pls,goods,bidp)




    }


    use std::option::Option;

    // Define the function
    fn generate_tuples(player_ids: Vec<usize>, good_ids: Vec<usize>, bids: Vec<usize>) -> Vec<(usize, Option<usize>, usize)> {
        // Calculate the lengths of the input vectors
        let m = player_ids.len();
        let n = good_ids.len();
    
        // Ensure that the length of bids is consistent with M*N
        assert_eq!(bids.len(), m * n, "Length of bids vector does not match M * N");
    
        // Create an empty vector to store the result
        let mut result: Vec<(usize, Option<usize>, usize)> = Vec::new();
    
        // Iterate through the player IDs
        for (pl_idx, &pl_id) in player_ids.iter().enumerate() {
            // Iterate through the good IDs using enumerate()
            for (good_idx, &good_id) in good_ids.iter().enumerate() {
                // Calculate the index to access the correct bid value in the bids vector
                let bid_idx = m * pl_idx + good_id;    
                // Push the tuple (player_id, Some(good_id), bid) into the result vector
                result.push((pl_id, Some(good_id), bids[bid_idx]));
            }
        }
    
        // Return the resulting vector of tuples
        result
    }
    

    
}