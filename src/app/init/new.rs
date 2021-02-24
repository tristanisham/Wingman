use crate::build;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

/// Creates standard seed file
pub fn seed(param: String) {
    if param == "blog" {
        match create_blog_seed_json() {
            Ok(()) => println!("Seed file generated"),
            Err(e) => eprintln!("new::seed() | {}", e),
        }
    } else {
        match create_seed_json() {
            Ok(()) => println!("Seed file generated"),
            Err(e) => eprintln!("new::seed() | {}", e),
        }
    }
}

pub fn post() {
    let file = fs::read_to_string("seed.json").expect("Something went wrong reading the file");
    let seed = json::parse(&file).expect("Parsing failed");
    if !seed["type"].is_empty()
        && !seed["type"].is_null()
        && seed["type"].dump().to_lowercase() == "\"blog\""
    {
        build::generate();
        match make::post() {
            Ok(_) => println!("New Post Generated"),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        panic!("Requires 'Seed type: blog'")
    }
}

pub mod make {
    use crate::build::markdown;
    use crate::new;
    use chrono::prelude::*;
    use chrono::Utc;
    use std::fs;
    use std::fs::File;

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

    pub fn post() -> std::io::Result<()> {
        let path = get_post_date(Utc::now());
        File::create(path)?;
        Ok(())
    }
    fn get_post_date(now: chrono::DateTime<Utc>) -> String {
        let mut path = String::from("./bin/posts/");
        path.push_str(&now.year().to_string());
        path.push_str("-");
        path.push_str(&now.month().to_string());
        path.push_str("-");
        path.push_str(&now.day().to_string());
        path.push_str("-");
        path.push_str(&now.hour().to_string());
        path.push_str("-");
        path.push_str(&now.minute().to_string());
        path.push_str("-");
        path.push_str(&now.second().to_string());
        path.push_str(".md");

        return path;
    }
}

fn create_seed_json() -> std::io::Result<()> {
    println!("Supported types: Portfolio, Blog, Form");
    let seed_raw = r#"{
"type":"",
"head":"",
"vars": {
    "title":"",
    "description":"",
    "author":""
},
"footer":"",
"theme":""
}"#;
    let mut seed_file = File::create("seed.json")?;
    seed_file.write_all(seed_raw.as_bytes())?;
    Ok(())
}

fn create_blog_seed_json() -> std::io::Result<()> {
    println!("Supported types: Portfolio, Blog, Form");
    let seed_raw = r#"{
"type":"blog",
"head":"",
"vars": {
    "title":"",
    "description":"",
    "author":""
},
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
    let mk_raw = r##"
# Hello World
Welcome to Wingman! Generate your blog by writing markdown in as many files as you like here in the ./posts folder. 
The order your posts appear on the page depends on how you organize your directory. I suggest a YYYY--MM-DD-SLUG methodology for reverse chronological.
There will be further customization and parsing in the future.
"##;

    file.write_all(mk_raw.as_bytes())?;

    Ok(())
}
