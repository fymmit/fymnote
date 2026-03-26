use std::env::{self, VarError};

pub struct Config {
    pub editor: String,
    pub folder_path: String,
}

impl Config {
    pub fn new() -> Result<Config, VarError> {
        let editor = env::var("EDITOR")?;
        let folder_path = String::from("notes");
        Ok(Config {
            editor,
            folder_path,
        })
    }
}
