use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("wmctrl")
        .arg("-l")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines:Vec<&str> = stdout.lines().collect();
    
    for i in lines.iter() {
        let split:Vec<&str> = i.split_whitespace().collect();
        println!("{}", split[1])
    }

}

