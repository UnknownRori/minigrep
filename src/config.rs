use crate::{default_config::DefaultConfig, parser::SearchMode, ErrorKind};

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    help: bool,
    search_mode: SearchMode,
}

impl Config {
    pub fn new(filename: String, query: String, help: bool, search_mode: SearchMode) -> Config {
        Config {
            filename,
            query,
            help,
            search_mode,
        }
    }

    pub fn parse(args: &Vec<String>) -> Result<Config, ErrorKind> {
        if args.len() < 1 {
            return Err(ErrorKind::NotEnoughArgs);
        }

        let mut config = DefaultConfig::new();
        for mut i in 0..args.len() {
            let arg = args.get(i);

            if arg.is_none() {
                break;
            }

            match arg.unwrap().as_str() {
                "--help" | "-h" => config.set_help(true),
                "--case-sensitive" | "-c" => {
                    i += 1;
                    config.set_case_sensitive(args.get(i));
                }
                "--filename" | "-f" => {
                    i += 1;

                    config.set_file(args.get(i));
                }
                "--query" | "-q" => {
                    i += 1;

                    config.set_query(args.get(i));
                }
                _ => {
                    if i == 0 {
                        config.set_query(arg);
                    } else if i == 1 {
                        config.set_file(arg);
                    }
                }
            }
        }

        Ok(config.try_into()?)
    }

    pub fn get_help(&self) -> bool {
        self.help
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_searchmode(&self) -> &SearchMode {
        &self.search_mode
    }
}
