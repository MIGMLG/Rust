fn main() {
    // mutable borrow
    let mut s1 = String::from("Hello!");
    let s2 = &mut s1; // mutable borrow occurs here
    s2.push('!');
    println!("s2 = {}", s2);
    println!("s1 = {}", s1);
}