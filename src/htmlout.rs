use std::collections::LinkedList;
use crate::{scanner::{Token, Text}};

pub fn html_out(token_list: &LinkedList<Token>) -> String {
    let mut output: String = "".to_string();
    let mut prev_token: &Token = &Token {
        token_type: crate::scanner::TokenType::None, 
        headline_level: 0, 
        sentence: Vec::new()
    };

    token_list.into_iter().for_each(|t| {
        let mut start: String = "".to_string();
        let mut end: String = "".to_string();

        match &t.token_type {
            crate::scanner::TokenType::None => {},
            crate::scanner::TokenType::Heading => {
                start = format!("<h{}>", &t.headline_level);
                end = format!("</h{}>", &t.headline_level);
            }
            crate::scanner::TokenType::Paragraph => {
                start = "<p>".to_string();
                end = "</p>".to_string();
            }
            crate::scanner::TokenType::UnorderedList => {
                match &prev_token.token_type {
                    crate::scanner::TokenType::UnorderedList => {
                        start = "<li>".to_string();
                    }
                    _ => {
                        start = "<ul>\n<li>".to_string();
                    }
                }
                end = "</li>".to_string();
            }
            crate::scanner::TokenType::OrderedList => {
                match &prev_token.token_type {
                    crate::scanner::TokenType::OrderedList => {
                        start = "<li>".to_string();
                    }
                    _ => {
                        start = "<ol>\n<li>".to_string();
                    }
                }
                end = "</li>".to_string();
            }
            crate::scanner::TokenType::Code => {
                start = "<code>".to_string();
                end = "</code>".to_string();
            }
        }

        match &prev_token.token_type {
            crate::scanner::TokenType::UnorderedList => {
                match &t.token_type {
                    crate::scanner::TokenType::UnorderedList => {}
                    _ => {
                        start = "</ul>\n".to_string() + &start;
                    }
                } 
            }
            _ => {}
        }

        match &prev_token.token_type {
            crate::scanner::TokenType::OrderedList => {
                match &t.token_type {
                    crate::scanner::TokenType::OrderedList => {}
                    _ => {
                        start = "</ol>\n".to_string() + &start;
                    }
                } 
            }
            _ => {}
        }

        prev_token = &t;
        let sentence = get_sentence(&t.sentence);
        output.push_str(&format!("{}{}{}\n", &start, &sentence, &end));
    });

    return output;
}

fn get_sentence(texts: &Vec<Text>) -> String {
    let mut sentence ="".to_string();

    IntoIterator::into_iter(texts).for_each(|text| {
        let mut start: String = "".to_string();
        let mut end: String = "".to_string();

        if text.italic {
            start = "<em>".to_string();
            end = "</em>".to_string();
        }
        if text.bold {
            start = format!("{}{}", start, "<strong>");
            end = format!("{}{}", end, "</strong>");
        }
        sentence.push_str(&format!("{}{}{}", &start, &text.text, &end));
    });

    return sentence;
}