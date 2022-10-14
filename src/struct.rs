/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

// use so IDE will not warn about code not used in project and only used in Tests
#![allow(dead_code)]

// tuple like struct
struct Pair<T>(T, T);

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair(first, second)
    }
}

// name field struct
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// unit struct
struct OneOff;

impl OneOff {
    fn new() -> Self {
        OneOff
    }
}

#[cfg(test)]
mod strcut_tests {
    use super::*;

    #[test]
    fn test_pair() {
        let Pair(x, y) = Pair::new(0, 0); // destructure
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn test_point() {
        let point = Point::new(0, 0);
        assert_eq!(point.x, 0)
    }
}
