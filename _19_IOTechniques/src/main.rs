use std::env;
use std::fs;

fn main() {
    {
        // Command line arguments

        // cargo run -- abc example.txt
        let args: Vec<String> = env::args().collect();
        println!("{}", args[2]);
    }

    {
        // Reading a txt file from root level of project

        let contents = fs::read_to_string("txt.txt").expect("Should have been able to read the file");
        println!("{contents}");
    }
}
