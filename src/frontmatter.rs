use rayon::iter::{IntoParallelIterator, ParallelBridge};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Frontmatter {
    pub meta: Meta,
    pub body: String,
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Default)]
pub struct Meta {
    pub title: String,
    pub desc: Option<String>,
    pub keywords: Option<String>,
}

impl Frontmatter {
    pub fn new(buff: &str) -> anyhow::Result<Self> {
        let mut me = Self::default();
        me.extract(buff)?;

        Ok(me)
    }

    fn extract(&mut self, data: &str) -> anyhow::Result<()> {
        let mut switch = false;
        let mut buff = String::new();
        let mut count: i8 = 0;
        let mut body_line: usize = 0;
        for (i, line) in data.lines().into_iter().enumerate() {
            if count > 2 {
                body_line = i;
                break;
            }
            if line == "---" {
                count += 1;
                switch = !switch;
                continue;
            }

            if switch {
                buff.push_str(&format!("{line}\n"));
            }
        }

        let meta: Meta = serde_yaml::from_str(&buff)?;
        self.meta = meta;
        self.body = String::from(&data[body_line..data.len()]);
        Ok(())
    }
}
