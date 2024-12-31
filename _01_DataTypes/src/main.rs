fn main() {
    //Unsigned integer data types = u8, u16, u32, u64
    let x1: u8 = 128;

    //Signed integer data types = i8, i16, i32, i64
    let x2: i8 = -127;  
    
    //Floating point numbers data types = f32, f64
    let x3: f32 = 1.234;
    
    //Character data type = char
    let x4: char = 'a';

    //Boolean data types = bool
    let x5: bool = true;

    println!("{x1}, {x2}, {x3}, {x4}, {x5}");

    //Tuples
    let tup:(i32, f64, &str)  = (500, 1.01, "hello");   //Can be used in destructuring as well
    let (t1, t2 , t3) = tup;
    
    println!("{tup:?}\n{}, {}, {}\n{t1}, {t2}, {t3}", tup.0,tup.1,tup.2);
    
    //Ararys  
    let arr: [i32;6] = [1,2,3,4,4,6];
    let _a = [3; 5]; //Creates [3, 3, 3, 3, 3]

    println!("{arr:?}, {}", arr[0]);

}
