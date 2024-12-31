fn main() {
    let x: u32 = 10;
    let x: f32 = 11.11;    //x is being overwritten

    println!("{x}");


    //This applies to when scope changes as well

    let x: u32 = 12;

    {
        let x = x+2;
        println!("{x}");    //prints 14
    }
    println!("{x}");        //prints 12
}
