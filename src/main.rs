// dh - Display Help

use std::env;
use std::fs::{DirEntry, File};
use std::io::prelude::BufRead;
use std::path::PathBuf;
use std::process::exit;

use std::io::{Result, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const HELP_FILE_TEXT: &str = r#"###--------------------------------DH-HELP-----------------------------------###
# INSTRUCTION
Create your helpfiles in the root directory of dh or,
set a location with the environment var 'DH_LIBRARY'
and put your helpfiles there.
## Files
Use '.toml' for File extension.

# USAGE EXAMPLE:
Filename      Command       Output
help.toml     $ dh help     this file!
yours.toml    $ dh yours    what ever you have jotted down.
              $ dh          Lists all Helpfiles within the 'DH_LIBRARY'-Location

# HIGHLIGHTING:
## Lines that start with '#','##','###...' will be highlighted
'#'   Yellow
'##'  Blue
'###' Orange

######--------------------------------------------------------------------######
"#;

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
            println!("    {}", filename);
        } else {
            println!("    {}", filename);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Configure output streams for colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    color_the_output_stream(&mut stderr, Color::Red)?;

    // check for environment var and set path
    let mut current_path: PathBuf = PathBuf::new();
    let mut file_name: PathBuf = PathBuf::new();

    match env::var(DH_LIBRARY) {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() {
                current_path = path;
            } else {
                writeln!(
                    &mut stdout,
                    "Error: Environment Variable '{}' not set correctly!",
                    DH_LIBRARY
                )?;
                std::mem::drop(current_path);
                exit(0);
            }
        }
        Err(var_err) => {
            if let env::VarError::NotUnicode(_) = var_err {
                writeln!(&mut stdout, "ENV VAR ERROR: Not Unicode -> {:?}", var_err)?
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
    let name_test = entries[0].file_name().into_string().unwrap();

    // Write HELP_FILE_TEXT to an newly created help file.
    if entries.is_empty() || name_test == "dh" || name_test == "-h" || name_test == "--help" {
        let mut help_file_path = current_path.clone();
        help_file_path.push("help.toml");
        let mut help_file = File::create(help_file_path)?;
        help_file.write_all(HELP_FILE_TEXT.as_bytes())?;
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
    if args.len() == 1 && file_name.display().to_string() != "help.toml" {
        color_the_output_stream(&mut stdout, Color::Yellow)?;
        writeln!(&mut stdout, "No Arguments found!\nTry one of these:")?;
        color_the_output_stream(&mut stdout, Color::White)?;
        print_existing_helpfiles(entries)?;
        exit(1);
    }

    // Passing help.toml through for config and help purpose.
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
            file_name = PathBuf::from("dh.toml");
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
            writeln!(
                &mut stderr,
                "Error: Could not open file: {}",
                &current_path.display()
            )?;

            // Printing Possible Hit
            let mut filename: String;
            color_the_output_stream(&mut stdout, Color::Ansi256(220))?;
            writeln!(&mut stdout, "Did you mean?")?;
            color_the_output_stream(&mut stdout, Color::White)?;
            for entry in entries {
                filename = entry.file_name().into_string().unwrap();
                if filename.contains(&args[1]) {
                    writeln!(
                        &mut stdout,
                        "    {}",
                        filename.strip_suffix(".toml").unwrap()
                    )?;
                }
            }
            exit(0);
        }
    };
    // Read the Contents of the File
    let file_lines = std::io::BufReader::new(file);
    let mut counter = 0_usize;
    let mut input = String::new();

    // Print the file contents to std stream.
    for line in file_lines.lines() {
        match line {
            Ok(line) => {
                // Green Header and Footer
                if line.starts_with('#') && line.chars().nth(6) == Some('-') {
                    color_the_output_stream(&mut stdout, Color::Ansi256(37))?;
                    writeln!(&mut stdout, "{}", line)?;
                // Orange Highlighting (###)
                } else if line.starts_with('#')
                    && line.chars().nth(2) == Some('#')
                    && line.chars().nth(3) != Some('-')
                {
                    color_the_output_stream(&mut stdout, Color::Ansi256(208))?;
                    writeln!(&mut stdout, "  {}", String::from(&line[3..]).trim_start())?;
                // Lightblue Subheader (##)
                } else if line.starts_with('#')
                    && line.chars().nth(1) == Some('#')
                    && line.chars().nth(2) != Some('#')
                {
                    color_the_output_stream(&mut stdout, Color::Ansi256(33))?;
                    writeln!(&mut stdout, "  {}", String::from(&line[2..]).trim_start())?;
                // Yellow HEADER (#)
                } else if line.starts_with('#') && line.chars().nth(1) != Some('#') {
                    color_the_output_stream(&mut stdout, Color::Ansi256(220))?;
                    writeln!(&mut stdout, "{}", String::from(&line[2..]).trim_start())?;
                // Normal output line
                } else {
                    color_the_output_stream(&mut stdout, Color::White)?;
                    println!("    {}", line);
                }
                counter += 1;
                if counter == 32 {
                    // Do a wait after 32 lines.
                    std::io::stdin().read_line(&mut input)?;
                    counter = 0;
                }
            }
            Err(err) => {
                writeln!(&mut stderr, "{:?}", err)?;
            }
        }
    }
    // Reset Line-Color
    color_the_output_stream(&mut stdout, Color::White)?;
    Ok(())
}
