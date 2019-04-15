#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;
use std::io::{Result, BufReader, stdin, stdout, BufRead, Write};
use std::fs::File;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "input", parse(from_os_str))]
    input: PathBuf,

    #[structopt(name = "output", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let stdin = stdin();
    let stdout = stdout();
    let mut input_file: Box<dyn BufRead> = if opt.input != PathBuf::from("-") {
        Box::new(BufReader::new(File::open(&opt.input)?))
    } else {
        Box::new(stdin.lock())
    };
    let mut output_file: Box<dyn Write> = if opt.output != PathBuf::from("-") {
        Box::new(File::create(&opt.output)?)
    } else {
        Box::new(stdout.lock())
    };

    loop {
        let mut line = String::new();
        if input_file.read_line(&mut line).is_err() {
            break;
        }
        output_file.write(&line.as_bytes())?;
    }

    println!("opt: {:?}", opt);

    Ok(())
}