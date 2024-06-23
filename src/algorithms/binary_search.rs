// pub fn binary_search(list: Vec<u128>, target: u128) {
//     let mut left_pointer = 0;
//     let mut right_pointer = list.len(); 
//     while (right_pointer - left_pointer) > 1 {
//         let middle_pointer = (left_pointer + right_pointer) / 2;
//         if list[middle_pointer] == target {
//             println!("O número {} foi encontrado na posição {}", target, middle_pointer);
//             return;
//         } else if list[middle_pointer] < target {
//             left_pointer = middle_pointer;
//         } else {
//             right_pointer = middle_pointer;
//         }
//     }
// }

use std::io;

pub fn binary_search_impl(target: u128) {
    let list: Vec<u128> = (1..=1_000_000).collect();
    let mut left_pointer = 0;
    let mut index = 0;
    let mut right_pointer = list.len(); 
    while (right_pointer - left_pointer) > 1 {
        index += 1;
        let middle_pointer = (left_pointer + right_pointer) / 2;
        if list[middle_pointer] == target {
            println!("O número {} foi encontrado na posição {}", target, middle_pointer);
            println!("Número de iterações: {}", index);
            return;
        } else if list[middle_pointer] < target {
            left_pointer = middle_pointer;
        } else {
            right_pointer = middle_pointer;
        }
    }
}

pub fn binary_search() {
    let mut str = String::new();
    println!("Escreva seu número de busca binária(1...1.000.000).");
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let number: u128 = str
        .trim()
        .parse()
        .expect("Você não providenciou um número inteiro positivo.");
    binary_search_impl(number);
}