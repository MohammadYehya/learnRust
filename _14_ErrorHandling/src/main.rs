fn main() {
    // Rust has a panic! function which is basically a err().

    // panic!("Wassup!");
    /*
        thread 'main' panicked at src/main.rs:4:5:
        Wassup!
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: process didn't exit successfully: `target\debug\_14_ErrorHandling.exe` (exit code: 101)
    */

    // We can use the Result enum to recover from errors
    {
        use std::fs::File;
        use std::io::ErrorKind;
        let greeting_file_result = File::open("hello.txt");
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        // To make error more specific:
        let greeting_file_result = File::open("hello.txt");
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        };

        // This is using too many match statements. Instead we can use the unwrap() or expect() functions
        {
            let greeting_file = File::open("hello.txt").unwrap();
            let greeting_file =
                File::open("hello.txt").expect("hello.txt should be included in this project");
        }

        // Propagting Errors
        {
            use std::fs::File;
            use std::io::{self, Read};
            {
                fn read_username_from_file() -> Result<String, io::Error> {
                    let username_file_result = File::open("hello.txt");

                    let mut username_file = match username_file_result {
                        Ok(file) => file,
                        Err(e) => return Err(e),
                    };

                    let mut username = String::new();

                    match username_file.read_to_string(&mut username) {
                        Ok(_) => Ok(username),
                        Err(e) => Err(e),
                    }
                }
            }
            // This function can be written in a much shorter way

            {
                fn read_username_from_file() -> Result<String, io::Error> {
                    let mut username_file = File::open("hello.txt")?;
                    let mut username = String::new();
                    username_file.read_to_string(&mut username)?;
                    Ok(username)
                }
            }
            // The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in the previous function.
            // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. 
            // If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
        }
    }
}
