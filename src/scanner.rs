enum TokenType {
    None,
    Heading,
    AlternativeHeadingOrText,
    Space,
    Text,
    Paragraph,
    LineBreak,
    Bold,
    Italic,
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
}

struct Heading {
    level: u8,
    text: String,
}

struct AlternativeHeading {
    level: u8,
    text: String,
}

struct Text {
    text: String,
}

pub(crate) fn scan_token(buffer: &Vec<u8>) {
    let mut token: TokenType = TokenType::None;
    let mut level: u8 = 1;
    let mut text: String = "".to_string();

    for c in buffer {
        match c {
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
                        text.push(*c as char);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(*c as char);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(*c as char);
                    }
                }
            }
            45 => {
                // Alternative Heading level 2 or Text ==
                match token {
                    TokenType::None => {
                        token = TokenType::AlternativeHeadingOrText;
                        level = 2;
                        text.push(*c as char);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(*c as char);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(*c as char);
                    }
                }
            }
            32 => { // Space
                 //ignore
            }
            10 => { // Newline \r
                match token {
                    TokenType::Heading => {
                        let header = Heading {
                            level,
                            text: text.clone(),
                        };
                        print!("{}", header.text);
                        print!("{}", header.level);
                        print!("{}", "  Heading  ");
                    }
                    TokenType::AlternativeHeadingOrText => {
                        let alternative_header = AlternativeHeading {
                            level,
                            text: text.clone(),
                        };
                        print!("{}", alternative_header.text);
                        print!("{}", alternative_header.level);
                        print!("{}", "  AlternativeHeadingOrText  ");
                    }
                    TokenType::Text => {
                        let text = Text { text: text.clone() };
                        print!("{}", text.text);
                        print!("{}", "  Text  ");
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
                        text.push(*c as char);
                    }
                    _ => {
                        text.push(*c as char);
                    }
                }
            }
        }
    }
}
