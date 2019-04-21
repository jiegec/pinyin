use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Debug;

mod match1;
pub use match1::*;

mod match2;
pub use match2::*;

pub trait Match: Ord + Debug {
    type Prefix: Ord + Debug + Match;

    fn new(prefix: &Self::Prefix, end: char) -> Self;

    fn min_len() -> usize;

    fn get_prefix(input: &[Vec<char>]) -> Vec<Self::Prefix>;

    fn shift_prefix(&self) -> Self::Prefix;
    
    fn empty() -> Self;
}

#[derive(Debug)]
pub struct Model<T: Match> {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<T, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModel {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<String, f64>,
}

impl<T: Match> Model<T> {
    pub fn empty() -> Self {
        Model {
            mapping: BTreeMap::new(),
            prob: BTreeMap::new(),
        }
    }

    /// Convert a pinyin sentence to chinese
    pub fn convert(
        &self,
        words: &Vec<&str>,
        last_model: Option<
            BTreeMap<<<T as Match>::Prefix as Match>::Prefix, (f64, f64, Vec<char>)>,
        >,
    ) -> (String, BTreeMap<T::Prefix, (f64, f64, Vec<char>)>) {
        let min_len = T::min_len();
        assert!(words.len() >= min_len);

        let mut cur_prefixes_prob: BTreeMap<T::Prefix, (f64, f64, Vec<char>)> = BTreeMap::new();
        if let Some(last) = last_model {
            let current = words[min_len-1];
            let chars = self.mapping.get(current).expect("found pinyin");
            for ch in chars {
                for (prefix, (old_prob, sum_prob, path)) in last.iter() {
                    let new_prefix = T::Prefix::new(&prefix, path[path.len()-1]);
                    let new_match = T::new(&new_prefix, *ch);
                    if let Some(prob) = self.prob.get(&new_match) {
                        cur_prefixes_prob.insert(new_prefix, (*old_prob * prob, 0.0, path.clone()));
                    }
                }
            }
        } else {
            cur_prefixes_prob.insert(T::Prefix::empty(), (1.0, 0.0, Vec::new()));
        }
        for i in (min_len - 1)..words.len() {
            let current = words[i];
            let chars = self.mapping.get(current).expect("found pinyin");
            let mut new_prefixes_prob = BTreeMap::new();
            for (cur_prefix, (prob_prefix, _, path)) in cur_prefixes_prob.iter() {
                for ch in chars {
                    let new_match = T::new(cur_prefix, *ch);
                    if let Some(prob) = self.prob.get(&new_match) {
                        let entry = new_prefixes_prob
                            .entry(new_match.shift_prefix())
                            .or_insert_with(|| {
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
        }

        let mut ans = Vec::new();
        let mut max_prob = 0.0;
        for (_, (prob, _, path)) in &cur_prefixes_prob {
            if *prob > max_prob {
                max_prob = *prob;
                ans = path.clone();
            }
        }
        ans.push('\n');
        (ans.into_iter().collect(), cur_prefixes_prob)
    }
}
