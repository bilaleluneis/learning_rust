/*
@author: Bilal El Uneis
@since: Oct 2022
@email: bilaleluneis@gmail.com
*/

trait MyTrait {
    type Type;
    fn doublit(&self) -> Self::Type;
}

impl MyTrait for i32 {
    type Type = i32;

    fn doublit(&self) -> Self::Type {
        self * self
    }
}

#[cfg(test)]
mod type_extension_tests {
    use crate::type_extension::MyTrait;

    #[test]
    fn test_i32_extension() {
        let mut v = 10;
        v = v.doublit();
        assert_eq!(v, 100)
    }
}
