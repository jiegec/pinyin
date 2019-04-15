extern crate structopt;

use encoding_rs::GBK;
use pinyin;
use std::collections::{BTreeSet,BTreeMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct News {
    html: String,
    time: String,
    title: String,
    url: String
}

#[derive(StructOpt, Debug)]
#[structopt(name = "train")]
struct Opt {
    /// pinyin mapping file
    #[structopt(name = "pinyin", parse(from_os_str))]
    pinyin: PathBuf,

    /// data files
    #[structopt(name = "files", parse(from_os_str))]
    files: Vec<PathBuf>,
}
fn main() {
    let opt = Opt::from_args();
    let mut model: pinyin::Model<pinyin::Match2> = pinyin::Model::empty();

    // insert pinyin mapping
    let mut all_char: BTreeSet<char> = BTreeSet::new();
    let mut pinyin_data = Vec::new();
    File::open(opt.pinyin)
        .expect("pinyin")
        .read_to_end(&mut pinyin_data)
        .expect("read pinyin");
    let pinyin = GBK.decode(&pinyin_data).0;
    for line in pinyin.split(|ch| ch == '\r' || ch == '\n') {
        if line.is_empty() {
            continue;
        }
        let mut words = line.split(|ch| ch == ' ');
        let pinyin = String::from(words.next().unwrap());
        let chinese: Vec<char> = words.map(|s| s.chars().next().unwrap()).collect();

        for ch in chinese.iter() {
            all_char.insert(*ch);
        }

        model.mapping.insert(pinyin, chinese);
    }
    println!("Mapping {} pinyin", model.mapping.len());

    // collect probabilities
    let mut count: BTreeMap<pinyin::Match2Prefix, u32> = BTreeMap::new();
    let mut occur: BTreeMap<pinyin::Match2, u32> = BTreeMap::new();
    for file in opt.files {
        println!("Processing file {:?}", file);
        let mut data = Vec::new();
        File::open(file).expect("open").read_to_end(&mut data).expect("read");
        let content = GBK.decode(&data).0;
        for line in content.split(|ch| ch == '\r' || ch == '\n') {
            if line.is_empty() {
                continue;
            }
            let news: News = serde_json::from_str(line).expect("parsing");
            let match2_iter = pinyin::Match2::iter(&news.html, &all_char);
            for match2 in match2_iter {
                *count.entry(match2.get_prefix()).or_insert(0) += 1;
                *occur.entry(match2).or_insert(0) += 1;
            }
        }
    }

    for (match2, o) in &occur {
        let prob = (*o as f32) / (*count.get(&match2.get_prefix()).expect("found") as f32);
        model.prob.insert(*match2, prob);
    }

    println!("Saving...");
    model.save();
}
