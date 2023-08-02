fn hello() {
    let number: f64 = 5.0;
    let string = "test";

    println!("{0}", string);
    println!("Hello World!");

    // formatting
    println!("{stringVar} {floatVar}",
              stringVar=string,
              floatVar=number);
    let formattedString = format!("s {0} {1}", number, string);
    println!("{}", formattedString);

    // bases
    println!("Base 2:            {:b}", 69420);
    println!("Base 8:            {:o}", 69420);
    println!("Base 16 (lower):   {:x}", 69420);
    println!("Base 16 (upper):   {:X}", 69420);

    // number formatting
    let pi = 3.141592653589;

    let pistr = format!("{pi:.3}");
    println!("pi is roughly -{0:^10}-", pistr)

    // other rules:
    /*
        format!: write formatted text to String
        print!: same as format! but the text is printed to the console (io::stdout).
        println!: same as print! but a newline is appended.
        eprint!: same as print! but the text is printed to the standard error (io::stderr).
        eprintln!: same as eprint! but a newline is appended.
    */
}

//use rand::{self, Rng};
    //let x: f32 = rand::thread_rng().gen();
    //println!("{}", x);

fn main() {
    hello();
}