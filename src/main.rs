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
    let current_path = PathBuf::from(env::current_exe()?.parent().unwrap());

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(&mut stderr, "Error: no Arguments found!")?;
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        print_existing_helpfiles(&current_path)?;
        exit(1);
    }

    // let current_path = env::current_dir()?;
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
    let mut counter = 0_usize;

    let mut input = String::new();

    for line in file_lines.lines() {
        match line {
            Ok(line) => {
                if line.contains('#') {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                    writeln!(&mut stdout, "{}", line)?;
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                    println!("{}", line);
                }
                counter += 1;
                if counter == 24 {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                    writeln!(&mut stdout, "  -----------------------------------------------------------------------------\\/")?;
                    std::io::stdin().read_line(&mut input)?;
                    counter = 0;
                }
            }
            Err(err) => {
                writeln!(&mut stderr, "{:?}", err)?;
            }
        }
    }

    Ok(())
}

fn print_existing_helpfiles(path: &PathBuf) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(path)? {
        // let filename = ;
        println!(
            "{}",
            entry?
                .file_name()
                .into_string()
                .expect("Error reading existing helpfiles!: ")
        );
    }

    Ok(())
}
