pub struct Repository{
    name: String,
    url: String, 
    dir: String
}

impl Repository{
    pub fn new(name: String, 
        url: String, 
        dir: String
    )->Self{
        return Self{name, url, dir};
    }

    pub fn pull(&self){
        // todo: go to 'dir'
        // call git pull
        // read error code of executed command       
    }
}
