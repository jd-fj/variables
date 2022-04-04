// Every element of an array must have the same type. Arrays have a fixed length.

fn main() {
  let a = [1, 2, 3, 4, 5];

  let bob: [i32; 5] = [1, 2, 3, 4, 5];
}

// Arrays are useful when you want your data allocated on the stack rather than the heap. Or when you want to ensure you always have a fixed number of elements.

//  A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size

let a = [3; 5];

// This is the same as writing:

let a = [3, 3, 3, 3, 3]; 
// but in a more concise way.

fn main() {
  let a = [1, 2, 3, 4, 5];

  let first = a[0];
  let second = a[1];
}

