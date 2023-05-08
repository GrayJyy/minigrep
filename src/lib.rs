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
        eprintln!("With text:\n{contents}");
        Ok(())
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }
