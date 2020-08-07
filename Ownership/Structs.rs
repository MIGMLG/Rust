#[derive(Debug, Clone)]//É necessário indicar que tem o metodo clone, 
//Podemos AINDA USAR COPY para que a var se comporte como um primitivo, desde que nao existam var de Heap
struct MigStruct {
    a: i32,
    b: f64,
}
//Struct pode ter milhares de campos, por isso não pode ser assumida com Stack Var, mas sim como Heap var\
//Se fosse Stack var com milhares de campos, a passagem por param resultaria na copia desses dados todos, tornando se ineficiente

#[allow(unused_variables)]
fn main() {
    println!("Hello World!");
    
    let var_1 = MigStruct {a : 9, b : 10.};
    my_procedure(&var_1);// Podemos usar clone se quisermos criar uma copia
    println!("{:?}", var_1);
}

fn my_procedure(param: &MigStruct) {
    println!("{:?}", param);
}