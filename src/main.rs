use std::{env, io, collections::HashMap};

use shitStatTracker::User;


fn main() {
    println!("Input your name:");
    let mut name: String;
    name = String::new();
    io::stdin().read_line(&mut name);
    let mut user = User::init_user(name, HashMap::new());

    println!("Input the names of the stats to track:");
    let mut stat: String;
    stat = String::new();
    io::stdin().read_line(&mut stat);
    user.add_stat(stat, 0);
    

    println!("Username is {}.", user.user_name);


}