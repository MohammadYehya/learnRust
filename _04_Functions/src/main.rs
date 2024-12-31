fn main() {
    println!("{}",test(50));
}

fn test(num: u32) -> u32 {
    num*2
    //Not ending in a semicolon means this is an expression not a statement, thus also indicating a return
}

//Basic syntax for functions
/*
fn FUNCTION_NAME(PARAMS) -> RETURN_TYPE{
    CODE
}

*/