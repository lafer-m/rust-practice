use itertools::Itertools;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Chars;


const SPACE: char = ' ';
const OPEN_PARENTHESIS: char = '(';
const CLOSE_PARENTHESIS: char = ')';
const EQUAL: char = '=';
const QUOTE: char = '"';

#[derive(Debug)]
enum Token {
    Equals(String, String),
    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
    Not(Box<Token>),
}

fn tokenize(input: &str) -> Token {
    let mut iter = input.chars().peekable();
    parse_expression(&mut iter)
}

fn parse_expression(iter: &mut Peekable<std::str::Chars>) -> Token {
    let mut token = parse_term(iter);
    while let Some(&ch) = iter.peek() {
        match ch {
            SPACE => {
                iter.next();
                continue;
            }
            'A'..='Z' => {
                let word = iter.peeking_take_while(|x| {*x != SPACE}).collect::<String>();
                token = match &word[..] {
                    "AND" => parse_and(iter, token),
                    "OR" => parse_or(iter, token),
                    "NOT" => parse_not(iter),
                    _ => panic!("Invalid token"),
                };
            }
            CLOSE_PARENTHESIS => {
                iter.next();
                break;
            },
            _ => panic!("Invalid token"),
        }
    }

    token
}

/// a=value1 OR a="value2",
/// value not support '"'
fn parse_term(iter: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    let mut term = String::new();
    let mut after_equal = false;
    let mut first_value_char =  false;
    while let Some(&ch) = iter.peek() {
        match ch {
            EQUAL => {
                after_equal = true;
                term.push(ch);
                iter.next();
            }
            QUOTE => {
                // pop first "
                iter.next();
                let value = iter.peeking_take_while(|x| {*x != '"'}).collect::<String>();
                term += &value;
                // pop last "
                iter.next();
                break;
            }
            SPACE => {
                // before = this will effect, means key space
                if !after_equal {
                    iter.next();
                    continue;
                }
                // after = before first value's char, this will 
                if !first_value_char {
                    iter.next();
                    continue;
                }

                break;
            },
            'N' => {
                // support NOT(xxxx=vv AND xxx=vv2)
                // support NOT xxx=vvv
                // support AND/OR NOT xxx=xxxx
                let word = iter.take(3).collect::<String>();
                match &word[..] {
                    "NOT" => return parse_not(iter),
                    _ => {
                        term += &word;
                        continue;
                    }  
                }
            }
            OPEN_PARENTHESIS => {
                iter.next();
                return parse_expression(iter);
            }
            CLOSE_PARENTHESIS => {
                break;
            }
            _ => {
                if after_equal && !first_value_char {
                    first_value_char = true;
                }
                term.push(ch);
                iter.next();
            }
        }
    }

    if term.contains(&EQUAL.to_string()) {
        let parts: Vec<&str> = term.split("=").collect();
        Token::Equals(parts[0].to_string(), parts[1].to_string())
    } else {
        panic!("Invalid token")
    }
}

fn parse_and(
    iter: &mut Peekable<Chars>,
    left: Token,
) -> Token {
    let right = parse_term(iter);
    Token::And(Box::new(left), Box::new(right))
}

fn parse_or(
    iter: &mut Peekable<Chars>,
    left: Token,
) -> Token {
    let right = parse_term(iter);
    Token::Or(Box::new(left), Box::new(right))
}

fn parse_not(iter: &mut Peekable<Chars>) -> Token {
    let term = parse_term(iter);
    Token::Not(Box::new(term))
}

fn evaluate(token: &Token, input: &str) -> bool {
    // println!("token: {:?}", token);
    match token {
        Token::Equals(field, value) => {
            let field_value = get_field_value(input, field);
            field_value == value
        }
        Token::And(left, right) => evaluate(left, input) && evaluate(right, input),
        Token::Or(left, right) => evaluate(left, input) || evaluate(right, input),
        Token::Not(inner) => !evaluate(inner, input),
    }
}

fn get_field_value<'a>(input: &'a str, field: &'a str) -> &'a str {
    // Dummy function to get the field value from input
    // Replace this with your own logic to extract field values
    match field {
        "a" => "value1",
        "b" => "value9",
        "c" => "value4",
        "d" => "value6",
        _ => "",
    }
}

fn main() {
    // let input = "NOT(a=value1 AND (b=value2 OR (d=value8 OR d=value7))) AND (c=value3 OR c=value4 AND d = value6)";
    let input = "NOT(a =  value1 AND NOT   b  =  \"value9 \") AND (c=value3 OR c=value4 AND d = value6)";
    // let input = "a=value1 AND b= \"value7\"";
    // let input = "a  =   value1 AND b= \"value9\" AND c=value4 AND d=value7"; // Result: false
    // let input = "a=value1 AND b=\"value8\" OR (c= value5 AND d=v2)";
    let tokens: Token = tokenize(input);
    println!("token: {:?}", tokens);
    let result = evaluate(&tokens, input);
    println!("Result: {}", result);
}
