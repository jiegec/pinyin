use pinyin;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pinyin")]
struct Opt {
    /// input file name, one sentence per line,
    /// use "-" for stdin
    #[structopt(name = "input", parse(from_os_str))]
    input: PathBuf,

    /// output file name,
    /// use "-" for stdout
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

    let model = pinyin::Model::<pinyin::Match2>::load();

    loop {
        let mut line = String::new();
        if input_file.read_line(&mut line).is_err() || line.is_empty() {
            break;
        }
        let result = model.convert(&line);
        output_file.write(&result.as_bytes())?;
    }

    Ok(())
}
