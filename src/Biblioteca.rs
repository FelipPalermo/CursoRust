fn main(){
    let mut lib1 : Biblioteca = Biblioteca {
        livros: Vec::new(),
        id: Vec::new(),
    };

    let a : &str = "Felipe"; 

    lib1.ad_livro(["a arte da guerra", "Sapiens", "Um amor anarquista"].to_vec());
    lib1.mostrar_livros();
    lib1.procurar_por_id(1);

    println!("{}", type_off(a));
}
struct Biblioteca{ 
    id : Vec<u16>,
    livros : Vec<String> 
}

impl Biblioteca {
   fn ad_livro(&mut self, add_livro : Vec<&str>) {
    let mut id_livro : u16 = 0; 

    for livro  in add_livro {
        self.livros.push(livro.to_string());

        self.id.push(id_livro);
        id_livro += 1; 
        } 
    } 

   fn mostrar_livros(&self) {
    let mut iter : usize = 0;
    println!("Mostrando livros : ");

    for livro in &self.livros {
        print!("ID : {} ", self.id[iter]);
        print!(" | Livro : {}\n", livro);
        iter += 1;
        }
    }

    fn procurar_por_id(&self, id : usize) {
        println!("\nID : {} | Livro : {}", id, self.livros[id])
   }
}

fn type_off<Generico>(_var : Generico ) -> &'static str {
    return std::any::type_name::<Generico>();
}