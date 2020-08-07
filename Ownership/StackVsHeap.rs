#[allow(unused_variables)] //Allows to create variables not used in any function
fn main() {
    //Stack
    // - Fast memory creation and retrieval... speed, Speed, SPEED!
    // - Memory is automatically recaptured by the program after variables go out of scope
    // - Is the default in Rust
    // - Fixed size variables... Collections CANNOT be stack based (and Strings are a collection of u8's)
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;// Ponto por causa de ser float
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    //Heap
    // - Flexibility
    // - Memory tha can grow in size (Vector, HashMap, String, etc)
    // - Runtime performance cost (speed);
    // - Memory that can live beyond the scope that created it
    // - Memory is automatically recaptured when the last OWNER goes out of scope
    let heap_vector: Vec<i8> = Vec::new();
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

    //Como são variáveis de tamanho fixo, são alocados na stack, 
    //sendo que é criado uma cópia da varíavel original quando a mesma é atribuida a outra var
    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);

    //Como estas vars são de Heap, pois o seu tamanho não é fixo, o proprietário passa a heap_i8_2, (transeferencia de propriedade)
    //sendo que a heap_i8 perde a propriedade dando erro em compilação
    let heap_i8_2 =  heap_i8.clone(); //heap_i8; Não se pode querer ler desta var pois ela perdeu a propriedade para a outra var
    //.clone() copia o conteudo para a nova Var, ficando com duas vars independente
    //.clone() é pesado computacionalmente
    println!("{}", heap_i8); // Aqui
    println!("{}", heap_i8_2);
}