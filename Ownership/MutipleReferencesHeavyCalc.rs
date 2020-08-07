#[allow(unused_variables)]
fn main() {
    let var_a = String::from("Howdy");
    let var_b = String::from("Partner");

    let mass_data: Vec<&String> = vec![&var_a, &var_b];

    println!("{}", heavy_cal(&mass_data));
}

fn heavy_cal(_param : &Vec<&String>) -> i64 {
    10
}