#![allow(while_true)]

#[path = "libs/utils.rs"]
mod utils;

pub fn execute(numero_max_cidades: &u8, matriz_distancias: &Vec<Vec<u16>>) {
    let tamanho_array: usize = (*numero_max_cidades + 1) as usize;
    let limite_destino_final: usize = *numero_max_cidades as usize;
    let mut cidade_inicial: i8 = -1;

    while true {
        print!("Introduza a cidade inical: ");
        cidade_inicial = match utils::read_input().parse::<i8>() {
            Ok(i) => i,
            Err(_e) => -1,
        };

        if cidade_inicial < 0 || cidade_inicial >= *numero_max_cidades as i8 {
            println!("Opção Inválida!");
            continue;
        }
        break;
    }

    let mut rota_menor = vec![0u8; tamanho_array];
    let mut rota_swap = vec![0u8; tamanho_array];

    for x in 0..limite_destino_final {
        rota_menor[x] = x as u8;
        rota_swap[x] = x as u8;
    }

    rota_menor[*numero_max_cidades as usize] = cidade_inicial as u8 ;
    rota_swap[*numero_max_cidades as usize] = cidade_inicial as u8;

    for i in (1..(cidade_inicial + 1)).rev() {
        rota_menor[i as usize] = rota_menor[(i - 1) as usize];
        rota_swap[i as usize] = rota_swap[(i - 1) as usize];
    }

    rota_menor[0] = cidade_inicial as u8;
    rota_swap[0] = cidade_inicial as u8;

    let mut distance_min: u16 = calculate_distance(&rota_menor, &matriz_distancias);

    for i in 1..limite_destino_final {
        for j in 1..limite_destino_final {
            let troca_valores_1 = rota_swap[i];
            let troca_valores_2 = rota_swap[j];
            rota_swap[j] = troca_valores_1;
            rota_swap[i] = troca_valores_2;

            let distance = calculate_distance(&rota_swap, &matriz_distancias);

            if distance < distance_min {
                rota_menor = rota_swap.clone();
                distance_min = distance;
            }
        }
    }

    println!("Rota Menor: {:?}\nDistancia : {} KM", rota_menor, distance_min);
    println!("------------------------------------------------------------------------");
}

fn calculate_distance(route: &Vec<u8>, matriz_distancias: &Vec<Vec<u16>>) -> u16 {
    let mut distance: u16 = 0;

    for i in 0..(route.len() - 1) {
        distance = distance + matriz_distancias[route[i] as usize][route[i + 1] as usize] as u16;
    }

    distance
}