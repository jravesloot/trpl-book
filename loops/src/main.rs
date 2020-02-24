fn main() {
    let mut counter = 0;
    let result = loop { // holds value returned by loop
        counter += 1;

        if counter == 10 {
            break counter * 2  // break can return a value
        }
    };
    println!("The value of result is {}", result);

    let mut counter = 3;
    while counter > 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("Bl as  t   o    f    f        !");

    let array: [u8; 4] = [10, 20, 40, 80];
    for element in array.iter() {
        println!("Element value: {}", element);
    }

    // Range object, reverse method
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("You gotta bleast awff");
}
