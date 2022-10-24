/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

#![allow(dead_code)]

trait Shape {}

struct Circle;
impl Shape for Circle {}

struct Triangle;
impl Shape for Triangle {}

struct Square;
impl Shape for Square {}

// takes an array of size 3
fn take_shape<T>(_: [T; 3] ) where T : Shape {}

#[cfg(test)]
mod trait_object {
    use crate::trait_object::*;
    #[test]
    fn dynamic_vs_static_dispatch() {

        // bellow is valid and will allow
        // the assignment of current and
        // future concrete types that implement
        // the Shape trait. Dynamic Dispatch (runtime)
        // also in dynamic dispatch we are really
        // pointing to location that holds the concrete
        // type, so we use Box
        let _: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle),
            Box::new(Triangle),
            Box::new(Square)
        ];

        // this is valid Static Dispatch
        // where T is replaced with one concrete
        // type that must implement the Shape trait
        take_shape([Circle, Circle, Circle]);

        // bellow will cause compiler error
        // because T is one concrete type
        // that implements the Shape trait
        // but we passed different types
        //take_shape([Circle, Square, Triangle])

    }
}
