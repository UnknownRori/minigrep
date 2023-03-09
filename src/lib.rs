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
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        let config = Config::parse(&args)?;

        Ok(Application { config })
    }

    pub fn parse(&self) -> Result<Vec<String>, ErrorKind> {
        let parser = Parser::new();
        let content = read_to_string(&self.config.filename)?;

        Ok(parser.parse(&self.config.query, &content))
    }
}

impl Config {
    #[allow(unused)]
    fn new(filename: &str, query: &str, case_sensitive: bool) -> Config {
        Config {
            filename: filename.to_owned(),
            query: query.to_owned(),
            case_sensitive,
        }
    }

    fn parse(args: &Vec<String>) -> Result<Config, ErrorKind> {
        if args.len() < 2 {
            return Err(ErrorKind::NotEnoughArgs(args.len()));
        }

        let mut query: Option<&str> = None;
        let mut filename: Option<&str> = None;
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
                            query = Some(b);
                        }
                        None => return Err(ErrorKind::ParseArgs),
                    }
                }
                "--query" | "-q" => {
                    i += 1;

                    match args.get(i) {
                        Some(b) => {
                            query = Some(b);
                        }
                        None => return Err(ErrorKind::ParseArgs),
                    }
                }
                _ => {
                    if i == 0 {
                        query = Some(arg.unwrap());
                    } else if i == 1 {
                        filename = Some(arg.unwrap());
                    }
                }
            }
        }

        if query.is_none() {
            return Err(ErrorKind::QueryEmpty);
        }

        if filename.is_none() {
            return Err(ErrorKind::FilenameEmpty);
        }

        Ok(Config::new(
            filename.unwrap(),
            query.unwrap(),
            case_sensitive,
        ))
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, query: &str, content: &str) -> Vec<String> {
        let mut result = vec![];

        for line in content.lines() {
            result.push(line.to_owned());
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
    use super::*;

    #[test]
    fn case_sensitive_1() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseSensitive).parse("lorem", content);

        assert_eq!(vec!["lorem"], result);
    }

    #[test]
    fn case_sensitive_2() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseSensitive).parse("flying", content);

        assert_eq!(vec!["flying dog", "flying cat"], result);
    }

    #[test]
    fn case_insensitive_1() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseInsensitive).parse("lorem", content);

        assert_eq!(vec!["Lorem ipsum", "LOREM IPSUM", "lorem"], result);
    }

    #[test]
    fn case_insensitive_2() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseSensitive).parse("Flying Cat", content);

        assert_eq!(vec!["Flying Cat"], result);
    }
}
