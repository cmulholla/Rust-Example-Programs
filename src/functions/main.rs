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

// A struct with two fields
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //find length of top/bottom edge
        let Rectangle { 
            top_left: Point {
                x: topl_x, 
                y: topl_y
            }, 
            bottom_right: Point {
                x: bottomr_x, 
                y: bottomr_y
            }
        } = self;

        write!(f, "top left: ( x: {}, y: {} ) :: bottom right: ( x: {}, y: {} )", topl_x, topl_y, bottomr_x, bottomr_y)
    }
}

// ACTIVITY
// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
fn rect_area(rect: Rectangle) -> f32 {
    //find length of top/bottom edge
    let Rectangle { 
        top_left: Point {
            x: topl_x, 
            y: topl_y
        }, 
        bottom_right: Point {
            x: bottomr_x, 
            y: bottomr_y
        }
    } = rect;

    (topl_x-bottomr_x) * (topl_y-bottomr_y)
}

fn square(top_left_p: Point, edge_len: f32) -> Rectangle {
    Rectangle { top_left: top_left_p, bottom_right: Point { x: top_left_p.x + edge_len, y: top_left_p.y - edge_len } }
}

fn hello() {
    let string = "Hello World!";
    println!("{}", string);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Make a new point by using struct update syntax to use the fields of our other one
    // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    let bottom_right = Point { x: 5.2, ..point };

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: _top_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: 4.2 },
        bottom_right: bottom_right,
    };

    println!("{}", rect_area(rectangle));
    print!("sqare: {}\narea: {}", square(point, -3.0), rect_area(square(point, -3.0)));
}

fn main() {
    hello();
}