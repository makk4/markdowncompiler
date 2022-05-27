use std::collections::LinkedList;

use crate::scanner::{Token, TokenType};



pub(crate) fn parse(token_list: &LinkedList<Token>) -> LinkedList<Token> {
    let mut token_tree:LinkedList<Token> = LinkedList::new();

    token_list.into_iter().for_each(|t| {
        match t.token_type {
            TokenType::AlternativeHeadingOrText => {
                let token = token_tree.pop_back();

                if token.is_some() {
                    let token_before = token.unwrap();
                    match token_before.token_type {
                        TokenType::Paragraph => {
                            token_tree.push_back(Token {
                                token_type: TokenType::AlternativeHeadingOrText,
                                level: t.level,
                                text: token_before.text,
                            });
                        }
                        _ => {
                            token_tree.push_back(t.clone());
                        }
                    }
                }
                else {
                    token_tree.push_back(t.clone());
                }
            },
            _ => {
                token_tree.push_back(t.clone());
            },
        }

    });

    return token_tree;
}