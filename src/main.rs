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
use regex::Regex;
use std::fmt;



// helper functions

fn regex_bool(regex: &str, to_match: &str) -> (bool, String) {
    let re = Regex::new(regex).unwrap();
    let Some(captured_str) = re.captures(to_match) else { return (false, String::new()) };
    //println!("{:?}", captured_str);
    return (true, (&captured_str[1]).to_string());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Copy, PartialEq)]
pub enum List {
    UnorderedList,
    OrderedList,
    PossibleUlist,
    PossibleOlist,
    NotList,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::NotList => write!(f, "NotList"),
            List::OrderedList => write!(f, "OrderedList"),
            List::PossibleOlist => write!(f, "Possible-OrderedList"),
            List::UnorderedList => write!(f, "UnorderedList"),
            List::PossibleUlist => write!(f, "Possible-UnorderedList"),
            _ => write!(f, "empty/error"),
        }
    }
}

#[derive(Clone)]
struct HTML {
    fpath: String,
    fname: String,
    contents: String,
    list_data: Vec<List>,
}

impl HTML {
    pub fn new() -> Self {
        Self {
            fpath: String::new(),
            fname: String::new(),
            contents: String::new(),
            list_data: Vec::new(),
        }
    }
}

impl fmt::Display for HTML {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, r"{{ ");
        for l in self.list_data.iter() {
            write!(f, "{}", l);
        }
        write!(f, "}}")
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
    let mut new_list: List = List::NotList;
    let mut new_line: String = String::new();
    // do things to convert the md line to HTML
    for (i, ch) in line.chars().enumerate() {

        // header tracking
        if i == header && ch == '#' {
            header += 1;
        }

        /*
        // ulist tracking
        if i == 0 && (ch == '-' || ch == '+' || ch == '*') {
            new_list = List::PossibleUlist;
        }
        else if i == 1 && ch == ' ' && new_list == List::PossibleUlist {
            // add <ul> to the beginning of the string if the html is not a list yet
            if storage.list_data == List::NotList {
                new_line = "<ul>\n".to_string();
            }
            storage.list_data = List::UnorderedList;
            new_list = List::UnorderedList;
        }
        // olist tracking
        else if (i == 0 || new_list == List::PossibleOlist) && ( 
            ch == '1' ||
            ch == '2' ||
            ch == '3' ||
            ch == '4' ||
            ch == '5' ||
            ch == '6' ||
            ch == '7' ||
            ch == '8' ||
            ch == '9' )
        {
            new_list = List::PossibleOlist;
        }
        else if new_list == List::PossibleOlist && ch == ' ' {
            // add <ol> to the beginning of the string if the html is not a list yet
            if storage.list_data == List::NotList {
                new_line = "<ol>\n".to_string();
            }
            storage.list_data = List::OrderedList;
            new_list = List::OrderedList;
        }
        else if ch == '\t' && storage.list_data != List::NotList && storage.list_height == 0 && i == 0 {
            // recurse without character
            // remove first character
            let mut chars = new_line.chars();
            chars.next();
            new_line = chars.as_str().to_string();
            // say that there is no list as to trick it into making another <ol> or <ul>
            let temp_list_data = storage.list_data;
            storage.list_data = List::NotList;
            new_line = convert_line(new_line.clone(), storage);
            storage.list_data = temp_list_data;
            // TODO: make tabs work with storage.list_height
        }*/

        // maybe try pattern matching with regex lol, see below under this for statement

    }

    let mut str_start: usize = 0;

    // find how many "\t" characters there are
    let mut olist_match = regex_bool(r"(\t+)[-+[0-9]+]\ .+", &line);
    println!("{}: {}", olist_match.0, olist_match.1.len());
    if olist_match.0 && olist_match.1.len() == storage.list_data.len() + 1 {
        // if there's an indentation, create a new <ol> or <ul> by adding 1 to the list_height and recursing
        let str_middle: String = line.chars().skip(olist_match.1.len()).take(line.len()).collect();
        new_line = convert_line(str_middle, storage);
        

        return format!("{}{}", olist_match.1, new_line);
    }

    // this should find a number, a period, then text. ex: 1. hi! (olist)
    olist_match = regex_bool(r"([0-9]+\.\ ).+", &line);
    if olist_match.0 {
        if storage.list_data.is_empty() {
            new_line = "<ol>\n".to_string();
            storage.list_data.push(List::OrderedList);
        }
        else if storage.list_data.ends_with(&[List::UnorderedList]) {
            new_line = "</ul>\n<ol>\n".to_string();
            storage.list_data.pop();
            storage.list_data.push(List::OrderedList);
        }
        str_start += olist_match.1.len();
        new_list = List::OrderedList;
    }
    // this should find a - or +, then text. ex: - hi! (ulist)
    else if regex_bool(r"([-+]\ ).+", &line).0 {
        // add <ul> to the beginning of the string if the html is not a list yet
        if storage.list_data.is_empty() {
            new_line = "<ul>\n".to_string();
            storage.list_data.push(List::UnorderedList);
        }
        else if storage.list_data.ends_with(&[List::OrderedList]) {
            new_line = "</ol>\n<ul>\n".to_string();
            storage.list_data.pop();
            storage.list_data.push(List::UnorderedList);
        }
        new_list = List::UnorderedList;
    }
    //(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})

    // if there was a leading space but nothing else, return to NotList
    if new_list == List::PossibleUlist || new_list == List::PossibleOlist {
        new_list = List::NotList;
    }

    // Find the middle part of the string
    str_start +=  header + (1*((header>0) as usize)) + 
                            2 * ((new_list == List::UnorderedList) as usize);
    
    let str_middle: String = line.chars().skip(str_start).take(line.len()).collect();

    // end the list
    if new_list == List::NotList {
        if storage.list_data.ends_with(&[List::UnorderedList]) {
            new_line = "</ul>\n".to_string();
            storage.list_data.pop();
        }
        else if storage.list_data.ends_with(&[List::OrderedList]) {
            new_line = "</ol>\n".to_string();
            storage.list_data.pop();
        }
    }

    // put the header in
    if header > 0 {
        new_line += &format!("<h{}>{}</h{0}>\n", header, str_middle);
        // headers do not cause recursion
    }
    else if new_list != List::NotList {
        // add the list
        new_line += &format!("<li>{}</li>\n", str_middle);
        //TODO: recurse w/ str_middle to find if header w/out list
    }
    else {
        new_line += &format!("{}\n", str_middle);
    }

    println!("{}List Data: {}", new_line, storage);
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

    // convert the contents of the HTML datatype into a file specified within the HTML datatype
    pub fn to_file(&self) {
        // Create a path to a new file
        let new_full_fpath: String = format!("{}{}{}", self.fpath, self.fname, ".html");
        let path: &Path = Path::new(&new_full_fpath);
        let display: std::path::Display<'_> = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file: File = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the string to `file`, returns `io::Result<()>`
        match file.write_all(self.contents.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {/*println!("successfully wrote to {}", display)*/},
        }
    }
}

fn run_md() {
    let string: &str = "Hello World!";
    println!("{}", string);
    let md: Markdown = Markdown {
        fpath: /*r#"E:\Programs\Sync\Notes\"*/ r#"C:\Users\cmulholla\Sync\Notes\"#.to_string(),
        fname: "NASA internship guide".to_string(),
        contents: String::new() 
    };

    let html: HTML = HTML::from_markdown(md);
    //println!("{}", html.contents);
    html.to_file();

}

fn main() {
    run_md();
}