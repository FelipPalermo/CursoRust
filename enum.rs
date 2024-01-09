fn main() {
    // Exemplo de uso com a enumeração Itens
    enumeracao(Itens::Espada);

    // Exemplo de uso com a enumeração Inv
    enum2(Inv::Ouro(50));

}

enum Itens {
    Espada,
    Escudo,
    Arco,
}

enum Inv {
    Ouro(i32),
    Espadas(i8),
    Arcos(i8),
}

// Função que aceita valores da enumeração Itens
fn enumeracao(var: Itens) {
    match var {
        Itens::Espada => println!("Espada"),
        Itens::Escudo => println!("Escudo"),
        Itens::Arco => println!("Arco"),
    }
}

fn enum2(var: Inv) {
    match var {
        Inv::Ouro(valor) => println!("Você tem {} de ouro", valor),
        Inv::Espadas(valor) => println!("Você tem {} espadas", valor),
        Inv::Arcos(valor) => println!("Você tem {} arcos", valor),
    }
}


