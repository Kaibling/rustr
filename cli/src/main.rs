use clap::Parser;
use crypto::create_key_pair;
use entity::Event;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use colored::*;
use chrono::prelude::DateTime;
use chrono::Utc;
use chrono::NaiveDateTime;
use std::time::{UNIX_EPOCH, Duration};

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
    events: bool,
    #[arg(short, long, default_value_t = false)]
    users: bool,
    #[arg(short, long, default_value_t = String::from(""))]
    id: String,
    #[arg(short='x', long, default_value_t = String::from("1970-01-01 00:00:00"))]
    expiration_date: String,
}

fn main() {
    let args = Args::parse();
    let api_url =  String::from("http://localhost:3000");
    if args.events {
        if args.id == "" {
        read_all_events(&format!("{}/{}",&api_url,"events"));
        } else {
            read_single_event(args.id, &format!("{}/{}",&api_url,"events"));
        }
        return;
    }



    if args.generate_key {
        let (private_key, public_key) = create_key_pair();
        let kp = KeyPair {
            private_key,
            public_key,
        };
        let serialized_kp = serde_json::to_string(&kp).unwrap();
        fs::write(&args.key_file, serialized_kp).expect("Unable to write file");
        return;
    }
    if args.content == "" {
        println!("-c content missing");
        return;
    }
    // read keyfile
    let data = match fs::read_to_string(&args.key_file) {
        Ok(val) => val,
        Err(err) => {println!("{}",err.to_string()); return;},
    };
    let key_pair: KeyPair = serde_json::from_str(&data).unwrap();
    let expiration_time = from_pretty_time(args.expiration_date);
    let mut e = Event::new(key_pair.public_key, args.content, expiration_time);
    e.sign(key_pair.private_key);
    create_event(e, &format!("{}/{}",&api_url,"events"));
}


#[derive(Serialize, Deserialize, Debug)]
struct KeyPair {
    public_key: String,
    private_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

fn create_event(event: Event, url: &String) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&event)
        .send()
        .expect("dsd");
    if response.status() != StatusCode::CREATED {
        println!("{} status code", response.status());
    }
    //println!("{:#?}", response_event);
}

fn read_all_events(url: &String) {
    let client = reqwest::blocking::Client::new();
    let response_event = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .expect("dsd")
        .json::<Vec<Event>>()
        .expect("sdsd");
    for e in response_event {
        print_event(e);
    }
}

fn read_single_event(id: String, url: &String) {
    let client = reqwest::blocking::Client::new();
    let full_url = format!("{}/{}", url, id);
    let response_event = client
        .get(full_url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .expect("dsd")
        .json::<Event>()
        .expect("sdsd");
    print_event(response_event);
}



fn print_event(event: Event) {
    let valid = if event.verify() {
        String::from("✓").green()
    } else {
        String::from("✗").red()
    };
    // show exporation_date
    println!("({})[{}] {}",valid,pretty_time(event.created_at),event.content);
}



fn pretty_time(time_stamp : u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(time_stamp);
    let datetime = DateTime::<Utc>::from(d);
    return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
}

fn from_pretty_time(str_time: String) -> u64 {
    let no_timezone = NaiveDateTime::parse_from_str(&str_time, "%Y-%m-%d %H:%M:%S").expect("s");
    return no_timezone.timestamp().try_into().unwrap();
}