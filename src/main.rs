use std::{io, time::Instant};

use algorithms::binary_search::binary_search;

use crate::algorithms::fatorial::fatorial;
pub mod algorithms;
fn main() {
    let arr = [fatorial, binary_search];
    let labels = ["Fatorial", "Binary Search"];
    println!("Escolha um algoritmo para rodar:");
    for (i, _f) in arr.iter().enumerate() {
        println!("{} - {}", i, labels[i]);
    }
    let mut str = String::new();
    io::stdin()
    .read_line(&mut str)
    .expect("Failed to read line");
    let number: u128 = str
    .trim()
    .parse()
    .expect("Você não providenciou um número válido");
    if number >= arr.len() as u128 {
        println!("Você não providenciou um número válido");
        return;
    }
    print!("{}[2J", 27 as char);
    run_function(arr[number as usize]);
}

fn run_function(func: fn() -> ()) {
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    println!("Tempo de execução: {:?}", duration);
    println!();
    println!("Sua função foi executada com sucesso!");
    println!();
    println!();
    println!("O que deseja fazer agora?");
    println!("1 - Rodar novamente");
    println!("2 - voltar ao menu");
    println!("3 - Sair");
    let mut str = String::new();
    io::stdin()
    .read_line(&mut str)
    .expect("Failed to read line");
    let number: u128 = str
    .trim()
    .parse()
    .expect("Você não providenciou um número válido");
    match number {
        1 => run_function(func),
        2 => main(),
        3 => return,
        _ => {
            println!("Você não providenciou um número válido");
            return;
        }
    }
}