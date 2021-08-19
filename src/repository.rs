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
    pub fn new(name: String, 
        url: String, 
        dir: String
    )->Self{
        return Self{name, url, dir};
    }

    ///
    /// Pull 
    /// 
    pub fn pull(&self){
        // todo: go to 'dir'
        // call git pull
        // read error code of executed command       
    }
}
