fn main(){
    //tuples, can have many different types, fixed length, can be used to return more than one var from a function
    let tup : (i32, f64, u8, f32) = (500, 6.4, 1, 29.29);
    let tup2 = (1500, 4.3);
    println!("tup and tup2: {:?} {:?}", tup, tup2);

    //get individuals vars from tuples
    let(w, x, y, z) = tup;
    println!("The values of w,x,y,z is : {} {} {} {}", w, x, y, z);

    let five_hundred = tup.0;
    let three_point_four = tup2.1;
    let one = tup.2;
    let twonine_29 = tup.3;
    println!("From the tuples: {} {} {} {}", five_hundred, three_point_four, one, twonine_29);
}