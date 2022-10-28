/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]


use crate::deref_trait::BasicBox;

// global counter
static mut DROP_COUNTER: i32 = 0;

// implemeting Drop trait, which is
// automatically invoked when
// instance goes out of scope.
impl <T> Drop for BasicBox<T> {
    fn drop(&mut self) {
        unsafe{ DROP_COUNTER += 1; }
    }
}

#[cfg(test)]
mod drop_trait_tests {
    use crate::deref_trait::BasicBox;
    use crate::drop_trait::DROP_COUNTER;

    #[test]
    fn instance_counter() {
        BasicBox::new(0);
        BasicBox::new(0);
        unsafe{ assert_eq!(DROP_COUNTER, 2); }

    }
}