use std::io::{self, BufRead};

extern crate simple_profanity_filter;
use simple_profanity_filter::ProfanityFilter;

fn main() {
    // Instantiate the filter
    let filter = ProfanityFilter::new();

    // Loop over stdin, printing the filtered version of each line
    let stdin = io::stdin();
    loop {
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");
        println!("{}\n", filter.filter(&line));
    }
}