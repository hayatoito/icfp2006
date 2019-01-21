// Suppress clippy

// extern crate log;

use std;
use std::fs::File;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use std::time::Instant;

type Array = Vec<u32>;

pub struct Um<R, W> {
    reg: [u32; 8],
    arrays: Vec<Array>,
    pc: usize,
    arrayid: u32,
    program_path: PathBuf,
    program: Option<Array>,
    stdin: R,
    stdout: W,
    print_stdin: bool,
}

// http://www.boundvariable.org/um-spec.txt

enum Step {
    Continue,
    NoInput,
    Halt,
}

impl Um<std::io::Stdin, std::io::Stdout> {
    pub fn new<P>(program: P) -> Self
    where
        P: AsRef<Path>,
    {
        Um {
            reg: [0; 8],
            arrays: Vec::new(),
            pc: 0,
            arrayid: 0,
            program_path: program.as_ref().into(),
            program: None,
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            print_stdin: false,
        }
    }
}

impl<R, W> Um<R, W>
where
    R: Read,
    W: Write,
{
    // pub fn stdin<R2>(self, r: R2) -> Um<R2, W>
    // where
    //     R2: Read,
    // {
    //     Um {
    //         reg: self.reg,
    //         arrays: self.arrays,
    //         pc: self.pc,
    //         arrayid: self.arrayid,
    //         program_path: self.program_path,
    //         program: self.program,
    //         stdin: r,
    //         stdout: self.stdout,
    //         print_stdin: self.print_stdin,
    //     }
    // }

    // pub fn stdout<W2>(self, w: W2) -> Um<R, W2>
    // where
    //     W2: Write,
    // {
    //     Um {
    //         reg: self.reg,
    //         arrays: self.arrays,
    //         pc: self.pc,
    //         arrayid: self.arrayid,
    //         program_path: self.program_path,
    //         program: self.program,
    //         stdin: self.stdin,
    //         stdout: w,
    //         print_stdin: self.print_stdin,
    //     }
    // }

    pub fn set_stdin(&mut self, stdin: R) {
        self.stdin = stdin;
    }

    pub fn set_stdout(&mut self, stdout: W) {
        self.stdout = stdout;
    }

    pub fn print_stdin(self) -> Um<R, W> {
        Um {
            print_stdin: true,
            ..self
        }
    }

    fn load_program(&mut self) -> Array {
        let now = Instant::now();
        info!("loading...");

        let mut f = File::open(&self.program_path).unwrap();
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).expect("read failed");
        debug_assert!(bytes.len() % 4 == 0);

        info!("bytes: len: {}: {:?}", bytes.len(), now.elapsed());
        // self.arrays.push(Vec::with_capacity(bytes.len() / 4));
        let mut array = Vec::with_capacity(bytes.len() / 4);
        for b4 in bytes.chunks(4) {
            let mut a: u32 = 0;
            for (i, &b) in b4.iter().enumerate() {
                a |= (b as u32) << (24 - i * 8);
            }
            array.push(a);
        }
        info!("loaded: {:?}", now.elapsed());
        array
    }

    fn reset_memory(&mut self) {
        self.reg = Default::default();
        self.arrays = Default::default();
        self.pc = Default::default();
        self.arrayid = Default::default();
    }

    pub fn run(&mut self) {
        match self.program {
            Some(_) => {
                self.reset_memory();
            }
            None => {
                self.program = Some(self.load_program());
            }
        }
        self.arrays.push(self.program.as_ref().unwrap().clone());
        debug_assert!(self.arrays.len() == 1);
        while let Step::Continue = self.step() {}
    }

    pub fn continue_run(&mut self) {
        while let Step::Continue = self.step() {}
    }

    fn step(&mut self) -> Step {
        let p = self.arrays[0][self.pc];
        let op = p >> 28;
        let a = ((p >> 6) & 0b111) as usize;
        let b = ((p >> 3) & 0b111) as usize;
        let c = (p & 0b111) as usize;
        debug_assert!(a < 8);
        debug_assert!(b < 8);
        debug_assert!(c < 8);

        trace!("p: {:032b}", p);
        trace!("reg: {:?}", self.reg);
        trace!("a: {}, b: {}, c: {}, pc: {}, op: {}", a, b, c, self.pc, op);
        trace!("");

        self.pc += 1;
        match op {
            0 => {
                if self.reg[c] != 0 {
                    self.reg[a] = self.reg[b];
                }
            }
            1 => self.reg[a] = self.arrays[self.reg[b] as usize][self.reg[c] as usize],
            2 => self.arrays[self.reg[a] as usize][self.reg[b] as usize] = self.reg[c],
            3 => self.reg[a] = self.reg[b].wrapping_add(self.reg[c]),
            4 => self.reg[a] = self.reg[b].wrapping_mul(self.reg[c]),
            5 => self.reg[a] = self.reg[b] / self.reg[c],
            6 => self.reg[a] = !(self.reg[b] & self.reg[c]),
            7 => {
                return Step::Halt;
            }
            8 => {
                self.arrays.push(vec![0; self.reg[c] as usize]);
                self.arrayid += 1;
                self.reg[b] = self.arrayid;
            }
            9 => {
                // println!("Abandonment: {}", self.reg[c]);
                self.arrays[self.reg[c] as usize] = vec![]
            }
            10 => {
                debug_assert!(self.reg[c] < 256);
                let buf = [self.reg[c] as u8];
                // print!("{}", unsafe { std::char::from_u32_unchecked(self.reg[c]) });
                self.stdout.write_all(&buf).expect("Write failed");
                self.stdout.flush().unwrap();
            }
            11 => {
                let mut b = [0; 1];
                match self.stdin.read(&mut b) {
                    Ok(1) => {
                        // debug!("read: {:?}", b[0]);
                        debug_assert!((b[0] as u32) < 256);
                        self.reg[c] = b[0] as u32;
                        if self.print_stdin {
                            self.stdout.write_all(&b).expect("Write failed");
                        }
                        self.stdout.flush().unwrap();
                    }
                    Ok(_) => {
                        warn!("stdin EOF");
                        return Step::NoInput;
                    }
                    Err(_) => {
                        error!("Can't read from stdin");
                        return Step::NoInput;
                    }
                }
            }
            12 => {
                if self.reg[b] != 0 {
                    self.arrays[0] = self.arrays[self.reg[b] as usize].clone();
                }
                self.pc = self.reg[c] as usize;
            }
            13 => {
                let a = ((p >> 25) & ((1 << 3) - 1)) as usize;
                let v = p & ((1 << 25) - 1);
                self.reg[a] = v;
            }
            _ => println!("Ops!"),
        }
        Step::Continue
    }
}

struct InteractiveUm<R, W> {
    um: Um<R, W>,
}

impl<R, W> InteractiveUm<R, W>
where
    R: Read,
    W: Write + std::default::Default + Clone,
{
    pub fn repl(&mut self, line: R) -> Option<&W> {
        self.um.set_stdin(line);
        let stdout: W = std::default::Default::default();
        self.um.set_stdout(stdout);
        self.um.continue_run();
        Some(&self.um.stdout)
    }
}
