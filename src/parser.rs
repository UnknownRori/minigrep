#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

pub struct Parser {
    mode: SearchMode,
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
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line.to_owned());
            }
        }

        result
    }
}
