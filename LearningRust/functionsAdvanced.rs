fn main() {
    let oe5 = is_odd(5);
    let oe6 = is_odd(6);

    println!("Is 5 odd ? 6 ?: {} {}", oe5, oe6);

    let td: (f32, f64) = tuple_and_cast((4,6,3));
    println!("Tuple demo: {:?}", td);

    println!("Factorial: {}", factorial(5));
}

fn is_odd(x: i32) -> bool {
    //bitwise binary operation that return the last number of binary
    if(x & 1) == 0 { //se o ultimo numero em binário for 0 então é par, se for 1 é impar
        false
    } else {
        true
    }
}

fn tuple_and_cast (t : (u8, u16, u32)) -> (f32, f64) {
    let x:f32 = t.0 as f32 + t.1 as f32;
    let y:f64 = t.2 as f64;
    (x, y)
}

fn factorial(num : u64) -> u64 {
    match num{ // equal to case statement
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}