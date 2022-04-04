// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. !! Tuples have a fixed length: once declared, they cannot grow or shrink in size!!

// optional type annotations

fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}


// To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

fn main() {
  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of y is: {}", y);
}

// To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

fn main() {
  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of y is: {}", y);
}

// We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

fn main() {
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
}

// The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.