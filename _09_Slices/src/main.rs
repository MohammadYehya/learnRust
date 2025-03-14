fn main() {
    // A slice is a reference to any collection of data

    let mut s = String::from("hello world");

    let word = first_word(&s);
    
    // s.clear();       This requires a mutable reference and there exists an immutable reference that is being used later

    println!("{word}");

    s.clear();          // Now this works!

    // Similarly here is a slice of an i8 array

    let a: [i8; 5] = [1,2,3,4,5];
    let r1 = &a[..1];
    println!("{}", r1[0]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}