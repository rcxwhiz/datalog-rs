use std::{fs::File, io::{BufRead, BufReader}, iter::Peekable};

use crate::{file_iterator::FileIterator, token::{Token, TokenValue}};

pub struct Lexer;

impl Lexer {
    pub fn lex(reader: BufReader<File>) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut file_iter = FileIterator::new(reader);

        while file_iter.current_char().is_some() || file_iter.next_line().is_some() {
            // pass the file iterator to the token finders
        }

        tokens
    }
}

trait TokenFinder {
    fn get_token<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)>;
}

struct SimpleTokenFinder;
impl TokenFinder for SimpleTokenFinder {
    fn get_token<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)> {
        let (char_num, ch) = file_iter.current_char()?;
        let mut token_value = match ch {
            ',' => TokenValue::Comma,
            '.' => TokenValue::Period,
            '?' => TokenValue::QMark,
            '(' => TokenValue::LeftParen,
            ')' => TokenValue::RightParen,
            '*' => TokenValue::Multiply,
            '+' => TokenValue::Add,
            ':' => TokenValue::Colon,
            _ => return None,
        };
        // check for colon dash case
        if let (TokenValue::Colon, Some((_, '-'))) = (token_value, file_iter.peek_char()) {
            token_value = TokenValue::ColonDash;
            let _ = file_iter.next_char();
        }
        Some(Token { value: token_value, line_num: file_iter.line_num(), char_num })
    }
}

struct CommentTokenFinder;
impl CommentTokenFinder {
    fn get_comment<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)> {
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
    fn get_block_comment<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)> {
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
    fn get_token<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)> {
        todo!()
    }
}

struct IdTokenFinder;
impl TokenFinder for IdTokenFinder {
    fn get_token<F, L>(file_iter: &mut FileIterator<F, L>) -> Option<Token>
    where
        F: Iterator<Item = (usize, Peekable<L>)>,
        L: Iterator<Item = (usize, char)> {
        if let Some((char_num, ch)) = file_iter.current_char().and_then(|(num, ch)| ch.is_alphanumeric().then(|| (num, ch))) {
            let mut id_value = String::from(ch);
            while let Some((_, next)) = file_iter.peek_char() {
                if !next.is_alphanumeric() {
                    break;
                }
                id_value.push(*next);
                let _ = file_iter.next_char();
            }
            let token_value = match id_value.as_str() {
                "Schemes" => TokenValue::Schemes,
                "Facts" => TokenValue::Facts,
                "Rules" => TokenValue::Rules,
                "Queries" => TokenValue::Queries,
                _ => TokenValue::Id(id_value),
            };
            return Some(Token { value: token_value, line_num: file_iter.line_num(), char_num })
        }
        None
    }
}
