fn main() {
    let x = 5;
    // println!("The value of x is {}", x);
    // x = 10;  // compile time error for multiple assignment of immutable

    let x = x + 1; // "shadows" the original x, allowing transformatins on an immutable variable
    let x = x * 2; // "shadows" the above x, allowing to change the type of x
    println!("The value of x is {}", x);

    let emoji = '🤔'; // U+1F914
    println!("{}", emoji);

    // tuples can be mixed type; are fixed size
    let tup: (i32, f64, u8) = (249, 4.49, 100);
    // getting values by "destructuring" the tuple
    let (_x, y, _) = tup;  // prepending an underscore tells compiler we don't expect to use the value
    println!("The value of y is {}", y);
    // access a value by position with tuple.index
    println!("The value of _x is {}", tup.0);

    // arrays are also fixed length but must consist of one data type
    // useful when you want data allocated on stack rather than heap
    // use a vector (from std lib) for a collection that can grow/shrink
    let _array: [u8; 5] = [1, 33, 4, 5, 2];
    // or initialize with same values * len of array
    let array = [0; 5];  // [0, 0, 0, 0, 0]
    println!("The first element is {} and the last element is {}", array[0], array[4]);
    // out of bounds access, eg. array[10] will panic at runtime as a safety precaution
}
