use std::process::*;

fn main() {
    Command::new("sh")
    	.arg("-C")
    	.arg("/home/ashwin/Projects/scriptrunner/cringe.sh")
    	.spawn()
    	.expect("sh command failed to start");
}


