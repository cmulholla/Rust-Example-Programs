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

// helper functions


struct HTML(String);

struct Markdown(String);

fn print_file (fname: &str) -> Markdown {
    let md: Markdown = Markdown(fname.to_string());
    return md;
}

impl Markdown {
    fn from_file(filename: &str) -> Self {
        print_file("potato")
    }
}

fn hello() {
    let string: &str = "Hello World!";
    println!("{}", string);
}

fn main() {
    hello();
}