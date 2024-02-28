use rust::tokenizer;
use rust::parser;
use rust::evaluator;
use std::fs::File;
use std::io::BufReader;

fn main() {
    for arg in std::env::args().into_iter().skip(1) {
        let mut reader = BufReader::new(File::open(&arg).expect("Error opening file."));

        match tokenizer::tokenize(&mut reader) {
            Err(error) => println!("\n{} in file {}", error, arg),
            Ok(tokens) => match parser::parse(&tokens) {
                Err(error) => println!("\n{} in file {}", error, arg),
                _ => ()//match evaluator::parse(&tokens) {
                    //Err(error) => println!("\n{} in file {}", error, arg),
                   //Ok(result) => ()
                //}
            }
        };
    }
}
