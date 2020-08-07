fn main() {
    let mut s = String::from("Please add a dot");
    append_dot(&mut s);
    println!("s with dot = {}", s);
}

fn append_dot(t : &mut String) {
    t.push('.');
}