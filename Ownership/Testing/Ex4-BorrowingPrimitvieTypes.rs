fn main() {
    let mut n: i32 = 19;
    plus_one_by_ref(&mut n);
    println!("n = {}", n);
}

fn plus_one_by_ref(n: &mut i32){
    *n = *n + 1;//Dereferencing the var to get to the primitive type
}