use super::*;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Cursor;
use std::iter::Iterator;
use std::str::Chars;

pub type Match3Prefix = Match2;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Match3((Match3Prefix, char));

impl Match3 {
    pub fn iter<'a>(input: &'a str, valid: &'a BTreeSet<char>) -> Match3Iter<'a> {
        Match3Iter {
            cur: None,
            cur2: None,
            chars: input.chars(),
            valid,
        }
    }

    pub fn get_prefix(&self) -> Match3Prefix {
        (self.0).0.clone()
    }


    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(2);
        res.push_str(&(self.0).0.to_string());
        res.push((self.0).1);
        res
    }
}

pub struct Match3Iter<'a> {
    cur: Option<char>,
    cur2: Option<char>,
    chars: Chars<'a>,
    valid: &'a BTreeSet<char>,
}

impl<'a> Iterator for Match3Iter<'a> {
    type Item = Match3;

    fn next(&mut self) -> Option<Match3> {
        loop {
            let cur = self.chars.next();
            match cur {
                Some(cur) => {
                    if self.valid.contains(&cur) {
                        match self.cur {
                            Some(ch) => {
                                let result = Match3((Match2::new(&Match1::new(&ch, ch), self.cur2.unwrap()), cur));
                                self.cur = self.cur2;
                                self.cur2 = Some(cur);
                                return Some(result);
                            }
                            _ => {
                                self.cur = self.cur2;
                                self.cur2 = Some(cur);
                            }
                        }
                    } else {
                        self.cur = None;
                        self.cur2 = None;
                    }
                }
                None => return None,
            }
        }
    }
}

impl Model<Match3> {
    pub fn load() -> Self {
        let data = GzDecoder::new(Cursor::new(include_bytes!("model3.json.gz").to_vec()));
        let json_model: JsonModel = serde_json::from_reader(data).expect("json");
        let mut prob = BTreeMap::new();
        for (key, value) in &json_model.prob {
            prob.insert(Match3::from_str(key), *value);
        }

        Model {
            mapping: json_model.mapping,
            prob,
        }
    }

    pub fn save(&self) {
        let writer = GzEncoder::new(
            File::create("model3.json.gz").expect("open file"),
            Compression::default(),
        );

        let mut prob = BTreeMap::new();
        for (key, value) in &self.prob {
            prob.insert(key.to_string(), *value);
        }

        let json_model = JsonModel {
            mapping: self.mapping.clone(),
            prob,
        };
        serde_json::to_writer(writer, &json_model).expect("json");
    }
}

impl Match for Match3 {
    type Prefix = Match3Prefix;

    fn min_len() -> usize {
        3
    }

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix> {
        let mut res = Vec::new();
        for word in input.iter() {
            assert!(word.len() == Self::min_len() - 1);
            res.push(Match2::new(&Match1::new(&word[0], word[0]), word[1]));
        }
        res
    }

    fn shift_prefix(&self) -> Self::Prefix {
        (self.0).0.clone()
    }

    fn new(prefix: &Self::Prefix, end: char) -> Self {
        Match3((prefix.clone(), end))
    }

    fn empty() -> Self {
        Match3((Match2::empty(), ' '))
    }

    fn from_str(s: &str) -> Match3 {
        let mut ch = s.chars();
        let _first = ch.next().unwrap();
        let _second = ch.next().unwrap();
        let third = ch.next().unwrap();
        Match3((Match2::from_str(s), third))
    }
}
