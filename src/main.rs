extern crate clap;
extern crate indicatif;

use indicatif::ProgressBar;
use clap::{App, Arg};

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

fn create_progress_bar(quiet_mode:bool,msg:&str,length:Option<u64>)->ProgressBar{
    let bar=match quiet_mode{
        true=>ProgressBar::hidden(),
        false=>match length{
            Some(len)=>ProgresssBar::new(len),
            None=>ProgressBar::new_spinner,
        }
    };

}
