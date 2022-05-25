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

pub(crate) fn scan_token(buffer: String) {
    let mut token: TokenType = TokenType::None;
    let mut level: u8 = 1;
    let mut text: String = "".to_string();

    for c in buffer.chars() {
        match c {
            '#' => {
                // Heading
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
            '=' => {
                // Alternative Heading level 1 or Text
                match token {
                    TokenType::None => {
                        token = TokenType::AlternativeHeadingOrText;
                        level = 1;
                        text.push(c);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(c);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(c);
                    }
                }
            }
            '-' => {
                // Alternative Heading level 2 or Text
                match token {
                    TokenType::None => {
                        token = TokenType::AlternativeHeadingOrText;
                        level = 2;
                        text.push(c);
                    }
                    TokenType::AlternativeHeadingOrText => {
                        text.push(c);
                    }
                    _ => {
                        token = TokenType::Text;
                        text.push(c);
                    }
                }
            }
            ' ' => { // Space
                 //ignore
            }
            _ => {
                // Text
                match token {
                    TokenType::None => {
                        token = TokenType::Text;
                        text.push(c);
                    }
                    _ => {
                        text.push(c);
                    }
                }
            }
        }
    }

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
}
