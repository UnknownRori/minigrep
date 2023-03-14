use std::mem;

use crate::{config::Config, parser::SearchMode, ErrorKind};

#[derive(Debug)]
pub struct DefaultConfig {
    query: Option<String>,
    filename: Option<String>,
    help: bool,
    search_mode: SearchMode,
}

impl DefaultConfig {
    pub fn new() -> DefaultConfig {
        DefaultConfig {
            query: None,
            filename: None,
            help: false,
            search_mode: SearchMode::CaseInsensitive,
        }
    }

    pub fn set_help(&mut self, new: bool) {
        self.help = new;
    }

    pub fn set_query(&mut self, new: Option<&String>) {
        if let Some(b) = new {
            self.query = Some(b.clone());
        }
    }

    pub fn set_file(&mut self, new: Option<&String>) {
        if let Some(b) = new {
            self.filename = Some(b.clone());
        }
    }

    pub fn set_case_sensitive(&mut self, new: Option<&String>) {
        if new.is_none() {
            return;
        }

        match new.unwrap().as_str() {
            "true" => self.search_mode = SearchMode::CaseSensitive,
            _ => {}
        }
    }
}

impl TryInto<Config> for DefaultConfig {
    type Error = ErrorKind;

    fn try_into(mut self) -> Result<Config, Self::Error> {
        let filename: Option<String> = mem::replace(&mut self.filename, None);
        let filename = filename.ok_or(ErrorKind::FilenameEmpty)?;

        let query: Option<String> = mem::replace(&mut self.query, None);
        let query = query.ok_or(ErrorKind::QueryEmpty)?;

        let help = self.help;
        let search_mode = self.search_mode.clone();

        Ok(Config::new(filename, query, help, search_mode))
    }
}
