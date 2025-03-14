fn main()
{
    /*
    Ownership Rules
     -> Each value in Rust has an owner.
     -> There can only be one owner at a time.
     -> When the owner goes out of scope, the value will be dropped
    */

    {
        let s = "hello";   // s is valid from this point forward
        println!("{s}")
    }
    // Out here s is not valid

    {
        let s = String::from("Hello");  // Allocate memory on heap
        println!("{s}")
    }
    // When going out of scope, Rust automatically deallocates it by calling drop()

    {
        let s1 = String::from("Test");
        let s2 = s1;    
        // Since s1 is allocating on heap, when performing s2 = s1, we woould need a copy which leads to bugs like double free error. To be safe from this Rust uses move semantics
        println!("{s2}")    // s1 doesnt exist
    }

    {
        let mut s1 = String::from("Test");
        println!("{s1}");

        s1 = String::from("Hi");    // The above allocation will be dropped
        println!("{s1}")
    }

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();    // For deep copy purposes

        println!("{s1}, {s2}")
    }

    {
        let s = String::from("Hello");
        takes_ownership(s);
        // s can no longer be used here as drop() was called when the function finished executing
    }

    {
        let s1 = gives_ownership();
        // Similarly functions can be used to give ownership
        println!("{s1}")
    }

    {
        // The problem with sending objects to a function is that we are effectively moving that object to that function. We will inevitably need to send that object back as a tuple.
        let s = String::from("Hello");
        let (s, len) = do_something(s);
        println!("{s},{len}")
    }
}

fn takes_ownership(str: String)
{
    println!("{str}")
}

fn gives_ownership() -> String {
    String::from("Yours")
}

fn do_something(str: String) -> (String, usize){
    let len = str.len();
    return (str, len)
}