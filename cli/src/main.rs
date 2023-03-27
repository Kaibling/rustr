
use clap::Parser;
use crypto::create_key_pair;
use std::fs;
use serde::{Deserialize, Serialize};
use entity::Event;
use reqwest::header::CONTENT_TYPE;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long,default_value_t = String::from(""))]
    content: String,
    #[arg(short, long, default_value_t = String::from("./private_key"))]
    key_file: String,
    #[arg(short, long, default_value_t = false)]
    generate_key: bool,
    #[arg(short, long, default_value_t = false)]
    read: bool,
    #[arg(short, long, default_value_t = String::from("events"))]
    object_type: String,
    #[arg(short, long, default_value_t = String::from(""))]
    id: String,
}




fn main() {
    let args = Args::parse();

    if args.read {
        if args.id == "" {
            match args.object_type.as_str(){
                "event" => read_all_events("http://localhost:3000/events".to_string()),
                _=> println!("default")
            }
        } else {
            match args.object_type.as_str(){
                "event" => read_all_events("http://localhost:3000/events".to_string()),
                _=> println!("default")
            }
        }

       // if args.object_type == 

        return;
    }


    if args.generate_key {
        let (private_key, public_key) = create_key_pair();
        let kp = KeyPair{private_key,public_key};
        let serialized_kp = serde_json::to_string(&kp).unwrap();
        fs::write(&args.key_file, serialized_kp).expect("Unable to write file");
        return;
    }
    if args.content == "" {
        println!("-c content missing");
        return;
    }
    // read keyfile
    let mut  data = "".to_string() ;
    let opt_data =  fs::read_to_string(&args.key_file);
    if !opt_data.is_ok(){
        println!("{}",opt_data.err().unwrap());
        return;
    } else {
        data = opt_data.unwrap();
    }


    //.expect("Unable to read file");
    let key_pair: KeyPair = serde_json::from_str(&data).unwrap();
    let mut  e = Event::new(key_pair.public_key, args.content);
    e.sign(key_pair.private_key);
   
    
    println!(" {:?} ",e);

    create_event(e,"http://localhost:3000/events".to_string());
}


// cli -c "dfs fs s fgdg dfg dg" -k 
// cli -g

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct KeyPair {
    public_key: String,
    private_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}


fn create_event(event: Event, url: String) {
    let client = reqwest::blocking::Client::new();
     let response_event = client.post(url)
    .header(CONTENT_TYPE, "application/json")
    .json(&event)
    .send()
    .expect("dsd");
    //.json::<Event>().expect("sdsd")
println!("{:#?}", response_event);
}



fn read_all_events(url: String) {
    let client = reqwest::blocking::Client::new();
     let response_event = client.get(url)
    .header(CONTENT_TYPE, "application/json")
    //.json(&event)
    .send()
    .expect("dsd")
    .json::<Vec<Event>>().expect("sdsd");
println!("{:#?}", response_event);
}

fn read_single_event(id: String,url: String) {
    let client = reqwest::blocking::Client::new();
    // TODO add id to url
     let response_event = client.get(url)
    .header(CONTENT_TYPE, "application/json")
    .send()
    .expect("dsd")
    .json::<Event>().expect("sdsd");
println!("{:#?}", response_event);
}