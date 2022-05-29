use std::collections::LinkedList;
use crate::scanner::{Token, Text};

pub(crate) fn html_out(token_list: &LinkedList<Token>) -> String {
    let mut output: String = "".to_string();

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
        }

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
            start = format!("{}","<em>");
            end = format!("{}","</em>");
        }
        if text.bold {
            start = format!("{}{}", start, "<strong>");
            end = format!("{}{}", end, "</strong>");
        }
        sentence.push_str(&format!("{}{}{}", &start, &text.text, &end));
    });

    return sentence;
}