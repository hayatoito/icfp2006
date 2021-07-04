// use std::fs::File;

// use crate::prelude::Result;

use std::io::{Read, Write};
use std::time::Instant;

use log::*;

#[derive(Debug, PartialEq, Eq)]
pub enum UmStatus {
    NoInput,
    Halt,
}

type Array = Vec<u32>;

#[derive(Clone)]
pub struct Um {
    code: Vec<u8>,
    reg: [u32; 8],
    arrays: Vec<Array>,
    pc: usize,
    array_id: u32,
    program: Option<Array>,
    print_stdin: bool,
}

// http://www.boundvariable.org/um-spec.txt

enum Step {
    Continue,
    NoInput,
    Halt,
}

impl Um {
    pub fn new(code: Vec<u8>) -> Self {
        Um {
            code,
            reg: [0; 8],
            arrays: Vec::new(),
            pc: 0,
            array_id: 0,
            program: None,
            print_stdin: false,
        }
    }

    pub fn set_print_stdin(&mut self, print_stdin: bool) {
        self.print_stdin = print_stdin;
    }

    fn load_program(&mut self) -> Array {
        let now = Instant::now();
        info!("loading...");

        // let mut f = File::open(&self.program_path).unwrap();
        // let mut bytes = Vec::new();
        // f.read_to_end(&mut bytes).expect("read failed");
        debug_assert!(self.code.len() % 4 == 0);

        info!("code: len: {}", self.code.len());
        let mut array = Vec::with_capacity(self.code.len() / 4);
        for b4 in self.code.chunks(4) {
            let mut a: u32 = 0;
            for (i, &b) in b4.iter().enumerate() {
                a |= u32::from(b) << (24 - i * 8);
            }
            array.push(a);
        }
        info!("loaded: {:?}", now.elapsed());
        array
    }

    fn reset(&mut self) {
        self.reg = Default::default();
        self.arrays = Default::default();
        self.pc = Default::default();
        self.array_id = Default::default();
    }

    pub fn run(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) -> UmStatus {
        match self.program {
            Some(_) => {
                self.reset();
            }
            None => {
                self.program = Some(self.load_program());
            }
        }
        self.arrays.push(self.program.as_ref().unwrap().clone());
        debug_assert!(self.arrays.len() == 1);
        self.run_loop(stdin, stdout)
    }

    pub fn continue_with(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) -> UmStatus {
        self.pc -= 1;
        self.run_loop(stdin, stdout)
    }

    fn run_loop(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) -> UmStatus {
        loop {
            match self.step(stdin, stdout) {
                Step::Continue => (),
                // Step::NoInput => return Err(UmErrorKind::NoInputError)?,
                Step::NoInput => return UmStatus::NoInput,
                Step::Halt => return UmStatus::Halt,
            }
        }
    }

    fn step(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) -> Step {
        let inst = self.arrays[0][self.pc];
        let op = inst >> 28;
        let a = ((inst >> 6) & 0b111) as usize;
        let b = ((inst >> 3) & 0b111) as usize;
        let c = (inst & 0b111) as usize;
        debug_assert!(a < 8);
        debug_assert!(b < 8);
        debug_assert!(c < 8);

        trace!("inst: {:032b}", inst);
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
                self.array_id += 1;
                self.reg[b] = self.array_id;
            }
            9 => {
                // println!("Abandonment: {}", self.reg[c]);
                self.arrays[self.reg[c] as usize] = vec![]
            }
            10 => {
                debug_assert!(self.reg[c] < 256);
                let buf = [self.reg[c] as u8];
                // print!("{}", unsafe { std::char::from_u32_unchecked(self.reg[c]) });
                stdout.write_all(&buf).expect("Write failed");
                stdout.flush().unwrap();
            }
            11 => {
                let mut b = [0; 1];
                match stdin.read(&mut b) {
                    Ok(1) => {
                        // debug!("read: {:?}", b[0]);
                        debug_assert!(u32::from(b[0]) < 256);
                        self.reg[c] = u32::from(b[0]);
                        if self.print_stdin {
                            stdout.write_all(&b).expect("Write failed");
                        }
                        stdout.flush().unwrap();
                    }
                    Ok(_) => {
                        // warn!("stdin EOF");
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
                let a = ((inst >> 25) & ((1 << 3) - 1)) as usize;
                let v = inst & ((1 << 25) - 1);
                self.reg[a] = v;
            }
            _ => println!("Ops!"),
        }
        Step::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Result;
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn um_sandmark_benchmark() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut f = std::fs::File::open(dir.join("task/01_implement_um/sandmark.umz")).unwrap();
        // let mut f = std::fs::File::open(dir.join("icfp2006/task/task_00/sandmark.umz")).unwrap();
        let mut code = Vec::new();
        f.read_to_end(&mut code).unwrap();
        let reader: [u8; 0] = [];
        let mut writer: Vec<u8> = vec![];
        let mut um = Um::new(code);
        assert_eq!(um.run(&mut reader.as_ref(), &mut writer), UmStatus::Halt);
        assert_eq!(writer.len(), 2946);
        assert_eq!(
            std::str::from_utf8(&writer).unwrap(),
            "trying to Allocate array of size 0..
trying to Abandon size 0 allocation..
trying to Allocate size 11..
trying Array Index on allocated array..
trying Amendment of allocated array..
checking Amendment of allocated array..
trying Alloc(a,a) and amending it..
comparing multiple allocations..
pointer arithmetic..
check old allocation..
simple tests ok!
about to load program from some allocated array..
success.
verifying that the array and its copy are the same...
success.
testing aliasing..
success.
free after loadprog..
success.
loadprog ok.
 == SANDmark 19106 beginning stress test / benchmark.. ==
100. 12345678.09abcdef
99.  6d58165c.2948d58d
98.  0f63b9ed.1d9c4076
97.  8dba0fc0.64af8685
96.  583e02ae.490775c0
95.  0353a77b.2f02685c
94.  aa25a8d7.51cb07e5
93.  e13149f5.53a9ae5d
92.  abbbd460.86cf279c
91.  2c25e8d8.a71883a9
90.  dccf7b71.475e0715
89.  49b398a7.f293a13d
88.  9116f443.2d29be37
87.  5c79ba31.71e7e592
86.  19537c73.0797380a
85.  f46a7339.fe37b85a
84.  99c71532.729e2864
83.  f3455289.b84ced3d
82.  c90c81a9.b66fcd61
81.  087e9eef.fc1c13a6
80.  e933e2f5.3567082f
79.  25af849e.16290d7b
78.  57af9504.c76e7ded
77.  68cf6c69.6055d00c
76.  8e920fbd.02369722
75.  eb06e2de.03c46fda
74.  f9c40240.f1290b2a
73.  7f484f97.bc15610b
72.  1dabb00e.61e7b75b
71.  dceb40f5.207a75ca
70.  c3ed44f5.db631e81
69.  b7addb67.90460bf5
68.  ae710a90.04b433ef
67.  9ca2d5f0.05d3b631
66.  4f38abe0.4287cc05
65.  10d8691d.a5c934f8
64.  27c68255.52881eaa
63.  a0695283.110266b7
62.  336aa5dd.57287a9b
61.  b04fe494.d741ddbd
60.  2baf3654.9e33305a
59.  fd82095d.683efb19
58.  d0bac37f.badff9d7
57.  3be33fcc.d76b127e
56.  7f964f18.8b118ee1
55.  37aeddc8.26a8f840
54.  d71d55ff.6994c78f
53.  bf175396.f960cc54
52.  f6c9d8e1.44b81fd5
51.  6a9b4d86.fe7c66cb
50.  06bceb64.d5106aad
49.  237183b6.49c15b01
48.  4ec10756.6936136f
47.  9d1855a7.1e929fe8
46.  a641ede3.36bff422
45.  7bbf5ad4.dd129538
44.  732b385e.39fadce7
43.  b7f50285.e7f54c39
42.  42e3754c.da741dc1
41.  5dc42265.928ea0bb
40.  623fb352.3f25bc5b
39.  491f33d9.409bca87
38.  f0943bc7.89f512be
37.  80cdbc9d.8ad93517
36.  c1a8da99.32d37f3f
35.  91a0b15c.6df2cf4e
34.  50cf7a7a.f0466dc8
33.  02df4c13.14eb615d
32.  2963bf25.d9f06dfe
31.  c493d2db.f39ce804
30.  3b6e5a8e.5cf63bd7
29.  4c5c2fbe.8d881c00
28.  9b7354a6.81181438
27.  ae0fe8c6.ec436274
26.  e786b98d.f5a4111d
25.  a7719df1.d989d0b6
24.  beb9ebc0.6c56750d
23.  edf41fcb.e4cba003
22.  97268c46.713025f1
21.  deb087db.1349eb6a
20.  fc5221f0.3b4241bf
19.  3fa4370d.8fa16752
18.  044af7de.87b44b11
17.  2e86e437.c4cdbc54
16.  fd7cd8aa.63b6ca23
15.  631ceaad.e093a9d5
14.  01ca9732.52962532
13.  86d8bcf5.45bdf474
12.  8d07855b.0224e80f
11.  0f9d2bee.94d86c38
10.  5e6a685d.26597494
9.   24825ea1.72008775
8.   73f9c0b5.1480e7a3
7.   a30735ec.a49b5dad
6.   a7b6666b.509e5338
5.   d0e8236e.8b0e9826
4.   4d20f3ac.a25d05a8
3.   7c7394b2.476c1ee5
2.   f3a52453.19cc755d
1.   2c80b43d.5646302f
0.   a8d1619e.5540e6cf
SANDmark complete.
"
        );
        Ok(())
    }
}
