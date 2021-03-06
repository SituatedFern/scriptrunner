use std::process::{Command, Stdio};
use itertools::Itertools;

fn main() {
    /*TODO 
     * allow the user to input a number as an argument (check git history) to check if it's in the vector dedup.
     */
    let output = Command::new("wmctrl")
        .arg("-l")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines:Vec<&str> = stdout.lines().collect();
    let mut numlist = Vec::new();

    for i in lines.iter() {
        let split:Vec<&str> = i.split_whitespace().collect();
        numlist.push(split[1]);
        
    }
    let dedup: Vec<_> = numlist.iter().unique().collect();
    println!{"{:?}", dedup};
}

