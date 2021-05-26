use ansi_term::Style;
use forge::{plant::Plant};
use std::env;

mod forge;

fn main() {
    //Setup
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    //Start
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];
    let mut param: String = "".to_string();
    if args.len() == 3usize {
        param = args[2].to_owned();
    }

    if cmd == "--help" || cmd == "-h" {
        print_help();
    } else if cmd == "new" || cmd == "n" {
        let garden = Plant::new(param, None);
        match garden.plant() {
            Ok(x) => println!("Yay! Your new Wingman site is located at {}", x),
            Err(e) => eprint!("{}", e),
        }
    } else if cmd == "build" || cmd == "b" {
        todo!("Need to build the site still.");
        let garden = Plant::build_with_existing_seed(param);
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
