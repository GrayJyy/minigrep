use std::{error::Error, ffi::OsString, fs};
pub struct Config<'a> {
    pub query: &'a OsString,
    pub file_path: &'a OsString,
}
impl<'a> Config<'a> {
    pub fn build(args: &[OsString]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Config { query, file_path })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.file_path)?;
        println!("With text:\n{contents}");
        Ok(())
    }
}
