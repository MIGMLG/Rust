#[allow(unused_variables)] //Allows to create variables not used in any function
fn main() {
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    
    // Desta maneira o var transfere a propiedade para o valor param da função sendo que fica vazia, 
    // Isto leva a um erro de compilação
    //heap_procedure(heap_f64);
    //& para reference type, passar por referencia em vez de valor
    heap_procedure(&heap_f64);
    println!("In main heap {}", heap_f64);
}

//Como a var recebida está numa stack, pois tem tamanho fixo, quando é passada por argumento
//É realizada uma cópia do conteudo, ou seja, o conteudo original não será afetado como podemos ver
fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack_procedure with param {}", param);
}

//& para reference type, passar por referencia em vez de valor
fn heap_procedure(param: &Box<f64>){
    println!("In heap_procedure with param {}", param);
}