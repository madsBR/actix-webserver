use std::marker::PhantomData;




pub struct IteratorAs<I,T>{
    old_iter : I,
    ph_T : PhantomData<T>
}

impl<I,T,F> Iterator for IteratorAs<I,T> where I : Iterator<Item = F>, T: From<F>{
    type Item = T;        
    fn next(&mut self) -> Option<Self::Item> {
        match self.old_iter.next(){
            Some(k) =>  Some(k.into()),
            None => None
        } 
    }    
}
pub trait IteratorAsTr<I,F>{
    fn each_into<T : From<F>>(iterator : I) -> IteratorAs<I,T>;
}


impl<I,F> IteratorAsTr<I,F> for I  where I : Iterator<Item = F>{
    fn each_into<T : From<F>>( iterator : I) -> IteratorAs<I,T>{
        IteratorAs { old_iter: iterator , ph_T : PhantomData{}}        
    }
    
}



pub struct IntoIteratorAs<I,T>{
    old_iter : I,
    ph_T : PhantomData<T>,
}

impl<I,T,F> Iterator for IntoIteratorAs<I,T> where I : Iterator<Item = F>, T: From<F>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.old_iter.next(){
            Some(k) =>  Some(k.into()),
            None => None
        } 
    }    
}

pub trait IntoIteratorAsTr<I,F,C>{
    fn into_iter_as<T: From<F>>(self) -> IntoIteratorAs<I,T>;
}



impl<F,C> IntoIteratorAsTr<C::IntoIter,F,C> for C  where C : IntoIterator<Item=F>, C::IntoIter : Iterator  {
    fn into_iter_as<T : From<F>>(self : C) -> IntoIteratorAs<C::IntoIter,T>{
        IntoIteratorAs{old_iter : self.into_iter(), ph_T : PhantomData{}}
    }
    
}
