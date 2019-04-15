use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Debug;

mod match2;
pub use match2::*;

pub trait Match : Ord {
    type Prefix : Ord + Debug;

    fn new(prefix: &Self::Prefix, end: char) -> Self;

    fn min_len() -> usize;

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix>;

    fn shift_prefix(&self) -> Self::Prefix;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model<T: Match> {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<T, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModel {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<String, f32>,
}

impl<T: Match> Model<T> {
    pub fn empty() -> Self {
        Model {
            mapping: BTreeMap::new(),
            prob: BTreeMap::new(),
        }
    }

    /// Convert a pinyin sentence to chinese
    pub fn convert(&self, input: &str) -> String {
        let words: Vec<&str> = input.trim().split(|c| c == ' ').collect();
        if words.is_empty() {
            return String::new();
        }
        //println!("{:?}", words);
        let min_len = T::min_len();
        assert!(input.len() >= min_len);

        let mut prefixes: VecDeque<Vec<char>> = VecDeque::new();
        prefixes.push_back(Vec::new());
        for i in 0..(min_len-1) {
            let current = words[i];
            let chars = self.mapping.get(current).expect("found pinyin");
            loop {
                match prefixes.pop_front() {
                    Some(prefix) => {
                        if prefix.len() <= i {
                            for ch in chars {
                                let mut new_prefix = prefix.clone();
                                new_prefix.push(*ch);
                                prefixes.push_back(new_prefix);
                            }
                        } else {
                            prefixes.push_front(prefix);
                            break;
                        }
                    },
                    None => break
                }
            }
        }
        let vec_prefixes: Vec<Vec<char>> = prefixes.into_iter().collect();
        let cur_prefixes = T::get_prefix(&vec_prefixes);

        let mut cur_prefixes_prob: BTreeMap<T::Prefix, (f32, f32, Vec<char>)> = BTreeMap::new();
        for (i, cur_prefix) in cur_prefixes.into_iter().enumerate() {
            cur_prefixes_prob.insert(cur_prefix, (1.0, 0.0, vec_prefixes[i].clone()));
        }

        //println!("{:?}", cur_prefixes_prob);
        for i in (min_len-1)..words.len() {
            let current = words[i];
            let chars = self.mapping.get(current).expect("found pinyin");
            let mut new_prefixes_prob = BTreeMap::new();
            for (cur_prefix, (prob_prefix, _, path)) in cur_prefixes_prob.iter() {
                for ch in chars {
                    let new_match = T::new(cur_prefix, *ch);
                    if let Some(prob) = self.prob.get(&new_match) {
                        let entry = new_prefixes_prob.entry(new_match.shift_prefix()).or_insert_with(|| {
                            let mut new_path = path.clone();
                            new_path.push(' ');
                            (0.0, 0.0, new_path)
                        });
                        entry.0 += prob_prefix * prob;
                        if prob_prefix * prob > entry.1 {
                            entry.1 = prob_prefix * prob;
                            *entry.2.last_mut().unwrap() = *ch;
                        }
                    }
                }
            }
            cur_prefixes_prob = new_prefixes_prob;
            //println!("{:?}", cur_prefixes_prob);
        }

        let mut ans = Vec::new();
        let mut max_prob = 0.0;
        for (_, (prob, _, path)) in cur_prefixes_prob {
            if prob > max_prob {
                max_prob = prob;
                ans = path;
            }
        }
        ans.push('\n');
        ans.into_iter().collect()
    }
}
