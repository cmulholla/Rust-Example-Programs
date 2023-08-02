
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn hello() {
    let number = 5;
    let string = "test";
    let bob = Person { name: string, age: number };

    //pretty print using debug
    println!("{0:#?}", bob);
}

//use rand::{self, Rng};
    //let x: f32 = rand::thread_rng().gen();
    //println!("{}", x);

fn main() {
    hello();
}