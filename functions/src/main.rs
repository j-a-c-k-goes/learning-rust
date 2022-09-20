fn main() {
    println!("FUNCTIONS");
    new_line();
    function_def();
    function_parameters();
    function_state_exp();
}

fn new_line() {
    println!("\n");
}
fn function_def(){
    let b_l: char = '{';
    let b_r: char = '}';
    println!("\t\tDEFINITION");
    println!("\t\t--function are pervasive (used widely) in Rust");
    println!("\t\t--the main function (fn main() {}{}) is the entry point of most programs", b_l, b_r);
    println!("\t\t--the \"fn\" keyword allows a new function to be declared");
    println!("\t\t--when naming functions, use snake case — lowercase with words separted by underscores");
    new_line();
}
fn function_parameters(){
    let b_l: char = '{';
    let b_r: char = '}';
    println!("\t\tPARAMETERS");
    println!("\t\t--functions can be defined with parameters, special variables which are part of a function's signature.");
    println!("\t\t--technically these parameters are called \"arguments\"");
    println!("\t\t--written as fn(parameter) {}{}", b_l, b_r);
    println!("\t\t--the type must be declared when declaring the parameter");
    f_w_p(3);
    f_w_two_p(303, '@');
    new_line();

}

fn function_state_exp(){
    let b_l: char = '{';
    let b_r: char = '}';
    println!("\t\tSTATEMENTS AND EXPRESSIONS");
    println!("\t\t--function bodies are a series of statements optionally ending in an expression.");
    println!("\t\t--because Rust is an expression based language, this is an important concept.");
    println!("\t\t--statements are instructions that perform some action and do not return a value.");
    println!("\t\t--expressions evaluate to a resulting value.");
    println!("\t\t--expressions can be part of statements");
    println!("\t\t--calling a function is an expression, so is calling a macro");
    println!("\t\t--the block used to create new scopes, {}{}", b_l, b_r);
    println!("\t\t--statements do not return values, so \"let x = (let y = n) or let x = y = n\" is no good");
    println!("\t\t--expression: let y {} let x = n; x + n {};", b_l, b_r);
    println!("\t\t--do not use semicolons when writing expressions within scope of function");
    f_w_e();
    new_line();

}

fn f_w_e (){
    let y = {
        let x = 3;
        x + 100_000 // no semicolon, semicolon make statements
    };
    
    println!("\t\t**function's value is: {}**", y);
}

fn f_w_p(x: i32){
    println!("\t\t--this function uses parameter: {}", x);
}

fn f_w_two_p(x: i32, y: char){
    println!("\t\t--this function uses two parameters: {} & {}", x, y);
}

fn function_bodies(){}
fn function_return_values(){}