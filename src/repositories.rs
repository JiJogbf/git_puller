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
        let lines = split_to_lines(&_content.as_str(), "\n");
        for line in lines.iter(){
            let items = split_to_lines(&line, " ");
            repos.append(
                Repository::new(
                    without_braces(&items[0]), 
                    without_braces(&items[1]),
                    without_braces(&items[2])
                )
            );         
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

fn without_braces(source: &str)->&str{
    // удалить сначала кавычку и потом так же 
    // удалить кавычку 
    let mut chars = source.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}


fn split_to_lines(content: &str, something: &str)->Vec<String>{
    return content.split(something)
        .map(|s| s.to_string())
        .collect();
}