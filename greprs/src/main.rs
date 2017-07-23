//use std::env;
//use std::fs::File;
//use std::io::prelude::*;
//use std::process;
//
//use std::error::Error;
//
extern crate greprs;
use std::env;
use std::process;

use greprs::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


//fn main() {
//    let args: Vec<String> = env::args().collect();
////    println!("{:?}", args);
////    let query = &args[1];
////    let filename = &args[2];
////
////    let (query, filename) = parse_config(&args);
////    println!("Searching for {}", query);
////    println!("In file {}", filename);
////    let mut f = File::open(filename).expect("file not found");
////    let config = parse_config(&args);
////    let config = Config::new(&args);
//    let config = Config::new(&args).unwrap_or_else(|err| {
//        println!("Problem parsing arguments: {}", err);
//        process::exit(1);
//    });
//
//    println!("Searching for {}", config.query);
//    println!("In file {}", config.filename);
////    let mut f = File::open(config.filename).expect("file not found");
////
////    let mut contents = String::new();
////    f.read_to_string(&mut contents).expect("something went wrong reading the file");
////    //fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
////
////    println!("With text:\n{}", contents);
////    run(config);
//   if let Err(e) = run(config) {
//        println!("Application error: {}", e);
//
//        process::exit(1);
//    }
//
//}
//
//struct Config {
//    query: String,
//    filename: String,
//}
//
////fn parse_config(args: &[String]) -> (&str, &str) {
////    let query = &args[1];
////    let filename = &args[2];
////
////    //(query, filename)//tuple
////}
//
////fn parse_config(args: &[String]) -> Config {
////    let query = args[1].clone();
////    let filename = args[2].clone();
////
////    Config { query, filename }
////}
////impl Config {
////    fn new(args: &[String]) -> Config {
////        if args.len() < 3 {
////            panic!("not enough arguments");
////        }
////        let query = args[1].clone();
////        let filename = args[2].clone();
////
////        Config { query, filename }
////    }
////}
//impl Config {
//    fn new(args: &[String]) -> Result<Config, &'static str> {
//        if args.len() < 3 {
//            return Err("not enough arguments");
//        }
//
//        let query = args[1].clone();
//        let filename = args[2].clone();
//
//        Ok(Config { query, filename })
//    }
//}
////fn run(config: Config) {
////    let mut f = File::open(config.filename).expect("file not found");
////
////    let mut contents = String::new();
////    f.read_to_string(&mut contents).expect("something went wrong reading the file");
////
////    println!("With text:\n{}", contents);
////}
//
//fn run(config: Config) -> Result<(), Box<Error>> {
//    let mut f = File::open(config.filename)?;//?
//
//    let mut contents = String::new();
//    f.read_to_string(&mut contents)?;
//
//    println!("With text:\n{}", contents);
//
//    Ok(())
//}
