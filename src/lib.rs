use std::fs;
use std::env;

pub struct User {
    pub charisma: i32,
    pub health: i32,
    pub intelligence: i32,
}

impl User {
    pub fn new() -> Result<User>{
        let charisma = 0;
        let health = 0;
        let intelligence = 0;
        
        Ok(User {
            charisma,
            health,
            intelligence,
        })
    }
}