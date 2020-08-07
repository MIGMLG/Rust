fn main() {
    let mut words = vec![String::from("Hello"),String::from("Yellow"),
    String::from("Tree"),String::from("Rust"),String::from("Compiler!")];

    println!("{:?}", words);

    //Borrowing não ia funcionar pois só se pode emprestar uma vez da estrutura toda
    let t = words[1].clone();
    words[1] = words[2].clone();//String não tem copy, por isso é necessário usar a função clone
    words[2] = t;
    //Podemos usar também words.swap(1,2);
    // ou std::mem::swap(&mut words1[0], &mut words2[0]); quando temos dois arrays separados
    println!("{:?}", words);
}