use std::env;

enum ErrorKind {
    ParseArgs,
    OpenFile,
}

struct Application<'a> {
    config: Config<'a>,
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    case_sensitive: bool,
}

struct Parser;

impl<'a> Application<'a> {
    fn new() -> Application<'a> {
        todo!()
    }

    fn parse() -> Vec<&'a str> {
        todo!()
    }
}

impl<'a> Config<'a> {
    fn new(filename: &'a str, query: &'a str, case_sensitive: bool) -> Config<'a> {
        Config {
            filename,
            query,
            case_sensitive,
        }
    }

    fn parse() -> Result<Config<'a>, ErrorKind> {
        let args = env::args();

        if args.len() < 2 {
            return Err(ErrorKind::ParseArgs);
        }

        //properly parse cmd args to make config struct
        Ok(Config {
            query: todo!(),
            filename: todo!(),
            case_sensitive: todo!(),
        })
    }
}

impl<'a> Parser {
    fn parse(query: &str, filename: &'a str) -> Vec<&'a str> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    //
}
