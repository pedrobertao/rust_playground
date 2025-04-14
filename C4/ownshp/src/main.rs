fn main() {
    example_string();
    example_string_push();
    example_string_scope();
}

fn example_string() {
    let s1 = String::from("hello");
    println!("Testing String: {s1}");
}

fn example_string_push() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");
}

fn example_string_scope() {
    {
        let s = String::from("hello");
    } // this scope is now over, and s is no

    // s.push_str("hello") // 20 | let s = String::from("hello"); // s is valid from this point forward
}
