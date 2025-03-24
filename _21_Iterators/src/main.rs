fn main() {
    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    {
        // The Iterator Trait and the next Method
        // All iterators implement a trait named Iterator that is defined in the standard library.
        // The next method returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    {
        // Methods that Consume the Iterator
        // Methods that call next are called consuming adapters, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    {
        // Methods that Produce Other Iterators
        // Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
        // The iterator adapter method map takes a closure to call on each item as the items are iterated through. The map method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1

        let v1: Vec<i32> = vec![1, 2, 3];
        // v1.iter().map(|x| x + 1); //Writing this line produces warnings as the closure we’ve specified never gets called. The warning reminds us why: iterator adapters are lazy, and we need to consume the iterator here.

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // We use .collect()
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
