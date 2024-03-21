# Wingman

A runtime for websites and a static site generator.

Usage: wingman [COMMAND]

## Commands: 
- **init** Initalize a new Wingman project 
- **build** Build your Wingman project in the specified distribution directory 
- **serve** Serve your site on a production
web server help Print this message or the help of the given subcommand(s)

## Options:
- *-h*, *--help* Print help 
- *-V*, *--version* Print version

## About

Wingman is a command line tool written in and library for people who want to
make a website. Let's go through setting up a new Wingman project.
```sh
cargo install wingman # If you have Rust installed, install Wingman from crates.io.
```

Open up your terminal and create a new empty directory. Then, run:
```sh
wingman init -f # In a nonempty directory, use `-f` to force initialize a new Wingman project.
```

Build your website with:
```sh
wingman build
```
Or rebuild on file change:
```sh
wingman build --watch
```

View your website locally.
```sh
wingman serve --port 3030 # The port flag is option. Wingman defaults to port 3030.
```