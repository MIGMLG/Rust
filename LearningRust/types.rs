fn main(){
    //signed int i8, i16, i32, i64, i128, for negatives to positives intervals
    //unsigned switch i to u, only positive
    let x128: u128 = 0xFAFBFCFD_FEF1F2F3_F4F5F6F7_F8F9FAFB;
    let x64 : i64 = 123456;
    //32 bit and 64 bit floating point numbers
    let x = 2.0; //f64 is the default
    let y : f32 = 3.0;//f32
    println!("The value of x128, x, y is: {} {} {}", x128, x, y)  ;

    let c = 'z';//plicas para char
    let thumb = '👍';
    let is_job_done : bool = false;
    println!("Some chars : {} {}", c, thumb);
}