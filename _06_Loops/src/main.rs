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

    // Return value with break
    num = loop {
        break 20;
    };
    println!("Loop returned with value: {num}");

    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
