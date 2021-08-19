use std::process::Command;

///
/// Representation of real repository
/// 
pub struct Repository{
    name: String,
    url: String, 
    dir: String
}

impl Repository{

    ///
    /// Allocate repository object
    /// 
    pub fn new(name: &str, 
        url: &str, 
        dir: &str
    )->Self{
        return Self{name: String::from(name), 
            url: String::from(url), 
            dir: String::from(dir)
        };
    }

    ///
    /// Pull 
    /// 
    pub fn pull(&self){
        let cmd = Command::new("git")
                    .args(&["pull"])
                    .output()
                    .expect("Failed pull this");
        let text = cmd.stdout;
        let t = std::str::from_utf8(&text);
        match t {
            Ok(v) => {
                println!("ok: {}", v)
            }
            Err(e) => {
                println!("error: {}",e)
            }
        }
    }
}
