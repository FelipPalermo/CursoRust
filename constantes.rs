fn main(){

    // diferente de variaveis constantes nao mudam
    // variavel sem mut no rust 
    // variaveis devem ser usadas com snake case

    // string imutavel 
    let nome : &str = "felipe";

    // string mutavel 
    let mut nome2 : &str = "felipe"; 

    println!("{}", nome);
    println!("{}", nome2);

    nome2 = "Alastor Mood";
    println!("{}", nome2);

    // constante --------

    // CONSTANTES DEVEM SER DECLARADAS TODAS EM MAIUSCULAS
    // CONSTANTES PRECISAM TER TIPO DECLARADO
    const NAME_COMPANY : &str = "HCODE";

    println!("{}", NAME_COMPANY);

    // na compilacao as constantes sao lidas e escritas como o proprio valor
    // enquanto as variaveis sao lidas como na memoria ram
}