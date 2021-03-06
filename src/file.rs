use std::fs;

///
/// Encapsulated filepath to specific file
/// with basic file supported operation's
/// 
pub struct File{
    path: String
}

impl File{
    ///
    /// Construct file object from string path
    /// 
    pub fn from(path: &str)->Self{
        Self{path: String::from(path)}
    }

    ///
    /// Reading all availalbe string content from file 
    /// assuming we dealing with text file.
    /// 
    pub fn content(&self)->String{ 
        match fs::read_to_string(&self.path) {
            Ok(text) => {
                println!("text={}", text);
                text
            },
            Err(error) => {
                println!("error: {}", error);
                String::from("")
            }
        }
    }
}

///
/// Implementation of `Display` trait for `File` struct
/// 
impl std::fmt::Display for File{

    ///
    /// Implementation of fmt method for better output 
    /// File propertie's
    /// 
    fn fmt(&self, f:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(f, "{}", self.path)
    }
}

