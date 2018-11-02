extern crate clap;
extern crate indicatif;
extern crate reqwest;


use reqwest::*;
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Vitaly Ankh <vitalyankh@gmail.com>")
        .about("wget clone written in rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to downnload"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        }
    };
    bar.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}]\
                [{wide_bar}:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };
    bar
}


fn download(target:&str,quiet_mode:bool)->Result<(),Box<std::error::Error>>{
    // parse url
    let url=parse_url(target)?;
    let client=Client::new().unwrap();
    let mut resp=client.get_url()?
        .send()
        .unwrap();
//    print(format!("HTTP request sent... {}",
//    style(format!("{}",resp.status())).green()),
//    quiet_mode);
    if resp.status().is_success(){
        let header=resp.header().clone();
    }

    Ok(())
}
