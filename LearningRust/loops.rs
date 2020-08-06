fn main(){
    let mut counter = 0;

    //loop that evaluates to an expression
    let result = loop {//loop construtor, keyword para loop
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // normal loop
    counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -=1;
    }
}