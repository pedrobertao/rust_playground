fn main() {}

// dangle = a pointer that references a location in memory that may have been given to someone else
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn mut_rereference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); // ;
                               // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn fix_reference() {
    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("Changed '{s2}'.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn reference_example() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}
