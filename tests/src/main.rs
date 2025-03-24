#[derive(Debug)]
enum List {
    X(i32, Option<Box<List>>)
}

use List::*;
fn new_element(x: i32, l: Option<Box<List>>) -> Option<Box<List>> {
    Some(Box::new(X(x, l)))
}

fn main() {
    let x = X(1, Some(Box::new(X(1, None))));
    let y = X(2, new_element(1, new_element(3, None)));
    
    println!("{x:?}");
    println!("{y:?}");
}