fn main(){
    let a : i8 = 30;
    println!("{}", inc(a));
    println!("{}", a);

    print!("{}", if_dec(2));
}

fn inc(mut var : i8) -> i8{
    var += 1; 
    return var;
}

fn if_dec(var : i8) -> String {

    // nao usar ; quando estiver definindo variavel em if
    let string : String = if var % 2 == 0 {
        format!("O numero {}, e par", var)
    } else {
        format!("O numero {}, e impar", var)
    };

    return string; 
}