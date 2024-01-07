fn main(){

    // Para declarar variavel usamos a palavra let 
    // mas para definir a mutabilidade precisamos definir isso
    // caso nao utilizemos mut nao temos como alterar a variavel
    let mut a = 0;
    let b = 0; 

    // propriedade unica : cada valor tem exatamente um proprietario 
    // string mutavel
    let string_Mutavel = String::from("String Mutavel");

    let string_Imutavel = "String Imutavel";

    println!("{}", string_Mutavel);
    println!("{}", string_Imutavel);

    // caso facamos isso : 
    let texto = string_Imutavel;
    // a variavel string_Imutavel deixa de existir e agora a variavel texto tem o valor dela
    println!("{}", texto);
    //println!(string_Imutavel);

    // para referenciarmos valor usamos como ponteiros 
    let mut valor = 50;
    let referencia = &valor;

    // por padrao a referencia nao pode fazer alteracao, a menos que seja explicito
    let referencia_mut = &mut valor;
    
}