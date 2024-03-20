use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Frontmatter {
    pub meta: Meta,
    pub content: String,
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Default)]
pub struct Meta {
    pub title: String,
    pub desc: Option<String>,
    pub keywords: Option<Vec<String>>,
}

impl Frontmatter {
    pub fn new(buff: &str) -> anyhow::Result<Self> {
        let mut me = Self::default();
        me.extract(buff)?;

        Ok(me)
    }

    fn extract(&mut self, data: &str) -> anyhow::Result<()> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(data);

        if let Some(data) = result.data {
            let meta: Meta = data.deserialize()?;
            self.meta = meta;
        }

        self.content = result.content;

        Ok(())
    }
}
