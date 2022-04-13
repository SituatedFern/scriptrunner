use std::process::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    Command::new("zsh")
    	.arg("-C")
    	.arg("/home/ashwin/.config/eww/scripts/checkmatch.sh")
        .arg(query)
    	.spawn()
    	.expect("sh command failed to start");
}


