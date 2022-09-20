fn main() {
    println!("\n");
    println!("immutables");
    println!("\t\tby default, variables in rust are immutable");
    let x = 5;
    println!("\n\t\tthe value of x: {}", x);
    
    let mut y = 0;
    y += 1;
    println!("\t\tthe value of y: {}", y);

    println!("constants");
    println!("\t\t'constants' have unchanging values/types");
    println!("\t\tthey can also be annotated by using ':'");
    println!("\t\texample: use 'u32', a primitive type representing a 32-bit integer");
    const MAX_POINTS: u32 = 100_000;
    println!("\n\t\tconstant value: {}", MAX_POINTS);
    
    println!("shadowing");
    println!("\t\t'shadowing' is declaring a new variable using previous variable");
    println!("\t\tshadowing is not mutability (does allow for altering the data type)");
    println!("\t\tuse: shadow to change data type, mutate to change variable's value (keeping same type)");

    let z = 100;
    println!("\n\t\toriginal value of z: {}", z);
    let z = z + 1; // shadowed
    println!("\t\tshadowed value of z: {}", z);
    let z = z * 2; // shadowed again
    println!("\t\tvalue of z (shadowed again): {}", z);
    let spaces = "     ";
    println!("\n\t\tstring of spaces: {}", spaces);
    let spaces = spaces.len(); // shadow var, changed type from string to int
    println!("\t\tthere are {} (integer) spaces in this string", spaces);
    println!("\nDATA TYPES NEXT\n");
}

fn divider_lines (){
    println!("--------------------------------------------------------------------------");
}
