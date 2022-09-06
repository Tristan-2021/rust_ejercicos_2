mod prueba_paso{

   pub struct Prueba {
         pub puntaje_final: i32,
         pub preguntas: i32,
         pub respues_correctas: i32,
        pub nombre : String, 
   } 
   impl Prueba {
    
    pub fn prueba_exceletne() -> Prueba {
        Prueba {
            puntaje_final: 20,
            preguntas: 20,
            respues_correctas: 20,
            nombre: String::from("Alter Ego"),
        }
    }

    pub fn prueba_regular(nombre: &str ) -> Prueba {
        Prueba {
            puntaje_final: 10,
            preguntas: 10,
            respues_correctas: 10,
            nombre: String::from(nombre),
        }
    }
   } 

  pub mod print_resultados{
    fn bienvenido(){
        println!("Bienvenido a los resultados de la prueba");
    }

   pub   fn dart_resultados()  {
        bienvenido();
    
    let prueba_excelente = super::Prueba::prueba_exceletne();
    let prueba_regular:super::Prueba   = super::Prueba::prueba_regular(  "Jhon Rodrìguez"); 

    show_resultados(prueba_excelente);
    show_resultados(prueba_regular);

    }

     fn show_resultados(prueba:super:: Prueba){
        println!("El puntaje final de {} es {}", prueba.nombre, prueba.puntaje_final);
    }
  }  

}


pub fn prueba_ejemplo() {
    crate::prueba::prueba_paso::print_resultados::dart_resultados();
}

