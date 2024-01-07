// Ler sobre em Explicacoes/Exp_Type_Off
// Funcao para retornar tipo
fn type_of<T>(_var : T) -> &'static str {
    return std::any::type_name::<T>();
}

fn main() {
    
    // Cast : int para float 
    let num : i8 = 10;
    let int_to_float : f32 = num as f32;
    println!("{}", int_to_float);
    
    // Cast : float para int
    let float : f32 = 2.5;
    let float_to_int : i8 = float as i8;
    println!("{}", float_to_int);
    println!("{}", type_of(float_to_int));
    
    // Cast : Int para string 
    let inteiro : i8 = 32;
    let _string : String = inteiro.to_string();
    
    // Motivo de usar & (referencia) :
    // 12:40, aula 14, curso avancado de rust
    println!("{}", type_of(&_string));

    // Cast : String para int
    let string : &str = "42";
    let string_to_int : i8 = string.parse::<i8>().unwrap();
    println!("{}", type_of(string_to_int))

}