use std::env;

// use shitStatTracker::User;

struct User {
    pub charisma: i32,
    pub health: i32,
    pub intelligence: i32,
}

fn main() {
    let intelligence = 0;
    let health = 0;
    let charisma = 0;
    let carter = User { charisma, health, intelligence};

    println!("{:?}", carter.health);

}