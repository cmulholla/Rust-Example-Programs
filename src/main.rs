/*
to run:
 - use `rustc main.rs` in the terminal
 - use `.\main.exe` (or type main then tab)
 OR
 - use cargo build
 - use cargo run (build not necessary)
*/

/*
to create:
 - use cargo new <name>
 - use cargo add <dependancy>
*/

// use
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

// helper functions

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// main functions

// takes in a markdown file path and name and converts it into an html file
fn create_html_from_md(fpath: &str, fname: &str) {
    // Concat the file path and name
    let fullfpath: String = format!("{}{}{}", fpath, fname, ".md");

    // Create a path to a new file
    let new_full_fpath: String = format!("{}{}{}", fpath, fname, ".html");
    let path: &Path = Path::new(&new_full_fpath);
    let display: std::path::Display<'_> = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut str_line: String;
    let ulist: bool = false;
    let last_ulist: bool = false;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(fullfpath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                str_line = ip;
                
                // convert string into character vector to make it easier to pick apart
                //let char_line: Vec<char> = str_line.chars().collect::<Vec<_>>();

                // Replace anything markdown related with the HTML counterpart
                let mut header: usize = 0;
                let mut possible_ulist: bool = false;
                let mut new_line: String = String::new();
                for (i, ch) in str_line.chars().enumerate() {

                    if i == header && ch == '#' {
                        header += 1;
                    }
                    if i == 0 && ch == '-' {
                        possible_ulist = true;
                    }
                    else if i == 1 && ch == ' ' && possible_ulist {
                        // TODO: make ulist end by tracking last string using the last_ulist variable
                        if !ulist {
                            new_line = "<ul>\n<li>".to_string();
                        }
                        else {

                        }
                    }
                }
                // Find the middle part of the string
                let str_start: usize = header+(1*((header>0) as usize));
                let str_middle: String = str_line.chars().skip(str_start).take(str_line.len()).collect();

                // put the header in
                if header > 0 {
                    new_line = format!("<h{}>{}</h{0}>", header, str_middle);
                }
                else if new_line.is_empty() {
                    new_line = format!("<p>{}</p>", str_middle);
                }
                else if ulist {
                    new_line += &format!("<li>{}</li>", str_middle);
                }
                else {
                    new_line += &str_middle;
                }


                // Write the string to `file`, returns `io::Result<()>`
                match file.write_all(new_line.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => {println!("successfully wrote to {}", display)},
                }
            }
        }
    }
}

fn main() {
    create_html_from_md("E:\\Programs\\Sync\\Notes\\","NASA internship guide");
}