use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        let readhst = fs::read_to_string("/etc/hostname").expect("could not read /etc/hostname");
        println!("current hostname: {}", readhst.as_str())
        
    } else {
        sudo::escalate_if_needed().expect("could not escalate to sudo privleges.");
        fs::write("/etc/hostname", args[1].as_str()).expect("failed to write hostname");
        println!("set host to {}", args[1].as_str())
    
    }
    
}
