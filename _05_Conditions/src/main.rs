fn main() {
    let num: i8 = 10;

    // Classical if operator
    if num == 10 {
        println!("Number is 10!");
    }
    else if num == 5 {
        println!("Number is 5!");
    }
    else {
        println!("Number is neither 10 nor 5!");
    }

    // if used as a ternary operator
    let num:i8 = if true {5} else {6};
    println!("{num}");
}