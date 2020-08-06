fn main(){

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() { // for each that iterates over the array
        println!("The value is ; {}", element);
    }

    //normal for that iterates from 3 to 0, doesnt include 4
    for number in (0..4).rev() { //.rev for in reverse 
        println!("{}", number);
    }
    
    //normal for that iterates from 0 to 3, doesnt include 4
    for number in 0..4 { //.rev for in reverse 
        println!("{}", number);
    }
}