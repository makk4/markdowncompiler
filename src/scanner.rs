use std::{collections::LinkedList, fmt};

#[derive(Debug, Clone)]
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

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub level: u8,
    pub text: String,
}

pub(crate) fn scan_token(buffer: &Vec<u8>) -> LinkedList<Token> {
    let mut token_list: LinkedList<Token> = LinkedList::new();
    let mut token: TokenType = TokenType::None;
    let mut level: u8 = 0;
    let mut text: String = "".to_string();
    let mut header_possible: bool = false;
    let mut alternative_header_l2_possible: bool = false;
    let mut alternative_header_l1_possible: bool = false;

    buffer.into_iter().for_each(|b| {
        match b {
            35 => {
                // # Possible Heading
                match token {
                    TokenType::None => {
                        header_possible = true;
                        level = level + 1;
                        if level >= 7 {
                            header_possible = false;
                        }
                    }
                    _ => {}
                }
            }
            61 => {
                // = Possible Alternative Heading level 1
                match token {
                    TokenType::None => {
                        alternative_header_l1_possible = true;
                        alternative_header_l2_possible = false;
                        level = 1;
                        text.push(*b as char);
                    }
                    _ => {
                        text.push(*b as char);
                    }
                }
            }
            45 => {
                // - Alternative Heading level 2
                match token {
                    TokenType::None => {
                        alternative_header_l2_possible = true;
                        alternative_header_l1_possible = false;
                        level = 2;
                        text.push(*b as char);
                    }
                    _ => {
                        text.push(*b as char);
                    }
                }
            }
            32 => {
                // Space
                match token {
                    TokenType::None => {
                        if header_possible {
                            token = TokenType::Heading;
                        }
                        else if alternative_header_l1_possible || alternative_header_l2_possible {
                            token = TokenType::AlternativeHeadingOrText;
                        }
                        else {
                            token = TokenType::Paragraph;
                        }
                    }
                    _ => {}
                }
            }
            10 => {
                // Newline \r
                if header_possible {
                    token = TokenType::Heading;
                }
                else if alternative_header_l1_possible | alternative_header_l2_possible {
                    token = TokenType::AlternativeHeadingOrText;
                }
                match token {
                    TokenType::None => {
                        //
                    }
                    _ => {
                        token_list.push_back(Token {
                            token_type: token.clone(),
                            level,
                            text: text.clone(),
                        });
                    }
                }
                token = TokenType::None;
                level = 0;
                text = "".to_string();
                header_possible = false;
                alternative_header_l2_possible = false;
            }
            _ => {
                alternative_header_l2_possible = false;

                if header_possible {
                    header_possible = false;
                }
                // Letters
                match token {
                    TokenType::None => {
                        token = TokenType::Paragraph;
                        text.push(*b as char);
                    }
                    TokenType::Heading => {
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
