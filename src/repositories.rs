use super::repository::Repository;
use super::utils::{without_braces, split_to_lines};
/// 
/// Container for all added repositories
/// 
pub struct Repositories{
    items: Vec<Repository>
}

impl Repositories{
    
    /// 
    /// Allocate container
    /// 
    pub fn new()->Self{
        return Self{items: Vec::new()}
    }

    ///
    /// Construct container from string
    /// 
    pub fn from(_content: String)->Self{
        // todo: parsing repositories and adding 
        // to constructed object
        let mut repos = Self::new();
        let lines = split_to_lines(&_content.as_str(), "\n");
        for line in lines.iter(){
            repos.append(Repository::from(line));         
        }
        return repos;
    }

    ///
    /// Add repository 
    /// 
    pub fn append(&mut self, repo: Repository){
        self.items.push(repo);
    }

    /// 
    /// Pull each repository
    /// 
    pub fn pull(&self){
        for repo in self.items.iter() {           
            repo.pull();
        }
    }
}
