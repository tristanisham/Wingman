use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::path::Path;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// This is the struct Wingman's config file is built off of.
#[derive(Serialize, Deserialize, Debug)]
pub struct Seed {
    title: String,
    version: String,
    theme: String,
}

impl Seed {
    pub fn new(title: String, theme: String) -> Self {
        Self {
            title,
            version: VERSION.to_string(),
            theme,
        }
    }

    pub fn make_json(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        return json;
    }

    pub fn build_from_config(path: &str) -> Self {
        let reform = |x: String| {
            let j: Seed = serde_json::from_str(x.as_str()).unwrap();
            return j;
        };

        if path == "" {
            let here = env::current_dir().unwrap();
            let mut file = File::open(here.as_path().join("seed.json")).expect("Unable to open seed.json.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read seed.json");
            return reform(contents);
        } else {
           let here = Path::new(path);
           let mut file = File::open(here.join("seed.json")).expect("Unable to open seed.json.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read seed.json");
            return reform(contents);
        }
    }
}

