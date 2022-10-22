/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]

// this function will take value and move it from caller (own it)
fn move_value_from_caller(value: String) -> String {
    return value;
}

fn borrow_value_from_caller(_: &String) {}

#[cfg(test)]
mod borrow_checker_basics_tests {

    use crate::borrow_checker_basics::{borrow_value_from_caller, move_value_from_caller};

    #[test]
    fn test_basic_move_semantics() {
        let s = String::from("testString");
        assert_eq!(move_value_from_caller(s), "testString");
        /* uncomment bellow to see move error */
        // assert_eq!(s, "testString");
    }

    #[test]
    fn test_basic_borrow_sematics() {
        let s = String::from("testString");
        borrow_value_from_caller(&s);
        assert_eq!(s, "testString");
    }
}
