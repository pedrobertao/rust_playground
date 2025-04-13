fn main() {
    let temp: f64 = 100.0;
    let x = f_to_c(temp);
    println!("Temp {temp}F in celcius is: {x} ");
}

fn f_to_c(f: f64) -> f64 {
    //°F=(°C×9/5)+32
    //(F-32)5/9 = C
    (f - 32.0) * 5.0 / 9.0
}
