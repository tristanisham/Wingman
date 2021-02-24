use json;
use std::fs;
use std::path::Path;

use crate::new;

pub fn generate() {
    let seed_json = Path::new("seed.json");
    if !seed_json.exists() {
        new::seed("".to_string());
        println!("Please run again after filling out required fields.");
    } else {
        parse();
    }
}

fn parse() {
    let file = fs::read_to_string("seed.json").expect("Something went wrong reading the file");
    let seed = json::parse(&file).expect("Parsing failed");
    if seed["type"].is_empty() || seed["type"].is_null() {
        panic!("Please enter a type in 'seed.json' before running 'build'.");
    } else {
        let build = seed["type"].dump().to_lowercase();
        if build == "\"blog\"" {
            println!("Type: blog detected");
            new::make::blog();
        }
    }
}

pub mod markdown {
    // use crate::new;
    use pulldown_cmark::{html, Options, Parser};
    // use std::ffi::OsStr;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    // use regex::Regex;
    // use std::path::Path;
    pub fn gather() -> std::io::Result<()> {
        let posts = fs::read_dir("./bin/posts/")?;
        let mut files: Vec<String> = Vec::new();
        // let mut file_name: Vec<&std::ffi::OsStr> = Vec::new();
        for post in posts {
            let p = post?;
            // match p.path().file_name() {
            //     Some(x) => file_name.push(x),
            //     None => break
            // }
            let content = fs::read_to_string(p.path())?;
            // println!("{:?}", content);
            md_html(content, &mut files);
        }

        let seed_js =
            fs::read_to_string("seed.json").expect("Something went wrong reading the file");
        let seed = json::parse(&seed_js).expect("Parsing failed");
        // VEC for total number of posts
        // let mut ids: Vec<u32> = Vec::new();

        let mut result: String = "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <meta name=\"description\" content="
            .to_string();
        //Description
        if !&seed["vars"]["description"].is_empty() || !&seed["vars"]["description"].is_null() {
            result.push_str(&seed["vars"]["description"].dump().to_string());
        } else {
            result.push_str("\"\"");
        }
        result.push_str(">");
        //Description
        if !&seed["vars"]["description"].is_empty() || !&seed["vars"]["description"].is_null() {
            result.push_str("<meta name=\"author\" content=");
            result.push_str(&seed["vars"]["author"].dump().to_string());
        } else {
            result.push_str("\"\"");
        }
        result.push_str(">");

        result.push_str(
            "
        <title>
            ",
        );
        //set title
        let vars_title = &seed["vars"]["title"];
        if !vars_title.is_empty() || !vars_title.is_null() {
            result.push_str(&vars_title.dump());
        }
        // result.push_str(&seed["vars"]["title"].dump().to_string());

        result.push_str(
            "
        </title>
        </head>
        <body>",
        );
        let mut placement = 0;
        //gets number of posts
        // Uses vec ID up by top of function

        // for _content in &files {
        //     ids.push(placement);
        //     placement += 1;
        // }
        // for x in ids{ println!("{}", x);}
        // placement = 0;

        for content in files {
            result.push_str("<article id =\"");
            let str_plc: &str = &placement.to_string()[..];
            result.push_str(str_plc);
            result.push_str("\">\n");
            placement += 1;
            result.push_str(&content);
            result.push_str("</article>");
        }

        result.push_str("<footer>");
        // result.push_str("<au");
        result.push_str("</footer>\n</body>\n</html>");
        //OUTPUT AND BUILD FINALIZES
        let index = File::create("./bin/index.html")?;
        write_index(index, result)?;
        Ok(())
    }

    fn md_html(file: String, vec: &mut std::vec::Vec<std::string::String>) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&file, options);

        let mut html = String::new();
        html::push_html(&mut html, parser);
        vec.push(html.clone());
    }

    fn write_index(mut file: File, string: String) -> std::io::Result<()> {
        let input = string.as_bytes();
        file.write_all(input)?;
        Ok(())
    }
}
