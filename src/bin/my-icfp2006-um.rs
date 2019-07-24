use loggerv;
use structopt;

use std::io::Read;

use icfp2006::um;
// use icfp2006::um::Result;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(short = "p", long = "print-stdin")]
    print_stdin: bool,
    file: String,
}

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();

    let mut f = std::fs::File::open(&opt.file)?;
    let mut code = Vec::new();
    f.read_to_end(&mut code)?;
    let mut um = um::Um::new(code);
    if opt.print_stdin {
        um.set_print_stdin(true);
    }
    um.run(&mut std::io::stdin(), &mut std::io::stdout());
    Ok(())
}
