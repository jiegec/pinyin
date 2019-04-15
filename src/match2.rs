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

pub type Match2Prefix = [char; 1];

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Match2((Match2Prefix, char));

impl Match2 {
    pub fn iter<'a>(input: &'a str, valid: &'a BTreeSet<char>) -> Match2Iter<'a> {
        Match2Iter {
            cur: None,
            chars: input.chars(),
            valid,
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
    valid: &'a BTreeSet<char>,
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
                            }
                            _ => {
                                self.cur = Some(cur);
                            }
                        }
                    } else {
                        self.cur = None;
                    }
                }
                None => return None,
            }
        }
    }
}

impl Model<Match2> {
    pub fn load() -> Self {
        let data = GzDecoder::new(Cursor::new(include_bytes!("model2.json.gz").to_vec()));
        let json_model: JsonModel = serde_json::from_reader(data).expect("json");
        let mut prob = BTreeMap::new();
        for (key, value) in &json_model.prob {
            prob.insert(Match2::from_str(key), *value);
        }

        Model {
            mapping: json_model.mapping,
            prob,
        }
    }

    pub fn save(&self) {
        let writer = GzEncoder::new(
            File::create("model2.json.gz").expect("open file"),
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

impl Match for Match2 {
    type Prefix = Match2Prefix;

    fn min_len() -> usize {
        2
    }

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix> {
        let mut res = Vec::new();
        for word in input.iter() {
            assert!(word.len() == Self::min_len() - 1);
            res.push([word[0]]);
        }
        res
    }

    fn shift_prefix(&self) -> Self::Prefix {
        [(self.0).1]
    }

    fn new(prefix: &Self::Prefix, end: char) -> Self {
        Match2((prefix.clone(), end))
    }
}