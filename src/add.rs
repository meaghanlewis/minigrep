/* use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //use BufReader instead: https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html
    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}
 */