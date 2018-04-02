use std::env;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;

// mod console;
const CONFIG_FILE: &'static str = "help.hlp";

fn main() {
    let path = &std::env::current_exe().unwrap();
    let path = path.parent().unwrap();
    let path = format!("{}\\{}",path.display(), CONFIG_FILE);

    let help_entries: Vec<String> = get_hlp_entries(&path);
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No Arguments found!");
        return;
    }

    let mut contains = false;
    for cmd in help_entries {
        if cmd == args[1] {
            contains = true;
        }
    }
    if contains == true{
        return_file_text(&args[1])
    }
    else {
        println!("Argument not valid!\nTry: \"help\"");
    }
}


/// Reads all help entries from CONFIG_FILE_PATH
fn get_hlp_entries(config_file_path: &str) -> Vec<String> {
    let help_file_text = "Command  Description\n\
                          help    this file!\n\
                          test    Test\n".as_bytes();

    // open config file                      
    let mut file = match File::open(config_file_path) {
        Err(_) => {
            println!("\nError: No Configuration File found!\n\
                      There needs to be a \"help.hlp\" file in the root dir");

            // create if not existing
            let mut file = match File::create(CONFIG_FILE) {
                Err(why) => panic!("Error: Could not create File:\n{}", why),
                Ok(new_file) => new_file,
            };

            // writing example config to new file
            match file.write_all(help_file_text) {
                Err(why) => panic!(
                    "Error: Could not write to \"{}\"\n{}",
                    CONFIG_FILE, why
                ),
                Ok(write) => write,
            };

            // force to restart
            println!("File has now been created");
            exit(0);
        }

        Ok(data) => data,
    };

    // reading the config file to get list with possible commands
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Error: Could not read {}:\n {}", CONFIG_FILE, why),
        Ok(cnt) => cnt,
    };

    // push it to the vec which gets returned
    let mut cmd_list: Vec<String> = Vec::new();
    for line in content.lines() {
        cmd_list.push(line.split_whitespace().nth(0).unwrap().to_string());
    }
    
    cmd_list
}


/// prints the text for thr associated command
fn return_file_text(filename: &str) {

    let path = &std::env::current_exe().unwrap();
    let path = path.parent().unwrap();
    
    let mut file_text = String::new();
    let filepath = format!("{}\\{}.hlp", path.display() , filename);
    
    let mut file = match File::open(&filepath) {
        Err(error) => panic!("\nCould not open: \"{}\":\n{}\n", filepath, error ),
        Ok(file) => file,
    };
    match file.read_to_string(&mut file_text) {
        Err(error) => panic!("{}", error),
        Ok(text) => text,
    };

    println!("{}", file_text);

    // let mut line_count = 0;
    // for (count, line) in file_text.lines().enumerate() {
        // println!("{}", line);
        // line_count += 10;
        // if count == line_count {
            // console::pause();
        // }#
    // }
}