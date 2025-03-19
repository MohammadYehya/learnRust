fn main() {
    // The purpose of generic types is to reduce redundant code
    // It is simply templates in C++

    {
        fn printarr<T: std::fmt::Display>(arr: &[T]) {
            for i in arr {
                print!("{i}");
            }
        }
        let arr1 = [1, 2, 3, 4];
        let arr2 = ['a', 'b', 'c', 'd'];
        printarr(&arr1);
        printarr(&arr2);
    }

    {
        struct Point<T> {
            _x: T,
            _y: T,
        }

        // Note that we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T> by declaring T as a generic type after impl.
        impl<T> Point<T> {
            fn _x(&self) -> &T {
                &self._x
            }
        }

        let _integer = Point { _x: 5, _y: 10 };
        let _float = Point { _x: 1.0, _y: 4.0 };
    }
}
