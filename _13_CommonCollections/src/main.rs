fn main() {
    // Vectors
    {
        let _v1: Vec<i8> = Vec::new();
        // or
        let _v1 = vec![1i8,2,3];

        let mut v = Vec::new();
        v.push(5i8);
        v.push(6);
        v.push(7);
        v.push(8);

        let third = &v[2];
        println!("{third}");
        // or
        if let Some(x) = v.get(2) {
            println!("{x}");
        }
        else {
            println!("Empty");
        }

        // Since we have an immutable reference, if we were to modify the vector between creating and using the reference, that would be an error.
        // This is because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space 

        for i in &v {
            println!("{i}");
        }

        for i in &mut v {
            *i += 50;
        }

        // To store multiple data types, we can use Enums
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        
        for i in &row {
            println!("{i:?}");
        }
    }

    // Strings
    {

    }

    // Hash Maps
    {

    }
}
