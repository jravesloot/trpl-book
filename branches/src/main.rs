fn main() {
    let number = 5;

    if number < 5 {
        println!("The number {} was < 5", number);
    } else if number > 5 {
        println!("The number {} was > 5", number);
    } else {
        println!("The number was {}", number);
    }

    let condition = true;
    // values in each arm of conditional assignment must be of same type so that
    // compiler can make guarantees
    let number = if condition {
        1
    } else {
        0
    };
    println!("The number was set to {}", number);
}
