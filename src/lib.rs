mod config;
mod default_config;

pub mod application;
pub mod parser;
pub mod utility;

#[derive(Debug)]
pub enum ErrorKind {
    QueryEmpty,
    FilenameEmpty,
    FailedParseArgs,
    NotEnoughArgs,
    FileErr(std::io::Error),
}
#[cfg(test)]
mod test {
    use crate::config::Config;

    use super::parser::{Parser, SearchMode};

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
    fn case_sensitive_3() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseSensitive).parse("lOreM", content);

        let expected: Vec<String> = vec![];
        assert_eq!(expected, result);
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

    #[test]
    fn case_insensitive_3() {
        let content = "Lorem ipsum\nFlying Cat\nflying dog\nLOREM IPSUM\nlorem\nflying cat";
        let result = Parser::new(&SearchMode::CaseInsensitive).parse("LOREM", content);

        assert_eq!(vec!["Lorem ipsum", "LOREM IPSUM", "lorem"], result);
    }

    #[test]
    #[should_panic]
    fn parse_config_none() {
        let args = vec![];
        Config::parse(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_config_1_args() {
        let args = vec!["lorem".to_owned()];
        Config::parse(&args).unwrap();
    }

    #[test]
    fn parse_config_2_args() {
        let args = vec!["lorem".to_owned(), "ipsum".to_owned()];
        let config = Config::parse(&args).unwrap();

        assert_eq!(config.get_filename(), "ipsum");
        assert_eq!(config.get_query(), "lorem");
        assert_eq!(config.get_searchmode(), &SearchMode::CaseInsensitive);
    }

    #[test]
    fn parse_config_2_with_flags() {
        let args = vec![
            "-f".to_owned(),
            "ipsum".to_owned(),
            "-q".to_owned(),
            "lorem".to_owned(),
        ];
        let config = Config::parse(&args).unwrap();

        assert_eq!(config.get_filename(), "ipsum");
        assert_eq!(config.get_query(), "lorem");
        assert_eq!(config.get_searchmode(), &SearchMode::CaseInsensitive);
    }
}
