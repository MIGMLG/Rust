fn main(){
    println!("Hello Rust!");
    let x = 6;
    let y = 9;
    println!("x is {}, y is {}", x ,y);
    println!("x is {valx}, y is {fred}", valx=x , fred=y); // passing values
    println!("Debug {:?}", (3,4)); //Debug show what is in variable
    println!("y is {1}, x is {0}", x, y);// using index on variables
}