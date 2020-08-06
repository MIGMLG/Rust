fn main(){
    //arrays same type
    let a = [1,2,3,4,5,6,7,8,9,10];
    let b: [u16; 5] = [1,2,3,4,5];// defines type and size
    let c = [0;5];//defines an array of size 5 filled with 0
    println!("a b c : {:?} {:?} {:?}", a, b, c);

    let months = ["January", "February", "March"];

    let first = a[0];
    let march = months[2];
    println!("Two elements of the arrays: {} {}", first, march);
}