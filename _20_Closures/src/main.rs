fn main() {
    // Closures are basically arrow functions in other languages

    fn _add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let _add_one_v3 = |x :u32| {x + 1};
    let _add_one_v4 = |x: u32| x + 1;
    // All of the above are the same

}
