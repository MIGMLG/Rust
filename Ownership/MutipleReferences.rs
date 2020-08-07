#[allow(unused_variables)] //Allows to create variables not used in any function
fn main() {
    let var_a = String::from("Howdy!");
    //É preciso garantir que var_a não é modificado, apos criar pointers, para que as referencias sejam atribuídas
    //Senão occorrer isso existe um erro de compilador
    let var_b = &var_a;//read only references
    let var_c = &var_a;//read only references

    println!("{} {} {}", var_a, var_b, var_c);

    //Após ficarem fora de scope é possível alterar os valores
    var_a.push('a');
}

//Rust tem de garantir que erros de memória não vão ocorrer
//Por isso inmut by default
//Usar borrowing para heavy calculations
//https://youtu.be/lQ7XF-6HYGc?t=1594