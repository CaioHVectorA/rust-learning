use std::io;
pub fn fatorial() {
    let mut str = String::new();
    println!("Escreva seu número de fatorial.");
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let number: u128 = str
        .trim()
        .parse()
        .expect("Você não providenciou um número inteiro positivo.");
    let fatorial_number = fatorial_calc(number);
    println!("{}", fatorial_number);
}

fn fatorial_calc(int: u128) -> u128 {
    if int <= 1 {
        return 1;
    }
    return int * fatorial_calc(int - 1);
}
