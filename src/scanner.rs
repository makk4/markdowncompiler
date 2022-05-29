use std::{collections::LinkedList, fmt};

#[derive(Debug, Clone)]
pub enum TokenType {
    None,
    Heading,
    Paragraph,
    /*
    AlternativeHeadingOrText,
    Text,
    Bold,
    Italic,
    BoldAndItalic,
    */
    
    
    /*
    Space,
    LineBreak,
    BoldAndItalic,
    Blockquote,
    BlockquotesWithMultipleParagraphs,
    NestedBlockquote,
    BlockquotesWithOtherElements,
    OrderedList,
    UnorderedList,
    Code,
    EscapingBacktick,
    CodeBlock,
    Link,
    Image,
    HTML,
    */
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub headline_level: u8,
    pub sentence: Vec<Text>,
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
}


pub(crate) fn scan_token(buffer: &Vec<u8>) -> LinkedList<Token> {
    let mut token_list: LinkedList<Token> = LinkedList::new();

    let mut token: TokenType = TokenType::None;
    let mut headline_level: u8 = 0;
    let mut sentence: Vec<Text> = Vec::new();

    let mut text: String = "".to_string();

    let mut header_possible: bool = false;

    IntoIterator::into_iter(buffer).for_each(|b| {
        match b {
            10 => {
                // Newline \r
                match token {
                    _ => {
                        let text = Text {text: text.clone(), bold: false, italic: false};
                        sentence.push(text);
                        token_list.push_back(Token {
                            token_type: token.clone(),
                            headline_level,
                            sentence: sentence.clone()
                        });
                    }
                }
                token = TokenType::None;
                headline_level = 0;
                text = "".to_string();
                sentence = Vec::new();
                header_possible = false;
            }
            32 => {
                // Space
                match token {
                    TokenType::None => {
                        if header_possible {
                            token = TokenType::Heading;
                            text = "".to_string();
                        }
                    }
                    _ => {
                        text.push(*b as char);
                    }
                }
            }
            // 33 => {
            //     // ! Image
            // }
            35 => {
                // # Possible Heading
                match token {
                    TokenType::None => {
                        text.push(*b as char);
                        header_possible = true;

                        headline_level = headline_level + 1;
                        if headline_level >= 7 {
                            header_possible = false;
                        }
                    }
                    _ => {
                        text.push(*b as char);
                    }
                }
            }
            /*
            40 => {
                // (
            }
            41 => {
                // )
            }
            42 => {
                // * Bold or Italics possible
            }
            45 => {
                // - Alternative Heading level 2
            }
            60 => {
                // < Link beginning
            }
            61 => {
                // = Possible Alternative Heading level 1
            }
            62 => {
                // > Blockquote or Link end
            }
            91 => {
                // [
            }
            93 => {
                // ]
            }
            95 => {
                // _ Bold or Italics possible
            }
            */
            _ => {
                // Letters
                match token {
                    TokenType::None => {
                        token = TokenType::Paragraph;
                        text.push(*b as char);
                    }
                    TokenType::Heading | TokenType::Paragraph => {
                        text.push(*b as char);
                    }
                }
            }
        }
    });

    return token_list;
}
