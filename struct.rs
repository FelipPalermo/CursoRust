fn main(){

    let mut lucalvo : Mamairo = Mamairo {
        Nome : "Lucas".to_string(),
        Altura : 1.20 
    };

    lucalvo.ScaleArmor("Drift de monza");    
    lucalvo.crescer(0.5);
    println!("{}", lucalvo.Altura);

}

struct Mamairo{
    Nome : String,
    Altura :  f32
}

impl Mamairo {
    fn ScaleArmor(&self,var: &str) {
        println!("{}", var);
    }
    fn crescer(&mut self, var : f32) {
        self.Altura += var; 
    }
}