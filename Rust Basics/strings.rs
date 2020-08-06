fn main(){
    //strings are utf-8
    //str - STATIC string literal, is not a struct
    let howdy = "Howdy👍"; 
    println!("Some str stuff : {} {}", howdy.len(), howdy.is_empty());
    println!("The bytes of howdy: {:?}", howdy.as_bytes());

    //String struct, can be manipulated
    let mut hello = String::from("Hello ");
    hello.push('R');// push Char
    hello.push_str("ust!");
    println!("String : {}", hello);
    hello.insert(5, ',');
    println!("String : {}", hello);
}