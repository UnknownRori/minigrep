use std::{env, error::Error};

pub enum ErrorKind {
    QueryEmpty,
    FilenameEmpty,
    ParseArgs,
    OpenFile(Box<dyn Error>),
    NotEnoughArgs(usize),
}

#[derive(Debug)]
pub struct Application {
    config: Config,
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

struct Parser;

impl Application {
    pub fn new() -> Result<Application, ErrorKind> {
        Ok(Application {
            config: Config::parse()?,
        })
    }

    pub fn parse(&self) -> Result<Vec<&str>, ErrorKind> {
        let parser = Parser::new();
        let content = read_to_string(&self.config.filename)?;

        Ok(parser.parse(&self.config.query, content))
    }
}

impl Config {
    fn new(filename: String, query: String, case_sensitive: bool) -> Config {
        Config {
            filename,
            query,
            case_sensitive,
        }
    }

    fn parse() -> Result<Config, ErrorKind> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        if args.len() < 2 {
            return Err(ErrorKind::NotEnoughArgs(args.len()));
        }

        let mut filename: String = String::new();
        let mut query: String = String::new();
        let mut case_sensitive = false;
        for mut i in 0..args.len() {
            let arg = args.get(i);

            if arg.is_none() {
                break;
            }

            // Todo : Refactor this
            match arg.unwrap().as_str() {
                "--case-sensitive" | "-c" => {
                    i += 1;
                    match args.get(i) {
                        Some(b) => {
                            if b.contains("true") {
                                case_sensitive = true;
                            }
                        }
                        None => return Err(ErrorKind::ParseArgs),
                    }

                    continue;
                }
                "--filename" | "-f" => {
                    i += 1;

                    match args.get(i) {
                        Some(b) => {
                            query = b.clone();
                        }
                        None => return Err(ErrorKind::ParseArgs),
                    }
                }
                "--query" | "-q" => {
                    i += 1;

                    match args.get(i) {
                        Some(b) => {
                            query = b.clone();
                        }
                        None => return Err(ErrorKind::ParseArgs),
                    }
                }
                _ => {
                    if i == 0 {
                        query = arg.unwrap().clone();
                    } else if i == 1 {
                        filename = arg.unwrap().clone();
                    }
                }
            }
        }

        if query.is_empty() {
            return Err(ErrorKind::QueryEmpty);
        }

        if filename.is_empty() {
            return Err(ErrorKind::FilenameEmpty);
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, query: &String, content: String) -> Vec<&str> {
        let mut result = vec![];

        for line in content.lines() {
            result.push(line);
        }

        result
    }
}

fn read_to_string(filename: &str) -> Result<String, ErrorKind> {
    match std::fs::read_to_string(filename).or_else(|v| Err(ErrorKind::OpenFile(Box::new(v)))) {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    //
}
