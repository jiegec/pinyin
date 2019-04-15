use std::iter::Iterator;
use serde::{Serialize, Deserialize};
use std::str::Chars;
use std::collections::BTreeSet;

pub type Match2Prefix = [char; 1];

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Match2((Match2Prefix, char));

impl Match2 {
    pub fn iter<'a>(input: &'a str, valid: &'a BTreeSet<char>) -> Match2Iter<'a> {
        Match2Iter {
            cur: None,
            chars: input.chars(),
            valid
        }
    }

    pub fn get_prefix(&self) -> Match2Prefix {
        (self.0).0.clone()
    }

    pub fn from_str(s: &str) -> Match2 {
        let mut ch = s.chars();
        let first = ch.next().unwrap();
        let second = ch.next().unwrap();
        Match2(([first], second))
    }

    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(2);
        res.push((self.0).0[0]);
        res.push((self.0).1);
        res
    }
}

pub struct Match2Iter<'a> {
    cur: Option<char>,
    chars: Chars<'a>,
    valid: &'a BTreeSet<char>
}

impl<'a> Iterator for Match2Iter<'a> {
    type Item = Match2;

    fn next(&mut self) -> Option<Match2> {
        loop {
            let cur = self.chars.next();
            match cur {
                Some(cur) => {
                    if self.valid.contains(&cur) {
                        match self.cur {
                            Some(ch) => {
                                let result = Match2(([ch], cur));
                                self.cur = Some(cur);
                                return Some(result);
                            },
                            _ => {
                                self.cur = Some(cur);
                            }
                        }
                    } else {
                        self.cur = None;
                    }
                },
                None => return None
            }
        }
    }
}