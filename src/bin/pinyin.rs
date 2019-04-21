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

    let model1 = pinyin::Model::<pinyin::Match1>::load();
    let model2 = pinyin::Model::<pinyin::Match2>::load();
    let model3 = pinyin::Model::<pinyin::Match3>::load();

    loop {
        let mut line = String::new();
        if input_file.read_line(&mut line).is_err() {
            break;
        }
        let words: Vec<&str> = line.trim().split(|c| c == ' ').collect();
        if words.is_empty() {
            continue;
        }
        if words.len() == 1 {
            let (result1, _) = model1.convert(&vec![words[0]], None);
            output_file.write(&result1.as_bytes())?;
            output_file.flush()?;
        } else if words.len() == 2 {
            let (_, prob1) = model1.convert(&vec![words[0]], None);
            let (result2, _) = model2.convert(&words, Some(prob1));
            output_file.write(&result2.as_bytes())?;
            output_file.flush()?;
        } else {
            let (_, prob1) = model1.convert(&vec![words[0]], None);
            let (_, prob2) = model2.convert(&vec![words[0], words[1]], Some(prob1));
            let (result3, _) = model3.convert(&words, Some(prob2));
            output_file.write(&result3.as_bytes())?;
            output_file.flush()?;
        }
    }

    Ok(())
}
