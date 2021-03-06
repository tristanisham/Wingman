use crate::forge::seed::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// The base of a Wingman Project.
pub struct Plant {
    pot: String,
    /// Holds the JSON seed file generated by *Seed.make_json()*.
    seed: String,
    seed_struct: Option<Seed>
}

impl Plant {
    pub fn new(path: String, seed_struct: Option<Seed>) -> Self {
        Plant {
            pot: path,
            seed: Seed::new("".to_string(), "".to_string()).make_json(),
            seed_struct,
        }
    }
    /// Sets up the project directory.
    pub fn plant(&self) -> std::io::Result<String> {
        let path = Path::new(&self.pot);
        fs::create_dir_all(path)?;

        let mut result = String::from("");
        if let Ok(x) = fs::canonicalize(path) {
            result = x.to_string_lossy().to_string();
        }
        self.create_config()?;

        Ok(result)
    }
    ///Creates the config file.
    fn create_config(&self) -> std::io::Result<()> {
        let path = Path::new(&self.pot);
        // creates config file
        let mut config = File::create(&path.join("seed.json"))?;
        config.write_all(&self.seed.as_bytes())?;
        // creates neccessary directories
        fs::create_dir_all(&path.join("posts"))?;
        Ok(())
    }
    /// Takes a ```Path``` and builds a ```Plant```
    pub fn build_with_existing_seed(path: String) -> Self {
        let existing: Seed = Seed::build_from_config(path.as_str());

        Self {
            pot: path,
            seed: existing.make_json(),
            seed_struct: Some(existing),
        }
        
    }
    /// Builds the final project.
    pub fn build(&self, flag: Option<&str>) -> std::io::Result<()> {
        let path = Path::new(&self.pot);
        match flag {
            None => fs::create_dir_all(&path.join("target"))?,
            Some(x) => {
                if x == "release" {
                    fs::create_dir_all(&path.join("target/release"))?
                }
            }
        } 
        Ok(())
    }
}
