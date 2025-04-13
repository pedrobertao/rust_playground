fn main() {
    let x = fib1(10);
    let y = fib2(10);
    println!("Fib1 {x} Fib2 {y}");
}

fn fib1(n: i32) -> i32 {
    // Começamos com os dois primeiros números da sequência de Fibonacci
    let mut a = 0; // fib(0)
    let mut b = 1; // fib(1)

    // Repetimos o processo n vezes
    for _ in 0..n {
        let temp = b; // Guardamos o valor atual de b
        b = a + b; // Atualizamos b para o próximo número da sequência
        a = temp; // a agora vira o valor antigo de b
    }

    // No final, 'a' será o n-ésimo número de Fibonacci
    a
}

fn fib2(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}
