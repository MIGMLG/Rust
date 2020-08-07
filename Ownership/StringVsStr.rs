#[allow(unused_variables)] //Allows to create variables not used in any function
fn main() {
    let mut some_string: String = String::from("Howdy"); //Strings are always on the heap
    let some_str: &str = "Partner"; //&str is pointer... to either stack or heap

    some_procedure(&mut some_string, &some_str);
    println!("{} {}", some_string, some_str);
}

//Só usar String se for para passar uma mutable String, senão usar sempre str slices
fn some_procedure(param_a: &mut String, param_b: &str ) {
    param_a.insert(5, '!');
    println!("{} {}", param_a, param_b);
}

// para alterar a var no procedimento, a mesma tem de ser mut
// o argumento enviado tem de levar mut
// e o argumento pedido tem de levar mut