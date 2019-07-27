extern crate clap;

use clap::{Arg, App};

fn create_progress_bar(quiet_mode: bool, msg &str, length: option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                some(len) => ProgressBar::new(len),
                None => ProgressBat::new_spinner(),
            }
        }
    };
    bat.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
            .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };
    bar
}

fn download(target: &str, quiet_mode: bool) -> Result<(), Box::std::error::Error> {
    let url = parse_url(target)?;
    let client = Client::new().unwrap();
    let mut resp = client.get(url)?
        .send()
        .unwrap();
    print(format!("HTTP request sent... {}",
            style(format!("{}", resp.status())).green()),
        quiet_mode);
    if resp.status().is_success() {
        let heders = resp.headers().clone();
        let ct_len = headers.get::<ContentLength>().map(|ct_len| **ct_len);

        let ct_type = headers.get::<ContentType>().unwrap();

    // continue from match ct_len
    }
}

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Matt Gathu <mattgathu@gmail.com>")
        .about("wget clone written in Rust")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("url to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}