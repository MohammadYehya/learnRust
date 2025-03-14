fn main() {
    {
        // To avoid the tuple code in the previous section, we can use references
        let s = String::from("hello");
        let l = do_something(&s);
        println!("{s}-{l}"); // As you can see s was not dropped after function execution
    }

    {
        let mut s = String::from("hello");
        do_somthing_mutable(&mut s); // Passing s as a mutable reference
        println!("{s}");
    }

    {
        /*
            In Rust, there are 2 types of references, mutable and immutable.
            At a time there can only be 1 mutable reference
            At a time there can be any number of immutable references
            If an immutable reference is in scope, a mutable reference can not exist
            These rules are in place to avoid race conditions
        */
        let mut s = String::from("Hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1}, {r2}");     //This works

        // let r3 = &s;
        // let r4 = &mut s;
        // println!("{r3}, {r4}")   This Doesn't

        // A reference only lives until the last time its called
    }
}

fn do_something(str: &String) -> usize {
    str.len()
}

fn do_somthing_mutable(str: &mut String) {
    str.push_str(", world!");
}
