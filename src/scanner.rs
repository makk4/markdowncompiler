use std::{collections::LinkedList, fmt};

#[derive(Debug)]
pub enum TokenType {
    None,
    Heading,
    AlternativeHeadingOrText,
    // Space,
    Text,
    Paragraph,
    // LineBreak,
    // Bold,
    // Italic,
    // BoldAndItalic,
    // Blockquote,
    // BlockquotesWithMultipleParagraphs,
    // NestedBlockquote,
    // BlockquotesWithOtherElements,
    // OrderedList,
    // UnorderedList,
    // Code,
    // EscapingBacktick,
    // CodeBlock,
    // Link,
    // Image,
    // HTML,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub level: u8,
    pub text: String,
}

pub(crate) fn scan_token(buffer: &Vec<u8>) -> LinkedList<Token> {
    let mut token_list: LinkedList<Token> = LinkedList::new();
    let mut token: TokenType = TokenType::None;
    let mut level: u8 = 1;
    let mut text: String = "".to_string();

    buffer.into_iter().for_each(|b| {
        match b {
            35 => {
                // Heading #
                match token {
                    TokenType::None => {
                        token = TokenType::Heading;
                    }
                    TokenType::Heading => {
                        level = level + 1;
                        if level >= 7 {
                            token = TokenType::Text;
                        }
                    }
                    _ => {}
                }
            }
            61 => {
                // Alternative Heading level 1 or Text ---
                match token {
                    TokenType::None => {
                        token = TokenType::AlternativeHeadingOrText;
                        level = 1;
                        text.push(*b as char);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(*b as char);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(*b as char);
                    }
                }
            }
            45 => {
                // Alternative Heading level 2 or Text ==
                match token {
                    TokenType::None => {
                        token = TokenType::AlternativeHeadingOrText;
                        level = 2;
                        text.push(*b as char);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(*b as char);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(*b as char);
                    }
                }
            }
            32 => { // Space
                //ignore
            }
            10 => { // Newline \r
                match token {
                    TokenType::Heading => {
                        token_list.push_back(Token {
                            token_type: TokenType::Heading,
                            level,
                            text: text.clone(),
                        });
                    }
                    TokenType::AlternativeHeadingOrText => {
                        token_list.push_back(Token {
                            token_type: TokenType::AlternativeHeadingOrText,
                            level,
                            text: text.clone(),
                        });
                    }
                    TokenType::Text => {
                        token_list.push_back(Token {
                            token_type: TokenType::Text,
                            level,
                            text: text.clone(),
                        });
                    }
                    TokenType::None => {
                        token_list.push_back(Token {
                            token_type: TokenType::Paragraph,
                            level,
                            text: text.clone(),
                        });
                    }
                    _ => {}
                }
                token = TokenType::None;
                level = 1;
                text = "".to_string();
            }
            _ => {
                // Text
                match token {
                    TokenType::None => {
                        token = TokenType::Text;
                        text.push(*b as char);
                    }
                    _ => {
                        text.push(*b as char);
                    }
                }
            }
        }
    });

    return token_list;
}
