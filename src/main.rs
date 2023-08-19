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

struct HTML(String);

struct Markdown(String);

impl Markdown {
    fn from_file(filename: &str) -> Self {
        todo!()
    }
}

fn hello() {
    let string: &str = "Hello World!";
    println!("{}", string);
}

fn main() {
    hello();
}