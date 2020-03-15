fn main() {
    // Integers
    let signed_8bit : i8 = 0;
    let signed_16bit : i16 = 0;
    let signed_32bit : i32 = 0;
    let signed_64bit : i64 = 0;
    let signed_128bit : i128 = 0;
    let signed_architecture_dependent : isize = 0;

    let unsigned_8bit : u8 = 0;
    let unsigned_16bit : u16 = 0;
    let unsigned_32bit : u32 = 0;
    let unsigned_64bit : u64 = 0;
    let unsigned_128bit : u128 = 0;
    let unsigned_architecture_dependent : usize = 0;

    // Number Literals
    let decimal = 12_345; // 12,345 (underscores can be used for added clarity)
    let hexadecimal = 0x77f;
    let octal = 0o127;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    // Floating Point Numbers
    let f32bit : f32 = 3.1;
    let f64bit : f64 = 3.1415;

    // Boolean
    let truthy : bool = true;
    let falsy = false;

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat : char = '😻';

    // Tuple
    // A group of numeric types
    let tuple : (f64, f64, f64) = (1.1, 2.2, 3.3);

    let (x, y, z) = tuple;

    let one_point_one = tuple.0;
    let two_point_two = tuple.1;
    let three_point_three = tuple.2;

    // Arrays
    let array = [1, 2, 3, 4, 5];
    let typed_array : [&str; 12] = [
        "one", "two", "three", "four", "five", "six", 
        "seven", "eight", "nine", "ten", "eleven", "twelve"
    ];

    let preset_array = [3; 5]; // [3, 3, 3, 3, 3]
}
