use icfp2006::adventure;
use icfp2006::prelude::Result;
use icfp2006::um::Um;
use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
enum Cmd {
    #[structopt(name = "um")]
    Um {
        #[structopt(short = "p", long = "print-stdin")]
        print_stdin: bool,
        file: String,
    },
    #[structopt(name = "qbasic")]
    QBasic { code: String },
    #[structopt(name = "adventure")]
    Adventure {
        #[structopt(subcommand)]
        cmd: Adventure,
    },
}

#[derive(StructOpt)]
enum Adventure {
    #[structopt(name = "solve1")]
    Solve1 { code: String },
    #[structopt(name = "solve2")]
    Solve2 { code: String },
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    loggerv::init_with_verbosity(args.verbose).unwrap();

    match args.cmd {
        Cmd::Um { print_stdin, file } => {
            let mut f = std::fs::File::open(&file)?;
            let mut code = Vec::new();
            f.read_to_end(&mut code)?;
            let mut um = Um::new(code);
            if print_stdin {
                um.set_print_stdin(true);
            }
            um.run(&mut std::io::stdin(), &mut std::io::stdout());
        }
        Cmd::QBasic { code } => icfp2006::qbasic::solve(code)?,
        Cmd::Adventure { cmd } => match cmd {
            Adventure::Solve1 { code } => adventure::part1::solve(code)?,
            Adventure::Solve2 { code } => adventure::part2::solve(code)?,
        },
    }
    Ok(())
}
