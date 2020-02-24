fn main() {
    println!("Hello, world!");

    let x = 88;  // a statement, which by definition doesn't return anything
    another_function(x);  // function calls are statements

    let y = {
        let z = 12;
        z + 13 // without a semicolon this is an expression that returns a value; with it would be a statement w/o return value
    };  // curly brackets block is an expression
    println!("The value assigned to y is {}", y);

    let f = five();
    println!("The value of f is... {}!", f);

    let f = plus_one(f);
    println!("The value of plus_one(f) is {}", f);
}

fn another_function(x: i32) {
    println!("Inside another_function! Received value {} for x", x);
}

fn five() -> u8 { // return type specified with -> after params
    5 // without an explicit 'return', functions return value from last expression
}

fn plus_one(x: u8) -> u8 {
    x + 1
}
