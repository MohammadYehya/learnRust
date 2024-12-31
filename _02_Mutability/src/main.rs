fn main() {
    //By default all variables are immutable
    let x = 10;
    //x = 11;   //This does not work

    println!("{x}");
    
    let mut x = 11;
    x = 12;     //This works

    println!("{x}");

    const PI: f32 = 3.1412;
    //const mut PI: f32 = 3.1412;     //This doesnt work


}
