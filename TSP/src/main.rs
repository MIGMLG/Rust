#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(while_true)]

use std::io::{stdin, stdout, Write};

const LIMITE_CIDADES: i8 = 32;
const LIMITE_ROTAS: i8 = 32;
const DISTANCIA_MAX_RANDOM: i16 = 32;

fn main() {
    execute();
}

fn execute() {
    clear_screen();
    println!("Trabalho realizado por Miguel Costa!");

    while true {
        println!("1- Comecar calculo de rota novo");
        println!("2- Sair do Pograma");
        print!("Escolha uma opcao valida: ");
        let input = read_input();
        match input.as_str() {
            "1" => ask_number_of_cities(),
            "2" => break,
            _ => println!("Opção Inválida!"),
        }
        //clear_screen();
    }

    println!("Goodbye!");
}

fn ask_number_of_cities() {
    clear_screen();
    let mut numeroMaxCidades: i8 = 0;
    while true {
        print!("Introduza o numeros de Cidades pretendidas: ");

        //Error Handling
        numeroMaxCidades = match read_input().parse::<i8>() {
            Ok(i) => i,
            Err(_e) => -1,
        };

        if numeroMaxCidades > LIMITE_CIDADES || numeroMaxCidades <= 2 {
            println!(
                "Valor Inválido. Não pode ser inferior ou igual a 2 ou superior a {}.",
                LIMITE_CIDADES
            );
            continue;
        }
        break;
    }
    ask_distance_options(numeroMaxCidades);
}

fn ask_distance_options(numeroMaxCidades: i8) {
    clear_screen();
    while true {
        println!("1- Distáncias introduzidas pelo utilizador");
        println!("2- Distáncias generadas aleatoriamente");
        print!("Escolha uma opção valida: ");
        let input = read_input();
        match input.as_str() {
            "1" => {
                println!(
                    "Chamar método que o utilizador define as distancias com {} cidades.",
                    numeroMaxCidades
                );
                break;
            }
            "2" => {
                println!(
                    "Chamar método que o computador define as distancias com {} cidades.",
                    numeroMaxCidades
                );
                break;
            }
            _ => println!("Opção Inválida!"),
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn read_input() -> String {
    let mut s = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}
