use ansi_term::Style;
use app::init::{build, new};
use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];
    let mut _param: &String;

    if args.len() == 3usize {
        _param = &args[2];
    }

    if cmd == "--help" || cmd == "-h" {
        let help_array = vec!["'--help' or '-h'", "'new' or 'n'", "'build' or 'b'"];

        for x in &help_array {
            println!("{}", x);
        }
    } else if cmd == "new" || cmd == "n" {
        new::seed();
    } else if cmd == "build" || cmd == "b" {
        build::generate();
    } else {
        println!(
            "{}\n{}",
            Style::new().bold().paint("Invalid Argument"),
            Style::new().italic().paint("Here are available arguments:")
        );
        // Future good use of a !
        let help_array = vec!["'--help' or '-h'", "'new' or 'n'", "'build' or 'b'"];

        for x in &help_array {
            println!("{}", x);
        }
    }
}
