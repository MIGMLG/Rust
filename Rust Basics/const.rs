fn main(){
    //const are not the same as immutable
    //consts are always mutable
    //cannot do shadow
    const FHD_WIDTH : u32 = 1920;
    const BAD_PI : f32 = 22.0/7.0;

    println!("Full HD width is {}, Bad PI is {}", FHD_WIDTH, BAD_PI);
}