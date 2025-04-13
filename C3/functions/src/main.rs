fn main() {
    another_function(1);
    print_labeled_measurement(5, 'h');
    statement_example();
    example_call_return();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statement_example() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn example_call_return() {
    let x = example_return();
    println!("The value of x is: {x}");

    let x_2 = plus_one(5);
    println!("The value of x is: {x_2}");
}

fn example_return() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
