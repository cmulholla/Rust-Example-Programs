// section 2: liternals and operators

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

use std::fmt;
use std::mem;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn hello() {
    let string = "Hello World!";
    println!("{}", string);
    // tuples
    let tuple = (1e4f32, -0.1, 'h', "potato", false);

    /*
    //This code errors out: https://stackoverflow.com/questions/44041673/create-an-array-from-a-tuple
    for i in 0..5 {
        print!("{} ", tuple[i]);
    }*/

    let (a, b, c, d, e) = tuple;
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("transpose:\n{}", transpose(matrix));

    // arrays and slices
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let _ys: [i32; 500] = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    analyze_slice(&xs);

    // mostly the same as python feeling, but slices are done by xs[0..3]
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}

fn main() {
    hello();
}