use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use num::FromPrimitive;
use num::bigint::{BigUint, ToBigUint};
use num::rational::Ratio;

/// Yet another FRACTRAN rust interpreter
///
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file with fractran code
    #[arg(short, long, value_name = "file")]
    code: PathBuf,

    /// start value
    #[arg(short, long, value_name = "number")]
    start: u64,

    /// list register very slow !!!
    #[arg(short, long, )]
    listregs: bool,

    /// debug
    #[arg(short, long)]
    debug: bool,
}

struct FractranVM {
    prog_mem: Vec<Ratio<BigUint>>,
    initvalue: Ratio<BigUint>,
    prog_size: usize,
    debug: bool,
    listreg: bool,
}

impl FractranVM {
    fn load(srcfile: PathBuf, start: u64, debug: bool, listreg: bool) -> Self {
        let start: Ratio<BigUint> = Ratio::from_integer(FromPrimitive::from_u64(start).unwrap());
        let mem = FractranVM::parse(srcfile);
        let size = mem.len();
        FractranVM {
            prog_mem: mem,
            initvalue: start,
            prog_size: size,
            debug: debug,
            listreg: listreg,
        }
    }

    fn parse(source_file: PathBuf) -> Vec<Ratio<BigUint>> {
        let mut mem: Vec<Ratio<BigUint>> = Vec::new();
        let f = File::open(source_file).expect("Unable to open file");
        let f = BufReader::new(f);
        for l in f.lines() {
            let line = l.expect("Unable to read line");
            // ignore comment lines starting with ;
            if !line.starts_with(";") {
                for s in line.split_whitespace() {
                    mem.push(Ratio::<BigUint>::from_str(s).unwrap());
                }
            }
        }
        mem
    }

    fn run(&self) -> Ratio<BigUint> {
        let mut ip: usize = 0;
        let mut accu = self.initvalue.clone();
        let mut value: Ratio<BigUint>;
        while ip < self.prog_size {
            value = &accu * &self.prog_mem[ip];
            if value.is_integer() {
                if self.debug {
                    print!("Debug {} ", value.numer());
                    self.show_register(value.numer().clone());
                }
                accu = value;
                ip = 0;
            } else {
                ip += 1;
            }
        }
        accu
    }

    fn show_register(&self, mut num: BigUint) -> () {
        if self.listreg {
            let mut i: u32;
            let max = num.sqrt();
            let num1 = &mut num;
            print!("::: Register({}):", num1);
            i = self.check_prime_factor(&2.to_biguint().unwrap(), num1);
            if i > 0 {
                print!("(R{}^:{})", 2, i);
            }
            let p = &mut 3.to_biguint().unwrap();
            while *p <= max {
                i = self.check_prime_factor(p, num1);
                if i > 0 {
                    print!("(R{}^:{})", *p, i);
                }
                *p += 1_u32;
            }
        }
        println!("");
    }

    fn check_prime_factor(&self, f: &BigUint, num: &mut BigUint) -> u32 {
        let mut i = 0_u32;
        loop {
            if self.has_remainder(f, num) {
                break;
            }
            i += 1;
            *num /= f;
        }
        i
    }

    fn has_remainder(&self, n: &BigUint, num: &BigUint) -> bool {
        let one = 1.to_biguint().unwrap();
        if num % n == one {
            return true;
        }
        false
    }
}

fn main() {
    let cli = Cli::parse();
    let vm = FractranVM::load(cli.code, cli.start, cli.debug, cli.listregs);
    print!("Start value {} ", vm.initvalue.numer());
    vm.show_register(vm.initvalue.numer().clone());
    let result = vm.run();
    print!("Result: {} ", result.numer());
    vm.show_register(result.numer().clone());
}
