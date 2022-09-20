fn main() {
    data_types();
    scalar_types();
    operator_types();
    boolean_types();
    character_types();
    compound_types();
}

fn new_line() {
    println!("\n");
}

fn data_types() {
    println!("DATA TYPES");
    println!("\t\t--every value in Rust is a data type.");
    println!("\t\t--this tells Rust what kind of data is being specified.");
    println!("\t\t--Rust is static (or omnipotent); @ compile time, compiler must know the types of all variables.");
    println!("\t\t--example: let guess: u32 = \"42\".parse().expect(\"not a number!\")");
    new_line();
}

fn scalar_types() {
    println!("SCALAR TYPES");
    println!("\t\t--scalars represent a single value (values are what \"scales\" a program.");
    println!("\t\t--examples: integers, floating-point numbers, Booleans and characters.");
    new_line();
    integers();
    floating_point_types();
}

fn integers() {
    println!("INTEGERS");
    signed_integers();
    unsigned_integers();
}

fn unsigned_integers() {
    let x: u8 = 32;
    let y: u16 = (x * 2).into(); // when changing bit values, use into() method
    let z: usize = 100_000_000;
    println!("\t\tUNSIGNED INTEGERS");
    println!("\t\t--annotation types for unsigned integers: u8, u16, u32, u64, u128, usize");
    println!("\t\t--usize reflects the architecture of your computer (32 bit or 64 bit)");
    println!("\t\t--declared let a: u8 = 32, the signed 8bit integer is: {}", x);
    println!("\t\t--declared let b: u16 = (x * 2), the signed 16bit integer is: {}", y);
    println!("\t\t--declared let c: usize = 100_000_00, the signed isize integer is: {}", z);
    println!("\t\t--when shadowing a smaller bit value to declare larger bit value (think: 16bit value derived from 8bit value), use the .into() method");
    println!("\t\t--let y: u16 = (x * 2).into();");
    new_line();
}

fn signed_integers() {
    let a: i8 = -32;
    let b: i16 = (a * -3).into(); // when changing bit values, use into() method
    let c: isize = -100_000_000; //
    println!("\t\tSIGNED INTEGERS");
    println!("\t\t--annotation types for signed integers: i8, i16, i32, i64, i128, isize");
    println!("\t\t--isize reflects the architecture of your computer (32 bit or 64 bit)");
    println!("\t\t--declared as: let a: i8 = -32, the signed 8bit integer is: {}",a);
    println!("\t\t--declared as: let b: i16 = (a * -3).into(), the signed 16bit integer is: {}", b);
    println!("\t\t--declared as: let c: isize = -100_000_00, the signed isize integer is: {}", c);
    new_line();
}

fn floating_point_types() {
    let x: f64 = 2.3; // f64
    let y: f32 = 3.1; // f32, using annotation
    println!("FLOATING POINT TYPES");
    println!("\t\t--Rust has two primitive types for floating-point numbers (numbers with decimal points).");
    println!("\t\t--primitives are: f32 and f64, 32bit floating points and 64bit floating point, respectively.");
    println!("\t\t--example: let x: f64 = {}", x);
    println!("\t\t--example: let y: f32 = {}", y);
    new_line();
}

fn operator_types() {
    println!("***** omitting operator b/c familiar with +, -, /, *, % *****");
    new_line();
}

fn boolean_types() {
    let t: bool = true;
    let f: bool = false; // explicit type annotation
    println!("BOOLEAN TYPES");
    println!("\t\t--booleans are true or false values.");
    println!("\t\t--mainly used in conditionals like if expressions.");
    println!("\t\t--example: i want to build projects: {}!", t);
    println!("\t\t--boolean declared as: let f: bool = {}", f);
    new_line();
}

fn character_types() {
    let c = 'z';
    let e: char = '@';
    let emoji: char = '🧩';
    println!("CHARACTER TYPES");
    println!("\t\t--in Rust (like in Java), \"char\" is the most primitive alphabetic type.");
    println!("\t\t--char literals are always specified with single quotes.");
    println!("\t\t--char declared as: let c = 'z'");
    println!("\t\t--alphabet as character: {}", c);
    println!("\t\t--symbol as character: {}", e);
    println!("\t\t--emoji as character: {}", emoji);
    new_line();
}

fn compound_types() {
    println!("COMPOUND TYPES");
    println!("\t\t--compound types can group multiple values into one type.");
    println!("\t\t--the two primitives here are: TUPLES and ARRAYS.");
    new_line();
    tuple_types();
    array_types();
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // unsigned 32 bit value, floating point 64 bit value, and utf8 value
    let (x, y, z) = tup;
    let a = tup.0; // first element
    let b = tup.1; // second element
    let c = tup.2; // third element
    println!("\t\tTUPLE TYPES");
    println!("\t\t--tuples are general ways of grouping togther a set of values of various types");
    println!("\t\t--tuples are fixed sets; once set, a tuple cannot grow or shrink in size (number of elements");
    println!("\t\t--tuple declared as let tup: (i32, f64, u8) = (500,6.4,1)");
    println!("\t\t--to get individual values from a tuple, use pattern matching to destructure the tuple");
    println!("\t\t--pattern mathching declared as: '(x,y,z) = tup'");
    println!("\t\t--in addition to destructuring, tuples use a \".\" notation to access the index of a value");
    println!("\t\t--tuple dot notation: tup.0 is {}, tup.1 is {}, tup.2 {}",a, b, c);
    println!("\t\tx is: {}\ty is: {}\tz is {}", x, y, z);
    new_line();
}

fn array_types() {
    let a: [i32; 3] = [300_000, 140_000, 50_000];
    let b = [3;5]; // an array of five lements all with identical values
    let alphabets: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']; 
    println!("\t\tARRAYS");
    println!("\t\t--arrays are on the stack, not in the heap");
    println!("\t\t--unlike tuples, every element of an array must have the same type.");
    println!("\t\t--Rust arrays have a fixed length (this is diff from other languages).");
    println!("\t\t--arrays are declared with bracket notation.");
    println!("\t\t--array of numbers declared as let a: [usize; 3] = [300_000, 140_000, 50_000] ");
    println!("\t\t--an array's elements are accessed via indexing => arrayName[int] => myArray[0]");
    println!("\t\t--the first alphabet in 'alphabets' is: {}", alphabets[0]);
    println!("\t\t--the first alphabet in 'array a' is: {}", a[2]);
    println!("\t\t--the first alphabet in 'array b' arrary is: {}", b[3]); 
    new_line();
}
