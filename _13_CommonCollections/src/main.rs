fn main() {
    // Vectors
    {
        let _v1: Vec<i8> = Vec::new();
        // or
        let _v1 = vec![1i8, 2, 3];

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
        } else {
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

        let row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row {
            println!("{i:?}");
        }
    }

    // Hash Maps
    {
        use std::collections::HashMap;

        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("{:?}", scores.get("Blue1").copied().unwrap_or(0));
        // Since the get method returns a Option<&T>, we will call the copied function to convert it to Option<T>.
        // Then we use the unwrap_or function to unwrap the Option type and get the actual values.

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // A HashMap takes ownership of types that dont implement the copy trait

        // or_insert is defined to return a mutable reference to the value of a key if that key exists, and if not, it inserts the parameter as the new value for this key
        {
            scores.entry(String::from("Yellow")).or_insert(100);
            scores.entry(String::from("Green")).or_insert(50);
            println!("{:?}", scores);
        }

        // Since or_insert returns a mutable reference, we can use it to update a value in a Map
        {
            let text = "hello world wonderful world";
            let mut map = HashMap::new();

            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }

            println!("{map:?}");
        }
    }
}
