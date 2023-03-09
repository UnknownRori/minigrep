use std::{env, error::Error};

#[derive(Debug)]
pub enum ErrorKind {
    QueryEmpty,
    FilenameEmpty,
    ParseArgs,
    OpenFile(Box<dyn Error>),
    NotEnoughArgs(usize),
}

#[derive(Debug, Clone)]
enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

#[derive(Debug)]
pub struct Application {
    config: Config,
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
    search_mode: SearchMode,
}

struct Parser {
    mode: SearchMode,
}

impl Application {
    pub fn new() -> Result<Application, ErrorKind> {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        let config = Config::parse(&args)?;

        Ok(Application { config })
    }

    pub fn run(&self) -> Result<Vec<String>, ErrorKind> {
        let parser = Parser::new(&self.config.search_mode);
        let content = read_to_string(&self.config.filename)?;

        Ok(parser.parse(&self.config.query, &content))
    }
}

impl Config {
    fn new(filename: &str, query: &str, search_mode: SearchMode) -> Config {
        Config {
            filename: filename.to_owned(),
            query: query.to_owned(),
            search_mode,
        }
    }

    fn parse(args: &Vec<String>) -> Result<Config, ErrorKind> {
        if args.len() < 2 {
            return Err(ErrorKind::NotEnoughArgs(args.len()));
        }

        let mut query: Option<&str> = None;
        let mut filename: Option<&str> = None;
        let mut search_mode = SearchMode::CaseInsensitive;
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
                                search_mode = SearchMode::CaseSensitive;
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

        Ok(Config::new(filename.unwrap(), query.unwrap(), search_mode))
    }
}

impl Parser {
    pub fn new(mode: &SearchMode) -> Parser {
        Parser { mode: mode.clone() }
    }

    pub fn parse(&self, query: &str, content: &str) -> Vec<String> {
        match self.mode {
            SearchMode::CaseSensitive => self.parse_case_sensitive(query, content),
            SearchMode::CaseInsensitive => self.parse_case_insensitive(query, content),
        }
    }

    fn parse_case_sensitive(&self, query: &str, content: &str) -> Vec<String> {
        let mut result = vec![];

        for line in content.lines() {
            if line.contains(query) {
                result.push(line.to_owned());
            }
        }

        result
    }

    fn parse_case_insensitive(&self, query: &str, content: &str) -> Vec<String> {
        let mut result = vec![];

        for line in content.lines() {
            if line.to_lowercase().contains(query) {
                result.push(line.to_owned());
            }
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
