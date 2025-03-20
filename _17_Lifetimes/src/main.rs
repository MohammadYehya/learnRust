fn main() {
    /*
    Lifetimes are another kind of generic that we’ve already been using.
    Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
    The main aim of lifetimes is to prevent dangling references.
    Lifetimes can be thought of as critical sections in a concurrent program.
    */

    {
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }
        // This implementation doesnt work as the compiler doesnt know what to reference to

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // We have to specify that the return value exists during the lifetime of the parameter

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }

    // Lifetime Annotations in Struct Definitions
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // The Static Lifetime
    {
        // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.
        // All string literals have the 'static lifetime, which we can annotate as follows:
        let s: &'static str = "I have a static lifetime.";
    }
}
