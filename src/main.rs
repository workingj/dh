// dh - Display Help

use std::env;
use std::fs::{DirEntry, File};
use std::io::prelude::BufRead;
use std::path::PathBuf;
use std::process::exit;

use std::io::{Result, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const HELP_FILE_TEXT: &str =
    r#"###--------------------------------DH-HELP-----------------------------------###
Create your helpfiles in the root directory of dh or,
set a location with the environment var 'DH_LIBRARY'
and put your helpfiles there. 
Use '.toml' extension and start header with '#' for highlighting. 

# EXAMPLE:
Filename      Command       Output
help.toml     $ dh help     this file!
yours.toml    $ dh yours    what ever you had jotted down."#;

/// Name for Environment Variable
const DH_LIBRARY: &str = "DH_LIBRARY";

// Function for coloring std output streams.
fn color_the_output_stream(stream: &mut StandardStream, color: Color) -> Result<()> {
    stream.set_color(ColorSpec::new().set_fg(Some(color)))
}

/// Print the content of the selected helpfile.
fn print_existing_helpfiles(entries: Vec<DirEntry>) -> Result<()> {
    for entry in entries {
        let mut filename = entry
            .file_name()
            .into_string()
            .expect("Error reading existing helpfiles!");
        if filename.contains("dh") {
            continue;
        }
        if filename.contains(".toml") {
            filename = filename[0..filename.len() - 5].to_string();
            println!("{}", filename);
        } else {
            println!("{}", filename);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Configure output streams for colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    // check for environment var and set path
    let mut current_path: PathBuf = PathBuf::new();
    let mut file_name: PathBuf = PathBuf::new();

    match env::var(DH_LIBRARY) {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() {
                current_path = path;
            } else {
                color_the_output_stream(&mut stderr, Color::Red)?;
                eprintln!(
                    "Error: Environment Variable '{}' not set correctly!",
                    DH_LIBRARY
                );
                std::mem::drop(current_path);
                exit(0);
            }
        }
        Err(var_err) => {
            match var_err {
                env::VarError::NotUnicode(_) => {
                    eprintln!("ENV VAR ERROR: Not Unicode -> {:?}", var_err)
                }
                _ => (), // ignore NotPresent case
            };
            current_path = PathBuf::from(env::current_exe()?.parent().unwrap());
        }
    }

    // Get filesystem entries.
    let mut entries: Vec<DirEntry> = vec![];
    for entry in std::fs::read_dir(&current_path)? {
        entries.push(entry.unwrap());
    }

    // Handel no file entry case:
    // Write HELP_FILE_TEXT to an newly created help file.
    if entries.len() == 0 || entries[0].file_name().into_string().unwrap() == "dh" {
        let mut help_file_path = current_path.clone();
        help_file_path.push("help.toml");
        let mut help_file = File::create(help_file_path)?;
        help_file.write(HELP_FILE_TEXT.as_bytes())?;
       
        // For creating the help file in an location
        file_name = PathBuf::from("help.toml");
    } else {
        // Sort filesystem entries name-ascending.
        entries.sort_by(|a, b| {
            a.file_name()
                .to_str()
                .unwrap()
                .cmp(&b.file_name().to_str().unwrap())
        });
    }

    // Get user args
    // exit if no filename is given and
    // if not in config mode 
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 &&  file_name.display().to_string() != "help.toml" {
        color_the_output_stream(&mut stderr, Color::Yellow)?;
        writeln!(&mut stderr, "No Arguments found!\nTry one of these:")?;
        color_the_output_stream(&mut stderr, Color::White)?;
        print_existing_helpfiles(entries)?;
        exit(1);
    }

    // Passing help.toml through for congif and help purpose.
    if file_name.display().to_string() != "help.toml" {
        file_name = PathBuf::from(&args[1]);
    }
    // Check for help output.
    {
        let name_test = file_name
            .to_str()
            .expect("Error reading existing helpfiles!")
            .as_bytes();
        if name_test == b"dh" || name_test == b"-h" || name_test == b"--help" {
            println!("{}", HELP_FILE_TEXT);
        }
    }

    // Check for the right file extension
    if file_name.extension() == None {
        file_name.set_extension("toml");
    }

    // Read the helpfile in to buffer.
    current_path.push(file_name);
    let file = match File::open(&current_path) {
        Ok(f) => f,
        Err(_) => {
            color_the_output_stream(&mut stdout, Color::Red)?;
            eprintln!("Error: Could not open file: {}", &current_path.display());
            exit(0);
        }
    };
    let file_lines = std::io::BufReader::new(file);
    let mut counter = 0_usize;
    let mut input = String::new();

    // Print the file contents to std stream.
    for line in file_lines.lines() {
        match line {
            Ok(line) => {
                if line.chars().nth(0) == Some('#') {
                    color_the_output_stream(&mut stdout, Color::Yellow)?;
                    writeln!(&mut stdout, "{}", line)?;
                } else {
                    color_the_output_stream(&mut stderr, Color::White)?;
                    println!("{}", line);
                }
                counter += 1;
                // Do a wait after 32 lines.
                if counter == 32 {
                    color_the_output_stream(&mut stdout, Color::Ansi256(8))?; // Gray
                    writeln!(&mut stdout, "{:─^77}", "▾▾▾")?;
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
