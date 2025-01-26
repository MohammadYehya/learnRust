fn main() {
    // While loops
    let mut num: i8 = 3;

    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    // For loops
    let a: [i8; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Unconditional Loops
    num = 5;
    loop {
        println!("again!");
        if num == 0 {break;}
        num -= 1;
    }
}
