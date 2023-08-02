/*
to run:
 - use `rustc main.rs` in the terminal
 - use `.\main.exe` (or type main then tab)
 OR
 - use cargo build
 - use cargo run
*/

/*
to create:
 - use cargo new <name>
 - use cargo add <dependancy>
*/

fn hello() {
    let string = "Hello World!";
    println!("{}", string);
}

fn main() {
    hello();
}