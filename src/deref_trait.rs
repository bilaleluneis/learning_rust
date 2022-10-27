/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]

use std::ops::Deref;

// tuple type struct, members accesses by index 0..
struct BasicBox<T>(T);

impl<T> BasicBox<T> {
    // associated function called as BasicBox::new(v)
    fn new(value: T) -> BasicBox<T> {
        BasicBox(value)
    }
}

// implemeting the Deref Trait will allow us to use * operator
// if you comment out the impl, the test will not work when
// using *b , but will work if you deref manually *(&b.0)
impl <T> Deref for BasicBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
       &self.0
    }
}

#[cfg(test)]
mod deref_traits_tests {
    use crate::deref_trait::BasicBox;

    #[test]
    fn use_deref_operator() {
        let b = BasicBox(9);
        assert_eq!(*b, 9);
        // if you comment out the Deref impl then bellow
        // is the only way to derefrence the value of b
        assert_eq!(*(&b.0), 9);
    }
}
