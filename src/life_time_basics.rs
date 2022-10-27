/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]

/* this function states that:
    - for reference x and y , both should have same lifetime
    - meaning that when this function return, those 2 vars
    - would still be alive
    - in the bellow test both s1 and s2 scope is until the end of
    - test_lifetime() s1 and s2 live from this scope {
        s1
        s2
        return_longest() called and after this call both s1 and s2 still alive
    } s1 and s2 get released at end of this scope
*/
fn return_longest<'a>(x: &'a String, y: &'a String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*  this implies that CycleRef holds a borrow reference
   to ref_val of type T, and CycleRef instance will
   Not live longer than ref_val property the instance
   holds.
*/
struct CycleRef<'a, T> {
    ref_val: &'a T,
}

#[cfg(test)]
mod life_time_basics_tests {
    use crate::life_time_basics::return_longest;

    #[test]
    fn test_lifetime() {
        let s1 = String::from("hello");
        let s2 = String::from("HI");
        assert_eq!(return_longest(&s1, &s2), "hello")
    }

    #[test]
    fn test_cycle_ref() {}
}
