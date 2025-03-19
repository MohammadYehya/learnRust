use std::fmt::Display;

fn main() {
    // Traits are similar to a feature often called interfaces in other languages
    // A trait defines the functionality a particular type has and can share with other types.

    use _16_Traits::{Summary, Tweet};
    {
        // Implementing a Trait on a Type
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        // Traits as parameters
        // Is basically asking for a parameter of any type that implements that specific trait
        // Is almost the same as the generic types code (templates)

        pub fn notify1(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        // OR
        pub fn notify2<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }

        // However, these 2 are not the same
        pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}
        // AND
        pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

        // We can also specify more than one trait bound.
        pub fn notify5(item: &(impl Summary + Display)) {}
        // OR
        pub fn notify6<T: Summary + Display>(item: &T) {}

        /*
        Using too many trait bounds has its downsides.
        Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.
        For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature.
        So, instead of writing this:

        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

        we can use a where clause, like this:

        fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {}
        */
    }

    {
        // Returning Types That Implement Traits
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }

    {
        // Using Trait Bounds to Conditionally Implement Methods
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // The next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait.
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
    }
}
