use ansi_term::Style;
use app::init::{build, new};
use std::env;

mod app;

fn main() {
    //Setup
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    //Start
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];
    let mut param: &String = &String::from("");
    if args.len() == 3usize {
        param = &args[2];
    }

    if cmd == "--help" || cmd == "-h" {
        print_help();
    } else if cmd == "new" || cmd == "n" {
        if param == "" {
            new::seed("".to_string());
        } else if param == "--blog" {
            new::seed("blog".to_string())
        } else if param == "post" {
            new::post();
        } else {
            println!(
                "{}\n{}",
                Style::new().bold().paint("Invalid Argument"),
                Style::new().italic().paint("Here are available arguments:")
            );
            print_help();
        }
    } else if cmd == "build" || cmd == "b" {
        build::generate();
    } else if cmd == "-V" {
        println!("Wingman - v{}", VERSION)
    } else {
        println!(
            "{}\n{}",
            Style::new().bold().paint("Invalid Argument"),
            Style::new().italic().paint("Here are available arguments:")
        );
        print_help();
    }

    fn print_help() {
        // Future good use of a !
        let help_array = vec![
            "'--help' or '-h'",
            "'new' or 'n'",
            "'build' or 'b', -V for version",
        ];

        for x in &help_array {
            println!("{}", x);
        }

        println!(
            "\n\n{}\n {} \n{}",
            Style::new().bold().paint("'new' flags:"),
            Style::new().italic().paint("--blog\n"),
            Style::new().italic().paint("post\n")
        );
    }
}
