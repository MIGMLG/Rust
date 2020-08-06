fn main(){
    let a = 3 + 7;
    //statemente unitl semicolon, expression after equal
    let b = (3 + 7);
    let c = { 3 + 7};

    let y = {
        let mut x = 3;
        x = x * 2;
        //ther is no ; as it is as expression
        x + 1// como não tem ; isto vai ser o resultado de y
    };

    println!("The value of y is : {}", y);
}