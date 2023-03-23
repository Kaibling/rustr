
use clap::Parser;
use crypto::create_key_pair;
use std::fs;
use serde::{Deserialize, Serialize};
use entity::Event;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long,default_value_t = String::from(""))]
    content: String,
    #[arg(short, long, default_value_t = String::from("./private_key"))]
    key_file: String,
    #[arg(short, long, default_value_t = false)]
    generate_key: bool,
}




fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    if args.generate_key {
        let (private_key, public_key) = create_key_pair();
        let kp = KeyPair{private_key,public_key};
        let serialized_kp = serde_json::to_string(&kp).unwrap();
        fs::write(&args.key_file, serialized_kp).expect("Unable to write file");
        return;
    }
    // read keyfile
    let data = fs::read_to_string(&args.key_file).expect("Unable to read file");
    let key_pair: KeyPair = serde_json::from_str(&data).unwrap();
    let mut  e = Event::new(key_pair.public_key, args.content);
    e.sign(key_pair.private_key);
   
    
    println!(" {:?} ",e);
}


// cli -c "dfs fs s fgdg dfg dg" -k 
// cli -g

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct KeyPair {
    public_key: String,
    private_key: String,
}
