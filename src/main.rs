// Entry for mofishcoin

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("mofishcoin deamon")
        .version("1.0")
        .author("mofish team")
        .about("mofishcoin is a fork of bitcoin with mofish features")
        .arg(
            Arg::with_name("enable-api")
                .help("Whether to enable api for mofishcoin network frontend")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let api_enabled = matches.is_present("enable-api");
    // more program logic goes here...

    println!("value of enable-api: {}", api_enabled);
}
