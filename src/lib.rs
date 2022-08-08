use std::fs;
use std::env;
use std::collections::HashMap;
use std::dbg;

pub struct User {
    pub user_name: String,
    pub stats: HashMap<String, i32>,
}

// pub struct Stat {
//     pub stat_name: String,
//     pub value: i32,
// }

impl User {
    pub fn init_user(name: String, stats: HashMap<String, i32>) -> User {
        return User {user_name: name.to_string(), stats: stats};
    }

    pub fn add_stat(& mut self, name:String, val:i32){
        self.stats.insert(name, val);
    }

    pub fn remove_stat(& mut self, name: String){
        self.stats.remove(&name);
    }

    pub fn update_stat(& mut self, name:String, val:i32){
        *self.stats.get_mut(&name).unwrap() = val;
    }
}