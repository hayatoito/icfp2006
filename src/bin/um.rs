use structopt;
#[macro_use]
extern crate structopt_derive;

use loggerv;


use icfp2006::um;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt]
struct Opt {
    #[structopt(short = "v")]
    verbose: u64,
    #[structopt(short = "p", long = "print-stdin")]
    print_stdin: bool,
    file: String,
}

fn main() {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();
    let mut um = um::Um::new(opt.file);
    if opt.print_stdin {
        um = um.print_stdin();
    }
    um.run();
}

/*

 sx, rx = stdin_channel;
 sx, rx = stdout_channel;


*/
