use std::fs::File;
use std::io::prelude::*;

/// Creates standard seed file
pub fn seed() {
    match create_seed_json() {
        Ok(()) => println!("Seed file generated"),
        Err(e) => eprintln!("new::seed() | {}", e),
    }
}

pub mod make {
    use crate::new;
    use std::fs;
    use std::fs::{File};
    use std::path::Path;
    use crate::build::markdown;

    pub fn blog() {
        match make_dir() {
            Ok(_) => println!("Build sucessful"),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn make_dir() -> std::io::Result<()> {
        fs::create_dir_all("./bin/posts/")?;
        
        //READS THE POSTS DIR
        let posts_paths = fs::read_dir("./bin/posts/")?;
        if posts_paths.count() == 0 {
            let markdown = File::create("./bin/posts/hello-world.md")?;
            new::create_example_markdown(markdown)?;
        } else {
            println!("Existing markdown detected");
            //Gets html from each post
            match markdown::gather() {
                Ok(_) => println!("{}", "Gather worked"),
                Err(e) => eprintln!("{}", e),
            }
        }

        

        Ok(())
    }
}

fn create_seed_json() -> std::io::Result<()> {
    println!("Supported types: Portfolio, Blog, Form");
    let seed_raw = 
r#"{
"type":"",
"head":"",
"footer":"",
"theme":""
}"#;
    let mut seed_file = File::create("seed.json")?;
    seed_file.write_all(seed_raw.as_bytes())?;
    Ok(())
}

// pub fn create_index_html(mut file: File) -> std::io::Result<()> {
//     let index_raw = 
// r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta charset="UTF-8">
//     <meta http-equiv="X-UA-Compatible" content="IE=edge">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>Made with Wingman</title>
// </head>
// <body>
    
// </body>
// </html>"#;
//     file.write_all(index_raw.as_bytes())?;
//     Ok(())
// }

fn create_example_markdown(mut file: File) -> std::io::Result<()> {
    let mk_raw = 
r##"
# Hello World
This is a test.
"##;

    file.write_all(mk_raw.as_bytes())?;

    Ok(())
}
