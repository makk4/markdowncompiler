use std::collections::LinkedList;
use crate::scanner::Token;


pub(crate) fn html_out(token_list: &LinkedList<Token>) -> String {
    let mut output: String = "".to_string();

    token_list.into_iter().for_each(|t| {
        let mut start: String = "".to_string();
        let mut end: String = "".to_string();
        match &t.token_type {
            crate::scanner::TokenType::None => {},
            crate::scanner::TokenType::Heading | crate::scanner::TokenType::AlternativeHeadingOrText => {
                start = format!("<h{}>", &t.level);
                end = format!("</h{}>", &t.level);
            },
            crate::scanner::TokenType::Text => {
                output.push_str(&t.text);
            },
            crate::scanner::TokenType::Paragraph => {
                start = "<p>".to_string();
                end = "</p>".to_string();
            },
        }
        output.push_str(&format!("{}{}{}\n", &start, &t.text, &end));
    });

    return output;
}