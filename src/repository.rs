use std::process::Command;
use super::utils::{*};

///
/// Representation of real repository
/// 
pub struct Repository{
    name: String,
    dir: String,
    url: String
}

impl Repository{

    ///
    /// Allocate repository object
    /// 
    pub fn new(name: &str, 
        dir: &str, 
        url: &str
    )->Self{
        return Self{
            name: String::from(name), 
            dir: String::from(dir), 
            url: String::from(url)
        };
    }

    pub fn from(line: &str)->Self{
        let items = split_to_lines(&line, " ");
        // TODO: parse line to three items and 
        // store items to fileds of this Repositories
        Self::new(without_braces(&items[0]), without_braces(&items[1]),without_braces(&items[2]))
    }

    ///
    /// Pull 
    /// 
    pub fn pull(&self){
        let cmd = Command::new("git")
                    .args(&["-C", &self.dir, "pull"])
                    .output()
                    .expect("Failed pull this");
        match std::str::from_utf8(&cmd.stdout) {
            Ok(verbose) => {
                println!("ok: {}", verbose)
            },
            Err(error) => {
                println!("error: {}", error)
            }
        }
    }
}
