use std::fmt;

struct Structure(i32);

// to use the {} print for this, it must be implemented manually.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the first element to the output stream
        // `f` is whether it was a success
        write!(f, "{}", self.0)
    }
}

struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


// --------- display list ----------------
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

// ----- formatting ------
struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl fmt::Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reds = format!("{:0>2}", format!("{:X}", self.red));
        let greens = format!("{:0>2}", format!("{:X}", self.green));
        let blues = format!("{:0>2}", format!("{:X}", self.blue));
        write!(f, "RGB ({}, {}, {}) 0x{}{}{}", self.red, self.green, self.blue, reds, greens, blues)
    }
}
/*
struct LinkedList {
    head: Node,
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedListIter<'a> {
    current_node: &'a Node,
}

impl Iterator for LinkedList {
    fn next(&mut self) -> Option<Box<Node>> {
        Option()
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(");
        for (count, v) in vec.iter().enumerate() {
            write!(f, "", )
        }
    }
}
*/

struct BaseHopping {
    curr_val: u64,
}

impl Iterator for BaseHopping {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.curr_val;
        let base_val: String = format!("{:b}", current);
        current = base_val.parse().unwrap();
        self.curr_val = current;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}

fn base_hop(a: u64) -> BaseHopping {
    BaseHopping { curr_val: a }
}

fn hello() {
    /*let string = "Hello World!";
    println!("{}", string);

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!();

    let complex_number = Complex { real: 3.3, imag: 7.2 };
    println!("{}", complex_number);

    // ----- list -----
    
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // ----- formatting -----
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:}", color);
    }*/

    /*let mut q1: LinkedList;
    q1 = LinkedList { value = 1, head = *q1, next = None };
    q1.next = Box<Node> { value = 2, head = *q1, next = None };

    let n1 = Node {
        value: 5,
        next: None
    };

    let n2 = Node {
        value: 1,
        next: Some(Box::new(n1))
    };

    let ll = LinkedList {
        head: n2
    };
    // ll -> n2 -> n1
    */
    
    for i in base_hop(2) {
        print!("{}, ", i);
    }
    println!();


}

fn main() {
    hello();
}