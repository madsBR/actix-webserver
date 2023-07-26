use std::ops::{AddAssign, Add, SubAssign, Index, IndexMut, Sub};
use serde::{Serialize,Deserialize};

macro_rules! impl_usize_like {
    ($usizeLike:ty) => {
        impl Add<usize> for $usizeLike {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                Self{val : self.val + rhs}
            }    
        }
        impl AddAssign<usize> for $usizeLike {
            fn add_assign(&mut self, rhs: usize) {
                self.val+=rhs;
            }
        }

        impl SubAssign<usize> for $usizeLike {
            fn sub_assign(&mut self, rhs: usize) {
                self.val-=rhs;
            }
        }


        impl From<$usizeLike> for usize{
            fn from(pl: $usizeLike) -> Self {
                pl.val
            }
        }
        impl From<usize> for $usizeLike{
            fn from(value: usize) -> Self {
                Self{val : value}
            }
        }

        impl From<&usize> for $usizeLike{
            fn from(value: &usize) -> Self {
                Self{val : *value}
            }
        }

    };
}



#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy,Hash,Serialize,Deserialize,Default)]
pub struct Player{
    pub val: usize
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy,Hash,Serialize,Deserialize,Default)]
pub struct Good{
    pub val : usize
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy,Hash,Serialize,Deserialize,Default)]
pub struct Price{
    pub val : usize
}

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy,Hash,Serialize,Deserialize)]
pub struct Pairing{
    pub pl : Player,
    pub bought_good : Option<GoodWPrice>,
}


impl AddAssign<Price> for Price {
    fn add_assign(&mut self, rhs: Price) {
        self.val+=rhs.val;
    }
}

impl SubAssign<Price> for Price {
    fn sub_assign(&mut self, rhs: Price) {
        self.val-=rhs.val;
    }
}

impl Add<Price> for Price{
    type Output = Self;
    fn add(self, rhs: Price) -> Self::Output {
        Self{val : self.val + rhs.val}
    }    
}
impl Sub<Price> for Price{
    type Output = Self;
    fn sub(self, rhs: Price) -> Self::Output {
        Self{val : self.val - rhs.val}
    }    
}



impl Pairing{
    pub fn new(pl : Player, good : Good, pr : Price) -> Self{
        Self{pl : pl, bought_good : Some(GoodWPrice{good : good , price : pr })}
    }
    pub fn empty_pairing(pl : Player) -> Self{
        Self { pl: pl, bought_good: None }
    }
    pub fn from_unpriced(pl : Player, good: Good) -> Self{
        Self { pl: pl, bought_good: Some(GoodWPrice { good: good, price: 0.into() }) }
    }
}
impl From<(usize,Option<usize>,usize)> for Pairing{
    fn from((pl,good,pr): (usize,Option<usize>,usize)) -> Self {
        match good {
            Some(g) => Self{pl: pl.into(),bought_good : Some(GoodWPrice{good: g.into(), price : pr.into()})},
            None => Self{pl: pl.into(),bought_good: None},
        }
    }
}

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy,Hash,Serialize,Deserialize,Default)]
pub struct GoodWPrice{
    pub good : Good,
    pub price : Price,
}


#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone,Hash)]
pub struct VCGOutput{
    res : Vec<Pairing>
}


impl Index<usize> for VCGOutput{
    type Output = Pairing;
    fn index(&self, index: usize) -> &Self::Output {
        self.res.index(index)
    }
}

impl VCGOutput{
    #[inline]
    pub fn nr_players(&self) -> usize{
        self.res.len()
        
    }
    
    pub fn new(res : Vec<Pairing>) -> Self{
        Self { res: res }
    }

    #[inline]
    pub fn into_buffer(self) -> Vec<Pairing>{
        self.res
    }

}


impl IndexMut<usize> for VCGOutput{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.res.index_mut(index)        
    }
}



impl IntoIterator for VCGOutput{
    type IntoIter = <Vec<Pairing> as IntoIterator>::IntoIter;
    type Item = <Vec<Pairing> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.res.into_iter()
    }
}

impl VCGOutput{
    pub fn iter(&self)-> core::slice::Iter<Pairing>{
        self.res.iter()        

    }
    
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Pairing>{
        self.res.iter_mut()
    }
}



impl Player{
    pub const MAX_PLAYERS : usize = 15;
    pub fn vec_into_usize<T : IntoIterator<Item = Self>>(vec : T) -> Vec<usize>{
        vec.into_iter().map(|x| x.into()).collect()
    }
    pub fn vec_from_usizes<T : IntoIterator<Item = usize>>(vec : Vec<usize>) -> Vec<Self>{
        vec.into_iter().map(|x| x.into()).collect()
    }

    
}
impl Good{
    pub const MAX_GOODS : usize = 15;
    pub fn vec_into_usize<T : IntoIterator<Item = Self>>(vec : T) -> Vec<usize>{
        vec.into_iter().map(|x| x.into()).collect()
    }
    pub fn vec_from_usizes<T : IntoIterator<Item = usize>>(vec : Vec<usize>) -> Vec<Self>{
        vec.into_iter().map(|x| x.into()).collect()
    }

}


impl_usize_like!(Player);
impl_usize_like!(Good);
impl_usize_like!(Price);
