use std::env;
use std::fs;
use std::process;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    // such as a vector, containing all the elements the iterator produces.
    let args: Vec<String> = env::args().collect();

    // Saves the values of the two arguments in variables so we can use the values throughout the rest of the program.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // A nonzero exit status is a convention to signal to the process that called our program that the program exited with an
        // error state
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Reading the file
    run(config);
    

}

// Extracting a run function containing the rest of program logic
fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}


struct Config {
    query: String,
    filename: String,
}

// Note: using primitive values when a complex type would be more appropriate is an anti-pattern knwon as primitive obsession.
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        // If the slice isn't long enough, the program panics and displays a better error message than the
        // `index out of bounds` message.
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        // When we printed the vector, the program's name takes up the first value in the vector at args[0], so we're starting at the
        // index 1. 
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
