// integers types can be signed or unsigned. Signed integers use the i prefix, unsigned integers use the u prefix.

// unsigned means it's a positive integer, signed means its important to know if it's positive or negative.

// INTEGER TYPES //  ---> Rust’s integer types default to i32
    // 8-bit: i8, u8  ---> -128 to 127
    // 16-bit: i16, u16 ---> 0 to 255
    // 32-bit: i32, u32
    // 64-bit: i64, u64
    // 128-bit: i128, u128
    // arch: isize, usize

//isize and usize types are 64 or 32 bits depending on the architecture of the computer. the primary situation in which you’d use isize or usize is when indexing some sort of collection.
//an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.