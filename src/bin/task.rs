extern crate regex;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate icfp2006;
extern crate loggerv;
extern crate log;

// use icfp2006::um;

use regex::Regex;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt]
struct Opt {
    #[structopt(short = "v")]
    verbose: u64,
    file: String,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(name = "task02")]
    Task02 {
        #[structopt(subcommand)]
        cmd: Task02,
    },
}

#[derive(StructOpt, Debug)]
enum Task02 {
    #[structopt(name = "extract-password")]
    ExtractPassword,
}

fn main() {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();

    match opt.cmd {
        Cmd::Task02 { cmd } => {
            match cmd {
                Task02::ExtractPassword => {
                    let _input = "guest";

                    // let mut _um = um::UM::new(opt.file);

                    // TODO: Update the followings. [2017-10-23 Mon]

                    // Sample line
                    // CXXV     words(I) = "airplane"

                    let re = Regex::new(r#"^.*words\(\w+\) = "(\w+)"$"#).unwrap();

                    let stdin = std::io::stdin();
                    for line in stdin.lock().lines() {
                        if let Some(caps) = re.captures(&line.unwrap()) {
                            println!("howie");
                            println!("{}", caps.at(1).unwrap());
                        }

                    }

                }
            }
        }
    }
}
