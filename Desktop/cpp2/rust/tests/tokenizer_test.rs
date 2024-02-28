use rust::tokenizer;
use rust::tokenizer::Position;
use rust::tokenizer::TokenInfo;
use rust::tokenizer::Token;
use std::io::Cursor;

fn tokenize_and_compare(input: &str, expected_output: &[TokenInfo]) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => {
            let result = tokens.iter().eq(expected_output);
            if !result {
                for (result_token, expected_token) in tokens.iter().zip(expected_output.iter()) {
                    if result_token != expected_token {
                        println!("result: {:?}", result_token);
                        println!("expected: {:?}", expected_token);
                    }
                }
            }
            return result; 
        }
        Err(error) => {
            println!("{}", error);
            return false;
        }
    }
}

#[test]
fn typenames() {
    assert!(tokenize_and_compare("u16", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("u16"), start_position: Position{ row: 1, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 4 } }
            ]));

    assert!(tokenize_and_compare("u16\nu64 u32", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("u16"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("u64"), start_position: Position{ row: 2, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("u32"), start_position: Position{ row: 2, col: 5}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 2, col: 8 } }
            ]));
}

#[test]
fn operators() {
    assert!(tokenize_and_compare("==\n= *", &[
                TokenInfo{token: Token::CppForwardedOperator, lexeme: String::from("=="), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::AssignmentOperator, lexeme: String::from("="), start_position: Position{ row: 2, col: 1}},
                TokenInfo{token: Token::CppForwardedOperator, lexeme: String::from("*"), start_position: Position{ row: 2, col: 3}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 2, col: 4 } }
            ]));
}

#[test]
fn assignments() {
    assert!(tokenize_and_compare("let\nmut", &[
                TokenInfo{token: Token::Let, lexeme: String::from("let"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Mut, lexeme: String::from("mut"), start_position: Position{ row: 2, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 2, col: 4 } }
            ]));
}

#[test]
fn identifiers() {
    assert!(tokenize_and_compare("let_\ni_2\nf22nn\n_KSs12", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("let_"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("i_2"), start_position: Position{ row: 2, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("f22nn"), start_position: Position{ row: 3, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("_KSs12"), start_position: Position{ row: 4, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 4, col: 7 } }
            ]));

    assert!(!tokenize_and_compare("1let_", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("1let_"), start_position: Position{ row: 1, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 6 } }
            ]));

    assert!(!tokenize_and_compare("%let_", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("%let_"), start_position: Position{ row: 1, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 6 } }
            ]));
}

#[test]
fn braces() {
    assert!(tokenize_and_compare("{identifier}", &[
                TokenInfo{token: Token::LeftBraces, lexeme: String::from("{"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("identifier"), start_position: Position{ row: 1, col: 2}},
                TokenInfo{token: Token::RightBraces, lexeme: String::from("}"), start_position: Position{ row: 1, col: 12}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 13 } }
            ]));

    assert!(!tokenize_and_compare("1let_", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("1let_"), start_position: Position{ row: 1, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 4, col: 7 } }
            ]));

    assert!(!tokenize_and_compare("%let_", &[
                TokenInfo{token: Token::Identifier, lexeme: String::from("%let_"), start_position: Position{ row: 1, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 6 } }
            ]));
}

#[test]
fn functions() {
    assert!(tokenize_and_compare(
r#"fn main(): u32 {
    return 30
}"#
            , &[
                TokenInfo{token: Token::Fn, lexeme: String::from("fn"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("main"), start_position: Position{ row: 1, col: 4}},
                TokenInfo{token: Token::LeftParantheses, lexeme: String::from("("), start_position: Position{ row: 1, col: 8}},
                TokenInfo{token: Token::RightParantheses, lexeme: String::from(")"), start_position: Position{ row: 1, col: 9}},
                TokenInfo{token: Token::Colon, lexeme: String::from(":"), start_position: Position{ row: 1, col: 10}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("u32"), start_position: Position{ row: 1, col: 12}},
                TokenInfo{token: Token::LeftBraces, lexeme: String::from("{"), start_position: Position{ row: 1, col: 16}},
                TokenInfo{token: Token::Return, lexeme: String::from("return"), start_position: Position{ row: 2, col: 5}},
                TokenInfo{token: Token::Number, lexeme: String::from("30"), start_position: Position{ row: 2, col: 12}},
                TokenInfo{token: Token::RightBraces, lexeme: String::from("}"), start_position: Position{ row: 3, col: 1}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 3, col: 2 } }
            ]));

    assert!(tokenize_and_compare(
r#"fn main():u32{return 30}"#
            , &[
                TokenInfo{token: Token::Fn, lexeme: String::from("fn"), start_position: Position{ row: 1, col: 1}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("main"), start_position: Position{ row: 1, col: 4}},
                TokenInfo{token: Token::LeftParantheses, lexeme: String::from("("), start_position: Position{ row: 1, col: 8}},
                TokenInfo{token: Token::RightParantheses, lexeme: String::from(")"), start_position: Position{ row: 1, col: 9}},
                TokenInfo{token: Token::Colon, lexeme: String::from(":"), start_position: Position{ row: 1, col: 10}},
                TokenInfo{token: Token::Identifier, lexeme: String::from("u32"), start_position: Position{ row: 1, col: 11}},
                TokenInfo{token: Token::LeftBraces, lexeme: String::from("{"), start_position: Position{ row: 1, col: 14}},
                TokenInfo{token: Token::Return, lexeme: String::from("return"), start_position: Position{ row: 1, col: 15}},
                TokenInfo{token: Token::Number, lexeme: String::from("30"), start_position: Position{ row: 1, col: 22}},
                TokenInfo{token: Token::RightBraces, lexeme: String::from("}"), start_position: Position{ row: 1, col: 24}},
                TokenInfo { token: Token::EOF, lexeme: String::from(""), start_position: Position { row: 1, col: 25 } }
            ]));
}
