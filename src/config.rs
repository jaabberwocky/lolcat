use crate::meow_file::MeowFile;
use clap::Parser;
use std::{io, io::prelude::*};

const ASCII_CAT: &str = r" /\_/\  
( o.o ) 
 > ^ < ";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = ASCII_CAT.to_string() + "cat but with meows! Concatenate file(s) to meows, or read from standard input to meows if no files are specified.\n\nAuthor: tobias",
    long_about
)]
pub struct Args {
    // path to filename
    #[arg(short, long, help = "Path to file(s)", default_value_t = String::from(""))]
    files: String,

    #[arg(short, long, help = "Number lines", default_value_t = false)]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "nonblank",
        help = "Number nonblank lines",
        default_value_t = false
    )]
    number_nonblank_lines: bool,
}

#[derive(Debug)]
pub struct Config {
    pub files: Vec<MeowFile>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum Mode {
    FileMode,
    StdinMode,
    NumberLines,
    NumberNonblankLines,
}

impl Config {
    pub fn new(args: Args) -> Result<Self, String> {
        let mut filepaths: Vec<MeowFile> = Vec::new();
        for file in args.files.split(' ') {
            if file.is_empty() {
                continue;
            }
            filepaths.push(MeowFile::new(file.to_string()));
        }

        let mode = match (
            filepaths.len(),
            args.number_lines,
            args.number_nonblank_lines,
        ) {
            (0, _, _) => Mode::StdinMode,
            (_, false, false) => Mode::FileMode,
            (_, true, _) => Mode::NumberLines,
            (_, false, true) => Mode::NumberNonblankLines,
        };

        match mode {
            Mode::StdinMode => {}
            _ => {
                Self::check_file_exists(filepaths.clone())?;
            }
        }

        Ok(Self {
            files: filepaths,
            number_lines: args.number_lines,
            number_nonblank_lines: args.number_nonblank_lines,
            mode,
        })
    }

    fn check_file_exists(filepaths: Vec<MeowFile>) -> Result<(), String> {
        for file in filepaths {
            if !std::path::Path::new(&file.filepath).exists() {
                return Err(format!("File: {} does not exist", file));
            }
        }
        Ok(())
    }

    pub fn read_all_files(&self) -> Result<(), String> {
        let mut line_number = 1;
        match &self.mode {
            Mode::StdinMode => Self::read_from_stdin(&mut line_number)?,
            _ => {
                for file in &self.files {
                    file.read_file(&self.mode, &mut line_number)?;
                }
            }
        }
        Ok(())
    }

    fn read_from_stdin(line_number: &mut i32) -> Result<(), String> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            MeowFile::process_line(line.unwrap(), line_number, &Mode::StdinMode);
        }
        Ok(())
    }
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
