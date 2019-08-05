// dh - Display Help

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
const CONFIG_FILE: &'static str = "help.toml";
#[allow(dead_code)]
const HELP_FILE_TEXT: &'static str = "fromGetHelpEntries__ Command  Description\n\
                                      help    this file!\n\
                                      test    Test\n";

fn main() -> Result<(), std::io::Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(&mut stderr, "Error: no Arguments found!")?;
        exit(1);
    }

    // let current_path = env::current_dir()?;
    let current_path = PathBuf::from(env::current_exe()?.parent().unwrap());
    let mut file_path = current_path.clone();
    let mut file_name = PathBuf::from(&args[1]);

    if file_name.extension() == None {
        file_name.set_extension("toml");
    } else if file_name.extension() != Some(&std::ffi::OsString::from("toml").as_os_str()) {
        file_name.set_extension("toml");
    }
    file_path.push(file_name);

    let file = File::open(file_path)?;
    let file_lines = std::io::BufReader::new(file);

    for line in file_lines.lines() {
        match line {
            Ok(line) => {
                if line.contains("#") {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                    writeln!(&mut stdout, "{}", line)?;
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                    println!("{}", line);
                }
            }
            Err(err) => {
                writeln!(&mut stderr, "{:?}", err)?;
            }
        }
    }

    Ok(())
}
