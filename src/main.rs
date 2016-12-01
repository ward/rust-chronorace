extern crate clap;
extern crate reqwest;

mod athlete;
// use athlete::*;
mod chronorace;
// use chronorace::*;

use std::io::Read;

fn main() {
    let matches = clap::App::new("rust-chronorace")
        .version("0.1.0")
        .author("Ward Muylaert")
        .about("Take chronorace results URL and turns it into CSV")
        .arg(clap::Arg::with_name("URL")
            .required(true)
            .help("The chronorace URL to parse"))
        .get_matches();

    // Panic if no URL supplied
    let url = matches.value_of("URL").unwrap();
    //println!("You provided the following URL: {}", url);

    let mut response = reqwest::get(url).unwrap();
    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();

    let mut athletes = chronorace::parse_athletes(&content);

    let urls = chronorace::parse_page_urls(&content);

    for url in urls {
        let mut response = reqwest::get(&url).unwrap();
        let mut content = String::new();
        response.read_to_string(&mut content).unwrap();
        let mut moreathletes = chronorace::parse_athletes(&content);
        athletes.append(&mut moreathletes);
    }

    for athlete in athletes {
        println!("{}", athlete.to_csv());
    }
}
