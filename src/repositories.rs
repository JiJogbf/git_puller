use super::repository::Repository;

pub struct Repositories{
    items: Vec<Repository>
}

impl Repositories{
    pub fn new()->Self{
        return Self{items: Vec::new()}
    }

    pub fn from(_content: String)->Self{
        return Self::new()
    }

    pub fn append(&mut self, repo: Repository){
        self.items.push(repo);
    }

    pub fn pull(&self){
        for repo in self.items.iter() {           
            repo.pull();
        }
    }
}