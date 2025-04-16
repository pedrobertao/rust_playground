fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // println!("{}", s); // s no longer valid

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
    println!("s1 {}", s1); // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
                                       // after takes and gives back s2 is dropped
    println!("s3 {}", s3);

    calculate_length(s3);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len(); // len() returns the length of a String
    println!("The length of '{s}' is {len}.");
    (s, len)
}
