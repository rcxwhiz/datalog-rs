use std::{fs::File, io::{BufRead, BufReader}, iter::Peekable};

use crate::token::{Token, TokenValue};

pub struct Lexer;

impl Lexer {
    pub fn lex(reader: BufReader<File>) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut line_iter = reader.lines().flatten().enumerate().map(|(ln, l)| (ln + 1, l));
        while let Some((line_num, line)) = line_iter.next() {
            let mut char_iter = line.chars().enumerate().map(|(cn, c)| (cn + 1, c)).peekable();
            while let Some((char_num, ch)) = char_iter.next() {
                // todo
            }
        }

        tokens
    }
}

trait TokenFinder {
    fn find_token<'a, C, L>(ch: char, line: &mut Peekable<C>, lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)>;
}

struct SimpleTokenFinder;
impl TokenFinder for SimpleTokenFinder {
    fn find_token<'a, C, L>(ch: char, line: &mut Peekable<C>, _lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)> {
        match ch {
            ',' => Some(TokenValue::Comma),
            '.' => Some(TokenValue::Period),
            '?' => Some(TokenValue::QMark),
            '(' => Some(TokenValue::LeftParen),
            ')' => Some(TokenValue::RightParen),
            '*' => Some(TokenValue::Multiply),
            '+' => Some(TokenValue::Add),
            ':' => {
                if let Some((_, next)) = line.peek() {
                    if *next == '-' {
                        return Some(TokenValue::ColonDash)
                    }
                }
                Some(TokenValue::Colon)
            },
            _ => None,
        }
    }
}

struct CommentTokenFinder;
impl CommentTokenFinder {
    fn get_comment<'a, C, L>(ch: char, line: &mut Peekable<C>, _lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)> {
            let mut comment_value = String::from(ch);
            while let Some((_, next)) = line.peek() {
                if next.is_whitespace() {
                    break
                }
                comment_value.push(*next);
                let _ = line.next();
            }
            Some(TokenValue::Comment(comment_value))
    }
    fn get_block_comment<'a, C, L>(ch: char, line: &mut Peekable<C>, lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)> {
            let mut comment_value = String::from(ch);
            comment_value.push(line.next().unwrap().1);
            let mut line_it = line;
            while let Some((_, cha)) = line_it.next() {
                // boooo how do I get the line iterator to work?
            }
            None
    }
}
impl TokenFinder for CommentTokenFinder {
    fn find_token<'a, C, L>(ch: char, line: &mut Peekable<C>, lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)> {
        if ch == '#' {
            if let Some((_, next)) = line.peek() {
                if *next == '|' {
                    return Self::get_block_comment(ch, line, lines)
                }
            }
            return Self::get_comment(ch, line, lines)
        }
        None
    }
}

struct IdTokenFinder;
impl TokenFinder for IdTokenFinder {
    fn find_token<'a, C, L>(ch: char, line: &mut Peekable<C>, lines: &mut L) -> Option<TokenValue>
    where
        C: Iterator<Item = (usize, char)>,
        L: Iterator<Item = (usize, String)> {
        if ch.is_alphabetic() {
            let mut id_value = String::from(ch);
            while let Some((_, next)) = line.peek() {
                if !next.is_alphanumeric() {
                    break;
                }
                id_value.push(*next);
                let _ = line.next();
            }
            return Some(match id_value.as_str() {
                "Schemes" => TokenValue::Schemes,
                "Facts" => TokenValue::Facts,
                "Rules" => TokenValue::Rules,
                "Queries" => TokenValue::Queries,
                _ => TokenValue::Id(id_value),
            })
        }
        None
    }
}
