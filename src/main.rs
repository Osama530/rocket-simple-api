#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate rustc_serialize;


use rustc_serialize::json::Json;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;



#[get("/")]
fn hello()->String {
    let path = Path::new("api.json");
    let display = path.display();

    println!("{:?} {:?}", path, display);

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => panic!("cloud not crate file"),
    };
    match reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=Lahore&Apikey=4970e4f266675063af77ad454f45ebd6&units=metric") {
        Ok(mut response) => {
            match response.text() {
                Ok(text) => match file.write_all(text.as_bytes()){
                    Ok(_) => println!("Data write to File api.json"),
                    Err(e)=> println!("The error is this: {}",e),
                },
                Err(_) => println!("No response from server"),

            }
        },
        Err(_) => println!("server could not establish the connection"),

    }
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("error opening file"),
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let json = Json::from_str(&buffer).unwrap();

    let result = format!("the temprtatur of lahore is {}", json.find_path(&["main"]).unwrap());
    result
}

fn main(){
    rocket::ignite().mount("/", routes![hello]).launch();
}