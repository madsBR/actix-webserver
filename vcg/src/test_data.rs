

#[cfg(test)]
pub mod test_utils {
    use tinyvec::{TinyVec,tiny_vec};
    use crate::ext_types::{ID,PlayerExt,GoodExt,Color};
    use std::fmt::{format, Debug};
    pub fn check_vec<T : Eq + Debug>(vec1 : Vec<T>,vec2 : Vec<T>){
        for (ind,(elem1,elem2)) in vec1.iter().zip(vec2.iter()).enumerate(){
            assert!(elem1 == elem2,"vec1 not equal vec2 at index {:?} : {:?} != {:?}",ind,elem1,elem2);
        }
    }

    type TestInput = (
        Option<ID>,TinyVec<[PlayerExt;5]>,
        TinyVec<[GoodExt;10]>,
        Vec<(usize,Option<usize>,
            usize)>
        );

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
            (2,Some(2),4),(2,Some(3),5),(2,Some(4),6),(2,None,7),
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

}