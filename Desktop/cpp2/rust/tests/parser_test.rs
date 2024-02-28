use rust::tokenizer;
use rust::parser::Node;
use rust::parser;
use rust::tokenizer::Token;
use rust::tokenizer::TokenInfo;
use rust::tokenizer::Position;
use std::io::Cursor;

fn is_parsable(input: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => {
            match parser::parse(&tokens) {
                Ok(_) => {
                    return true;
                }
                Err(error) => {
                    println!("{}", error);
                    return false;
                }
            }  
        }
        Err(error) => {
            println!("{}", error);
            return false;
        }
    }
}

#[test]
fn assignment() {
    assert!(is_parsable(
r#"
let i: u32 = u
let a: UserDefined = 11+(22*2)"#
    ));
}

#[test]
fn functions() {
    assert!(is_parsable(
r#"
    fn main(): u32 {
        0 
    }
"#
    ));

    assert!(is_parsable(
r#"
    fn func(i: i32): u32 {
        0 
    }
"#
    ));
}
