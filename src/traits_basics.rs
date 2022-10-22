/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]

trait ResultTrait<Type> {
    fn get(&self) -> (&Type, bool);
}

struct Result<T>(T, bool);

// implemeting the ResultTrait for Result
impl<T> ResultTrait<T> for Result<T> {
    fn get(&self) -> (&T, bool) {
        (&self.0, self.1)
    }
}

struct Point {
    x: i32,
    y: i32,
}

// generic function that returns a concrete type
// that impl the ResultTrait
fn gen_result<T>(value: T) -> impl ResultTrait<T> {
    Result(value, true)
}

#[cfg(test)]
mod traits_basics_tests {
    use crate::traits_basics::*;

    #[test]
    fn return_of_trait() {
        let result = gen_result(1);
        let (value, ok) = result.get();
        assert_eq!(*value, 1);
        assert_eq!(ok, true);

        let result = gen_result(Point { x: 0, y: 0 });
        let (value, _) = result.get();
        assert_eq!(value.x, 0);
    }
}
