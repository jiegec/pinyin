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

pub type Match1Prefix = char;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Match1((Match1Prefix, char));

impl Match1 {
    pub fn iter<'a>(input: &'a str, valid: &'a BTreeSet<char>) -> Match1Iter<'a> {
        Match1Iter {
            chars: input.chars(),
            valid,
        }
    }

    pub fn get_prefix(&self) -> Match1Prefix {
        (self.0).0.clone()
    }

    pub fn from_str(s: &str) -> Match1 {
        let mut ch = s.chars();
        let first = ch.next().unwrap();
        Match1((first, first))
    }

    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(2);
        res.push((self.0).1);
        res
    }
}

pub struct Match1Iter<'a> {
    chars: Chars<'a>,
    valid: &'a BTreeSet<char>,
}

impl<'a> Iterator for Match1Iter<'a> {
    type Item = Match1;

    fn next(&mut self) -> Option<Match1> {
        loop {
            let cur = self.chars.next();
            match cur {
                Some(cur) => {
                    if self.valid.contains(&cur) {
                        return Some(Match1((cur, cur)));
                    }
                }
                None => return None,
            }
        }
    }
}

impl Model<Match1> {
    pub fn load() -> Self {
        let data = GzDecoder::new(Cursor::new(include_bytes!("model1.json.gz").to_vec()));
        let json_model: JsonModel = serde_json::from_reader(data).expect("json");
        let mut prob = BTreeMap::new();
        for (key, value) in &json_model.prob {
            prob.insert(Match1::from_str(key), *value);
        }

        Model {
            mapping: json_model.mapping,
            prob,
        }
    }

    pub fn save(&self) {
        let writer = GzEncoder::new(
            File::create("model1.json.gz").expect("open file"),
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

impl Match for Match1Prefix {
    type Prefix = Match1Prefix;
    fn min_len() -> usize {
        0
    }

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix> {
        unimplemented!()
    }

    fn shift_prefix(&self) -> Self::Prefix {
        *self
    }

    fn new(prefix: &Self::Prefix, end: char) -> Self {
        unimplemented!()
    }

    fn empty() -> Self {
        ' '
    }
}

impl Match for Match1 {
    type Prefix = Match1Prefix;

    fn min_len() -> usize {
        1
    }

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix> {
        let mut res = Vec::new();
        for word in input.iter() {
            assert!(word.len() == Self::min_len() - 1);
            res.push(word[0]);
        }
        res
    }

    fn shift_prefix(&self) -> Self::Prefix {
        (self.0).1
    }

    fn new(prefix: &Self::Prefix, end: char) -> Self {
        Match1((end, end))
    }

    fn empty() -> Self {
        Match1((' ', ' '))
    }
}
