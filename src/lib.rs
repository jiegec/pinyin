use std::collections::BTreeMap;
use std::io::Cursor;
use std::fs::File;
use serde::{Serialize, Deserialize};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

mod match2;
pub use match2::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model<T: Ord> {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<T, f32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModel {
    pub mapping: BTreeMap<String, Vec<char>>,
    pub prob: BTreeMap<String, f32>
}

impl Model<Match2> {
    pub fn load() -> Self {
        let data = GzDecoder::new(Cursor::new(include_bytes!("model2.json.gz").to_vec()));
        let json_model: JsonModel = serde_json::from_reader(data).expect("json");
        let mut prob = BTreeMap::new();
        for (key, value) in &json_model.prob {
            prob.insert(Match2::from_str(key), *value);
        }
        println!("Loaded {} pinyin mapping and {} probs", json_model.mapping.len(), json_model.prob.len());

        Model {
            mapping: json_model.mapping,
            prob
        }
    }

    pub fn save(&self) {
        let writer = GzEncoder::new(File::create("model2.json.gz").expect("open file"), Compression::default());

        let mut prob = BTreeMap::new();
        for (key, value) in &self.prob {
            prob.insert(key.to_string(), *value);
        }

        let json_model = JsonModel {
            mapping: self.mapping.clone(),
            prob
        };
        serde_json::to_writer(writer, &json_model).expect("json");
    }
}

impl<T: Ord> Model<T> {
    pub fn empty() -> Self {
        Model {
            mapping: BTreeMap::new(),
            prob: BTreeMap::new()
        }
    }

    /// Convert a pinyin sentence to chinese
    pub fn convert(&self, input: &str) -> String {
        let words: Vec<&str> = input.trim().split(|c| c == ' ').collect();
        println!("{:?}", words);
        String::new()
    }
}
