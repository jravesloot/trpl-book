fn main() {
    let mut s1 = String::from("hello"); // allocated on heap so as to be mutable/growable (vs string literal)

    s1.push_str(", ownership!"); // appends a literal to a String

    println!("{}", s1); // 'hello, ownership!'

    // String data (pointer, len in bytes, capacity in bytes) copied to s2.
    // The actual String contents on the heap is not copied
    let s2 = s1;

    // s1 is now considered invalid (prevents 'double free' errors on scope-leaving cleanup)
    // and s1 was 'moved' into s2
    // println!("{}", s1);  // fails to compile

    // clone copies the String on the heap; both s2 and s3 remain valid
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // types such as integers that have a known size at compile time are stored
    // entirely on the stack, so copies of the actual values are quick to make.
    // x is still valid and wasn’t moved into y.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
