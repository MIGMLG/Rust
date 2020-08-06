fn main() {
    let x = five();
    println!("The value of x is : {}", x);

    let x = plus_one(x);
    println!("The vlaue of x is: {}", x);
}


//expressions are what is returned
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}