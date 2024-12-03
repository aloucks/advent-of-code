use core::str;
use std::iter::Peekable;

#[derive(Debug, Clone)]
enum Token {
    Mul,
    Do,
    Dont,
    LeftParen,
    RightParen,
    Comma,
    Number(i64),
    #[allow(unused)]
    Junk(String),
}

fn consume_number<I: Iterator<Item = char>>(
    input_chars: &mut Peekable<I>,
    buffer: &mut String,
    tokens: &mut Vec<Token>,
) {
    while let Some(d) = input_chars.next_if(|c| c.is_ascii_digit()) {
        buffer.push(d);
    }
    if !buffer.is_empty() && buffer.chars().next().unwrap().is_ascii_digit() {
        tokens.push(Token::Number(str_to_i64(&buffer)));
        buffer.clear();
    }
}

fn process_junk(buffer: &mut String, tokens: &mut Vec<Token>) {
    if !buffer.is_empty() {
        tokens.push(Token::Junk(buffer.to_owned()));
        buffer.clear();
    }
}

fn process_partial_junk(end_pos: usize, buffer: &mut String, tokens: &mut Vec<Token>) {
    let junk = &buffer[0..end_pos];
    if !junk.is_empty() {
        tokens.push(Token::Junk(junk.to_owned()));
    }
    buffer.clear();
}

fn has_left_paren_next<I: Iterator<Item = char>>(input_chars: &mut Peekable<I>) -> bool {
    input_chars.peek() == Some(&'(')
}

fn main() {
    let input = include_str!("aoc2024day03_input.txt");

    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let mut input_chars = input.chars().peekable();

    // Lexer
    loop {
        match input_chars.next() {
            Some(')') => {
                process_junk(&mut buffer, &mut tokens);
                tokens.push(Token::RightParen);
            }
            Some('(') => {
                process_junk(&mut buffer, &mut tokens);
                tokens.push(Token::LeftParen);
                consume_number(&mut input_chars, &mut buffer, &mut tokens);
            }
            Some(',') => {
                process_junk(&mut buffer, &mut tokens);
                tokens.push(Token::Comma);
                consume_number(&mut input_chars, &mut buffer, &mut tokens);
            }
            Some(c) => {
                buffer.push(c);
                if let Some(pos) = buffer.find("mul") {
                    if has_left_paren_next(&mut input_chars) {
                        process_partial_junk(pos, &mut buffer, &mut tokens);
                        tokens.push(Token::Mul);
                    }
                }

                if let Some(pos) = buffer.find("don't") {
                    if has_left_paren_next(&mut input_chars) {
                        process_partial_junk(pos, &mut buffer, &mut tokens);
                        tokens.push(Token::Dont);
                    }
                }

                if let Some(pos) = buffer.find("do") {
                    if has_left_paren_next(&mut input_chars) {
                        process_partial_junk(pos, &mut buffer, &mut tokens);
                        tokens.push(Token::Do);
                    }
                }
            }
            None => {
                process_junk(&mut buffer, &mut tokens);
                break;
            }
        }
    }


    // Parser
    let mut total = 0;

    let mut pos = 0;
    let mut do_add = true;
    while pos < tokens.len() {
        let a = tokens.get(pos);
        let b = tokens.get(pos + 1);
        let c = tokens.get(pos + 2);
        let d = tokens.get(pos + 3);
        let e = tokens.get(pos + 4);
        let f = tokens.get(pos + 5);

        match a {
            Some(Token::Do) => do_add = true,
            Some(Token::Dont) => do_add = false,
            _ => {}
        }

        match (a, b, c, d, e, f) {
            (
                Some(&Token::Mul),
                Some(&Token::LeftParen),
                Some(&Token::Number(a)),
                Some(&Token::Comma),
                Some(&Token::Number(b)),
                Some(&Token::RightParen),
            ) => {
                if do_add {
                    total += a * b;
                }
                pos += 5;
            }
            _ => {}
        }
        pos += 1;
    }

    println!("total: {total}");
}

fn str_to_i64(a: &str) -> i64 {
    a.parse().unwrap()
}
