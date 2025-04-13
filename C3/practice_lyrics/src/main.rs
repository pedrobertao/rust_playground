fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[day]
        );

        // Mostra os presentes do dia, de cima pra baixo (reverso)
        for i in (0..=day).rev() {
            // Adiciona "and" no último presente, exceto no primeiro dia
            if i == 0 && day != 0 {
                println!("and {}", gifts[i]);
            } else {
                println!("{}", gifts[i]);
            }
        }

        println!(); // espaço entre os dias
    }
}
