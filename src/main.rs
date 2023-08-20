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

#[derive(Clone)]
struct HTML {
    fpath: String,
    fname: String,
    contents: String,
    in_ulist: bool,
}

impl HTML {
    pub fn new() -> Self {
        Self {
            fpath: String::new(),
            fname: String::new(),
            contents: String::new(),
            in_ulist: false,
        }
    }
}

struct Markdown {
    fpath: String,
    fname: String,
    contents: String,
}

impl Markdown {
    pub fn new() -> Self {
        Self {
            fpath: String::new(),
            fname: String::new(),
            contents: String::new(),
        }
    }
}

// input a single line of the file, and the storage variable
// returns the line in HTML style
fn convert_line (line: String, storage: &mut HTML) -> String {
    let mut header: usize = 0;
    let mut ulist: bool = false;
    let mut new_line: String = line.clone();
    // do things to convert the md line to HTML
    for (i, ch) in line.chars().enumerate() {

        // header tracking
        if i == header && ch == '#' {
            header += 1;
        }
    }

    // Find the middle part of the string
    let str_start: usize = header+(1*((header>0) as usize)) + 2*(ulist as usize);
    let str_middle: String = line.chars().skip(str_start).take(line.len()).collect();
    // put the header in
    if header > 0 {
        new_line = format!("<h{}>{}</h{0}>\n", header, str_middle);
    }
    else {
        new_line = format!("{}\n", str_middle);
    }

    storage.in_ulist = ulist;
    print!("{}", new_line);
    return new_line;
}

impl HTML {
    pub fn from_file(filepath: &str, filename: &str) -> Self {
        // Create a path to the existing file
        let full_fpath: String = format!("{}{}{}", filepath, filename, ".md");

        // create the place to store the data
        let mut storage: HTML = HTML::new();
        let mut html_contents: String = String::new();

        // File hosts.txt must exist in the current path
        if let Ok(lines) = read_lines(full_fpath) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    // add converted lines into the md file, share storage across lines
                    html_contents += &convert_line(ip, &mut storage);
                }
            }
        }
        storage.contents = html_contents;
        storage.fpath = filepath.to_string();
        storage.fname = filename.to_string();

        // return the html file
        return storage;
    }

    pub fn from_markdown(md: Markdown) -> Self {
        Self::from_file(&md.fpath, &md.fname)
    }

    pub fn to_file(html: Self) {
        // Create a path to a new file
        let new_full_fpath: String = format!("{}{}{}", html.fpath, html.fname, ".html");
        let path: &Path = Path::new(&new_full_fpath);
        let display: std::path::Display<'_> = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file: File = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the string to `file`, returns `io::Result<()>`
        match file.write_all(html.contents.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {println!("successfully wrote to {}", display)},
        }
    }
}

fn run_md() {
    let string: &str = "Hello World!";
    println!("{}", string);
    let md: Markdown = Markdown {
        fpath: r#"E:\Programs\Sync\Notes\"#.to_string(),
        fname: "NASA internship guide".to_string(),
        contents: String::new() 
    };

    let html: HTML = HTML::from_markdown(md);
    println!("{}", html.contents);
    HTML::to_file(html);

}

fn main() {
    run_md();
}