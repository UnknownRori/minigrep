use std::{env, fs::File, process};

use crate::{
    config::Config,
    parser::Parser,
    utility::{buffer_read, open_file},
    ErrorKind,
};

#[derive(Debug)]
pub struct Application {
    config: Config,
    file: File,
}

impl Application {
    fn new() -> Result<Application, ErrorKind> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        let config = Config::parse(&args)?;
        let file = open_file(&config.get_filename())?;

        Ok(Application { config, file })
    }

    pub fn run() {
        match Application::new() {
            Err(e) => handle_err(&e),
            Ok(app) => {
                if app.config.get_help() {
                    print_help();
                    process::exit(0);
                }

                let parser = Parser::new(&app.config.get_searchmode());
                let content = buffer_read(&app.file);

                let result = parser.parse(&app.config.get_query(), &content);

                for s in result {
                    println!("{s}");
                }
            }
        }
    }
}

fn handle_err(err: &ErrorKind) {
    match err {
        ErrorKind::QueryEmpty => eprintln!("Query is empty"),
        ErrorKind::FilenameEmpty => eprintln!("Filename is empty"),
        ErrorKind::FailedParseArgs => eprintln!("Failed to parse Args"),
        ErrorKind::NotEnoughArgs => {
            print_help();
            process::exit(0);
        }
        ErrorKind::FileErr(e) => eprintln!("Failed to open a file: {}", e),
    }
}

fn print_help() {
    println!("minigrep v1.0");
    println!("usage : minigrep <query> <filename>\n");
    println!("flags : ");
    println!("-h --help\t\t Showing this commands");
    println!("-q --query\t\t Search param");
    println!("-f --file\t\t File that will be searched");
    println!("-c --case-sensitive\t How sensitive search");
}
