use super::repository::Repository;

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
        repos.append(
            Repository::new(
                "manuals", "https://github.com/JiJogbf/manuals.git", "F:\\doc\\manuals"
            )
            
        );
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