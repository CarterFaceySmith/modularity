use std::{env, io, collections::HashMap};
use shitStatTracker::User;

fn main() {
    println!("Input your name: \n>");
    let mut name: String<::new();
    io::stdin().read_line(&mut name);
    let mut user = User::init_user(name, HashMap::new());

    println!("Input the names of the stats to track, or 0 to continue: \n>");
    let mut stat: String::new();
    io::stdin().read_line(&mut stat).expect("Failed to get console input");
    user.add_stat(stat, 0);
    

    println!("{}", user.user_name);

}