use serde::{Serialize, Deserialize};
use serde_json;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// This is the struct Wingman's config file is built off of.
#[derive(Serialize, Deserialize, Debug)]
pub struct Seed {
    title: String,
    version: String,
    theme: String,
}

impl Seed {
    pub fn new(title: String, version: String, theme: String) -> Self {
        Self {
            title,
            version,
            theme,
        }
    }

    pub fn make_json() -> String {
        let base = Seed::new("".to_string(), VERSION.to_string(),"".to_string());
        let json = serde_json::to_string(&base).unwrap();
        return json;
    }
}

