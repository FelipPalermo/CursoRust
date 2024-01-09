use std::fs::File;
use std::io::{self, Read};

fn main(){

    let result = lerArquivo("lendoarquivo1.rs");

    match result {
        Ok(conteudo) => {println!("Conteudo do arquivo : {}", conteudo); }
        Err(Error) => {println!("Erro : {}", Error)}
    }

}

// sempre que retornamos erro ele e o segundo parametro
// caso exista um antes

// Result e um tipo isolado que carrega dois valores
// sendo um indicado pelo programador e o outro um erro
fn lerArquivo(caminho : &str) -> Result<String, io::Error> {
    
    // ? serve para indicar que isso pode dar um erro 
    // open pode retornar um result que retorna erro ou nao 
    let mut arquivo : File = File::open(caminho)?;

    // criando um espaco reservado na memoria 
    // para uma string vazia
    let mut conteudo : String = String::new();

    // temos que passar &mut para nao passaar a propriedade para a nova variavel
    // & e usado para emprestar os valores de referencia
    // mut para indicar que vai ser algo que podemos alterar
    arquivo.read_to_string(&mut conteudo);

    // caso tudo de certo retornamos o conteudo
    // Caso de erro lendo o arquivo, ja retorna erro direto
    Ok(conteudo)
}