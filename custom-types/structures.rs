use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right
  top_left: Point,
  bottom_right: Point,
}

impl Display for Rectangle {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Top left: ({}, {})\nBottom right: ({}, {})",
      self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y)
  }
}

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle { top_left, bottom_right } = rect;
  (top_left.y - bottom_right.y).abs() * (top_left.x - bottom_right.x).abs()
}

// TODO ^ Add a function square which takes a Point and a f32 as arguments,
// and returns a Rectangle with its lower left corner on the point, and a
// width and height corresponding to the f32.
// fn square(point: Point, number: f32) -> Rectangle {}

fn main() {
  // Create struct with field init shorthand
  let name = String::from("Petter");
  let age = 27;
  let peter = Person { name, age };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Access the fields of the point
  println!("point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our
  // other one
  let bottom_right = Point { x: 5.2, ..point };

  // `bottom_right.y` will be the same as `point.y` because we used that field
  // from `point`
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);


  // Destructure the point using a `let` binding
  let Point { x: top_edge, y: left_edge } = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("{}", _rectangle);
  println!("Rectangle area: {:?}", rect_area(_rectangle));
}