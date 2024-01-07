enum Itens {
    Espada, 
    Escudo,
    Arco
}

fn main() {

    enumeracao(Itens::Espada); 

}

fn enumeracao(Itens : Itens) { 

    match Itens {
        Itens::Espada => println!("Espada"),
        Itens::Escudo => println!("Escudo"),
        Itens::Arco => println!("Arco"),
    };

}