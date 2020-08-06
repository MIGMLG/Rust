fn main() {
    //shadow - concecpt of using the same name
    let y = 5;
    let y = y + 1; // using let to recreate the var, other y is cleaned from memory
    let y = y * 2;
    println!("The Value of y is : {}", y);

    //shadow and change type
    let abc = "ABC";
    let abc = abc.len();
    println!("The value of abc is : {}", abc);
}