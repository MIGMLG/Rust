#![allow(while_true)]

const LIMITE_CIDADES: i8 = 32;
const LIMITE_ROTAS: i8 = 32;
const DISTANCIA_MAX_RANDOM: i16 = 32;

#[path = "libs/utils.rs"]
mod utils;
#[path = "tsp_solver.rs"]
mod tsp_solver;
use rand::Rng;

//Main Menu
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

    println!("Goodbye!");
}

//Menu to ask the number of cities to present to the problem
fn ask_number_of_cities() {
    utils::clear_screen();
    let mut numeroMaxCidades: i8 = 0;
    while true {
        print!("Introduza o numeros de Cidades pretendidas: ");

        //Error Handling
        numeroMaxCidades = match utils::read_input().parse::<i8>() {
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
    ask_distance_options(&numeroMaxCidades);
}

//Menu to ask how should the distance be defined
fn ask_distance_options(numeroMaxCidades: &i8) {
    utils::clear_screen();
    while true {
        println!("1- Distáncias introduzidas pelo utilizador");
        println!("2- Distáncias generadas aleatoriamente");
        print!("Escolha uma opção valida: ");
        let input = utils::read_input();
        match input.as_str() {
            "1" => {
                distance_defined_by_user(&numeroMaxCidades);
                break;
            }
            "2" => {
                distance_defined_by_computer(&numeroMaxCidades);
                break;
            }
            _ => println!("Opção Inválida!"),
        }
    }
}

fn distance_defined_by_user(numeroMaxCidades: &i8) {
    utils::clear_screen();
    let size = *numeroMaxCidades as usize;
    let mut matrizDistancias = vec![vec![0i16; size]; size];

    let mut h : usize = 0;
    for x in 0..size {
        for y in h..size {
            if x == y {
                continue;
            }
            print!("Introduza a distancia entre {} e {} : ", x, y);
            let input = match utils::read_input().parse::<i16>() {
                Ok(i) => i,
                Err(_e) => 0,
            };
            matrizDistancias[x][y] = input;
            matrizDistancias[y][x] = input;
        }
        h = h + 1;
    }

    println!("------------------------------------------------------------------------");

    for element in matrizDistancias.iter_mut() {
        println!("{:?}", element);
    }
    tsp_solver::execute(&numeroMaxCidades, &matrizDistancias);
}

fn distance_defined_by_computer(numeroMaxCidades: &i8) {
    utils::clear_screen();
    let mut rng = rand::thread_rng();
    let size = *numeroMaxCidades as usize;
    let mut matrizDistancias = vec![vec![0i16; size]; size];

    let mut h : usize = 0;
    for x in 0..size {
        for y in h..size {
            if x == y {
                continue;
            }
            let input : i16 = rng.gen_range(1, DISTANCIA_MAX_RANDOM);
            matrizDistancias[x][y] = input;
            matrizDistancias[y][x] = input;
        }
        h = h + 1;
    }

    println!("------------------------------------------------------------------------");

    for element in matrizDistancias.iter_mut() {
        println!("{:?}", element);
    }
    tsp_solver::execute(&numeroMaxCidades, &matrizDistancias);
}