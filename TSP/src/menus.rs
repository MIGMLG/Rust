#![allow(while_true)]
#![allow(unused_must_use)]

const LIMITE_CIDADES: u8 = 15;
const DISTANCIA_MAX_RANDOM: u16 = 100;

#[path = "libs/utils.rs"]
mod utils;
#[path = "tsp_solver.rs"]
mod tsp_solver;
use rand::Rng;

/// Main Menu
pub fn execute() {
    utils::clear_screen();
    println!("Trabalho realizado por Miguel Costa!");

    while true {
        println!("1- Comecar calculo de rota novo");
        println!("2- Sair do Pograma");
        print!("Escolha uma opcao valida: ");
        let input = utils::read_input();
        match input.as_str() {
            "1" => ask_number_of_cities(),
            "2" => break,
            _ => println!("Opção Inválida!"),
        }
        //clear_screen();
    }
    utils::clear_screen();
    println!("Goodbye!");
}

/// Menu to ask the number of cities to present to the problem
fn ask_number_of_cities() {
    utils::clear_screen();
    let mut numero_max_cidades: u8 = 0;
    while true {
        print!("Introduza o numeros de Cidades pretendidas: ");

        // Error Handling
        numero_max_cidades = match utils::read_input().parse::<u8>() {
            Ok(i) => i,
            Err(_e) => 0,
        };

        if numero_max_cidades > LIMITE_CIDADES || numero_max_cidades <= 2 {
            println!(
                "Valor Inválido. Não pode ser inferior ou igual a 2 ou superior a {}.",
                LIMITE_CIDADES
            );
            continue;
        }
        break;
    }
    ask_distance_options(&numero_max_cidades);
}

/// Menu to ask how should the distance be defined
fn ask_distance_options(numero_max_cidades: &u8) {
    utils::clear_screen();
    while true {
        println!("1- Distáncias introduzidas pelo utilizador");
        println!("2- Distáncias generadas aleatoriamente");
        print!("Escolha uma opção valida: ");
        let input = utils::read_input();
        match input.as_str() {
            "1" => {
                distance_defined_by_user(&numero_max_cidades);
                break;
            }
            "2" => {
                distance_defined_by_computer(&numero_max_cidades);
                break;
            }
            _ => println!("Opção Inválida!"),
        }
    }
}

fn distance_defined_by_user(numero_max_cidades: &u8) {
    utils::clear_screen();
    let size = *numero_max_cidades as usize;
    let mut matriz_distancias = vec![vec![0u16; size]; size];

    let mut h : usize = 0;
    for x in 0..size {
        for y in h..size {
            if x == y {
                continue;
            }
            print!("Introduza a distancia entre {} e {} : ", x, y);
            let input = match utils::read_input().parse::<u16>() {
                Ok(i) => i,
                Err(_e) => 0,
            };
            matriz_distancias[x][y] = input;
            matriz_distancias[y][x] = input;
        }
        h = h + 1;
    }

    println!("------------------------------------------------------------------------");

    for element in matriz_distancias.iter_mut() {
        println!("{:?}", element);
    }
    tsp_solver::execute(&numero_max_cidades, &matriz_distancias);
}

fn distance_defined_by_computer(numero_max_cidades: &u8) {
    utils::clear_screen();
    let mut rng = rand::thread_rng();
    let size = *numero_max_cidades as usize;
    let mut matriz_distancias = vec![vec![0u16; size]; size];

    let mut h : usize = 0;
    for x in 0..size {
        for y in h..size {
            if x == y {
                continue;
            }
            let input : u16 = rng.gen_range(1, DISTANCIA_MAX_RANDOM);
            matriz_distancias[x][y] = input;
            matriz_distancias[y][x] = input;
        }
        h = h + 1;
    }

    println!("------------------------------------------------------------------------");
    for element in matriz_distancias.iter_mut() {
        println!("{:?}", element);
    }
    println!("------------------------------------------------------------------------");

    tsp_solver::execute(&numero_max_cidades, &matriz_distancias);
}