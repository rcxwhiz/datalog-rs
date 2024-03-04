use std::{fs::File, io::{BufRead, BufReader}, iter::Peekable};

pub struct FileIterator<F, L>
where
    F: Iterator<Item = (usize, Peekable<L>)>,
    L: Iterator<Item = (usize, char)>, {
    lines_iterator: F,
    line_iterator: Option<(usize, Peekable<L>)>,
    current_char: Option<(usize, char)>,
}

impl<F, L> FileIterator<F, L>
where
    F: Iterator<Item = (usize, Peekable<L>)>,
    L: Iterator<Item = (usize, char)>, {
    pub fn new(reader: BufReader<File>) -> Self {
        let lines_iterator = reader
            .lines()
            .flatten()
            .enumerate()
            .map(|(line_num, line)| (line_num + 1, line
                .chars()
                .enumerate()
                .map(|(ch_num, ch)| (ch_num + 1, ch))
                .peekable()));

        let mut i = Self { lines_iterator, line_iterator: None, current_char: None };
        let _ = i.next_line();
        i
    }

    pub fn current_char(&self) -> Option<(usize, char)> {
        self.current_char
    }

    pub fn peek_char(&self) -> Option<&(usize, char)> {
        self.line_iterator?.1.peek()
    }

    pub fn next_char(&mut self) -> Option<(usize, char)> {
        self.current_char = self.line_iterator?.1.next();
        self.current_char
    }

    pub fn next_line(&mut self) -> Option<()> {
        self.line_iterator = self.lines_iterator.next();
        match self.next_char() {
            Some(_) => Some(()),
            None => None,
        }
    }

    pub fn line_num(&self) -> usize {
        match self.line_iterator {
            Some((line_num, _)) => line_num,
            None => 0,
        }
    }
}
