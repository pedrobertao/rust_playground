fn tuple() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {x} {y} {z}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value is: {five_hundred} {six_point_four} {one}");
}

fn arrays() {
    let arr_with_size: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_with_size_2 = [3; 5];
    println!("The values: {0}", arr_with_size_2[1]);

    let arr_access = [1, 2, 3, 4, 5];
    let first = arr_access[0];
    let second = arr_access[1];
    println!("The values: {first} {second}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
