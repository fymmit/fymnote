use std::env;

pub struct Config {
    pub editor: String,
    pub folder_path: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let editor = env::var("EDITOR").unwrap_or_else(|err| {
            return err.to_string();
        });
        let folder_path = String::from("notes");
        Ok(Config {
            editor,
            folder_path,
        })
    }
}
