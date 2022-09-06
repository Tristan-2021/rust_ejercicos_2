#![allow(unused)]

use rand::Rng;
use core::num;
use std::{io, vec};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std:: fs:: File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

mod restaurant;
mod prueba;
use crate::restaurant::order_food;
use crate::prueba::prueba_ejemplo;

 

fn main() {
    //println!("Whats is your name?");

//   let  lista = [1,4,5,3,21,4,43];

//    let mut newlist:Vec<i32> =Vec::new(); 
//    for d in lista{
     
//     if d < 6 {
//         newlist.push(d);
//         println!(" Elementos de la NewLista {:?}" ,newlist);
//     }
//     println!(" Elementos de la lista D {}" ,d);

//   } 
      

//   let mut name: String = String::new();
  
//   io::stdin().read_line(&mut name).expect("dign´t Receive inputc");
  
//   let greeting = "Nice to meet you ";

    
//   println!(
//   "el valor del saludo :{}! {}", name.trim_end(), greeting );


//   const ONE_MILL : u32 =  1_000_000;
//   let age :&str = "432";
//   let mut age: u32 = age.trim().parse().expect("Not a number");
//   age = age+1;

//   println!(
//     " {}  {}  ", ONE_MILL,  age); 



// println!("Max u32  {} ", u32::MAX );
// println!("Max u64  {} ", u64::MAX );
// println!("Max usize  {} ", usize::MAX );
// println!("Max u128  {} ", u128::MAX );
// println!("Max f32  {} ", f32::MAX );
// println!("Max f64  {} ", f64::MAX );
 
//  let float_number:f32 = 1.11111111111;
//  let float_number1:f64 = 1.11111111111;

//  println!("Float f32 {}", float_number + 0.0000000000 );
//  println!("Float f64 {}", float_number1 + 0.0000000000);
 

// let mut num_3  :u32 = 5;
// let num_4 :u32 = 4;

// println!(
//     "4 +5 {}", num_3 + num_4);

//     num_3 += 1;

//     println!("resultaod {}", num_3);

//   let my_age:i32 = 18; 

//   let can_vote = if my_age >=  18 {
//       true
//   } else {
//       false
//   }; 

//   println!("CAn vote ?  {}", can_vote );
  
//Todo: emaprejador usamos aqui en el match
 
//  let age2 :i32 = 81; 
    
//  match age2 {
//      1..=18 => println!("Child"), 
//      21 | 50   => println!("Adult"),  
//      65 ..=i32 ::MAX => println!("Senior"), 
//      _ => println!("Invalid age")
//  } 

// let my_age :i32 = 18;
// let voting_age:i32 = 18; 
// match my_age.cmp(&voting_age) {
//     Ordering::Less => println!("You are too young to vote"),
//     Ordering::Greater => println!("You are too young to vote"),
//     Ordering::Equal => println!("Your gained thee right to vote"),



// };
  
 //Todo: Matrices con loops
  


 let arr_2 = [1,2,3,4,5,6,7,8,9 ]; 

  let mut loop_index = 0; 

//   loop {
   
//     if arr_2[loop_index] % 2 == 0 {
//         println!(" what result does it show :{} ",  loop_index ); 
//         loop_index += 1; 

//         continue;
//     }  
//     if arr_2[loop_index] == 9 {
//         break;
//     } 

//     println!(" Val impares :{} ",  arr_2[loop_index] );
//     loop_index += 1; 

//   }

// while loop_index < arr_2.len() {
//    println!("arr: {}", arr_2[loop_index]); 
   
//    loop_index += 1;

   
// }


// for x in arr_2.iter() {
//     println!("For: {}", x);
// }

//todo: tuples

// let my_tuple : (u8, String,f64) = (18, "Juan".to_string(), 50_0000.00);
  
// println!("MY tuple  {}", my_tuple.0 ); 

// let (my_age, my_name, my_salary) = my_tuple;
//   println!(
// "my_age: {}  my_name: {} my_salary: {}", my_age, my_name, my_salary); 

 //Todo: Strings
// let mut  string = String::from(""); 
//   string.push('V'); 
//   string.push_str("Balores");
 

//   println!("String :{}", string);

// let st3:String = String::from("F S F G H J K L C D V "); 
 
// let mut v1  :Vec<char> = st3.chars().collect(); 
// v1.sort();
// v1.dedup();
 

// println!("V1 {:?}  ", v1);

// let str4 :&str =  "valores";

// let mut st5  = str4.to_string();
// println!("st5 {}  ", st5); 

// let byte_arr1 = &st5[0..4];
//  println!("byte_arr1 {}  ", byte_arr1);

//Todo: CAsting

// let int_u8 = 5; 
// let int2_u8 = 6; 
// let int_u8_3:u32 =  (int2_u8 as u32 ) + (int_u8 as u32);
// println!("int_u8_3: {}", int_u8_3);

//Todo: Enum 

// enum Days {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday


// }

//  impl Days {
//     fn is_weekend(&self) -> bool {
//         match self {
//             Days::Saturday | Days::Sunday => true,
//             _ => false
//         }
//     }

//     fn is_weekend1(&self) -> bool {
//         match self {
//             Days::Tuesday | Days::Wednesday => true,
//             _ => false
//         }
//     }
//  }

//  let today :Days = Days::Monday;
//  let today1 :Days = Days::Tuesday;


//  match  today {
//         Days::Monday => println!("Everyone Hates Mondays"),
//         Days::Tuesday => println!("Today is Tuesday"),
//         Days::Wednesday => println!("Is Wednesday, i dont Work"),
//         Days::Thursday => println!("Today is Thursday"),
//         Days::Friday => println!("Today is Friday"),
//         Days::Saturday => println!("Weekend"),
//         Days::Sunday => println!("Weekend"),
//  }

//  println!("Is todat the weekend? {}", today.is_weekend());
//  println!("Is today Wednesday or Tuesday? {}", today1.is_weekend1());


//Todo: Vectores
// let  vec1:Vec<i32> = Vec::new(); 
// let mut vec2 = vec![1,2,3,4,5];
// vec2.push(6);
// println!("vec2 {:?}", vec2[0]);


// let second:&i32 = &vec2[0];

// match vec2.get(4) {
//     Some(x) => println!("Varlo de la referencia {}", x),
//     None => println!("None"),
// }
  
// for i in &mut  vec2 {
//     *i *= 2;
 
// } 
 
   
// for i in &mut  vec2 {
  
// println!("index {:?}", i);


  
// }
// println!("lenght Vec2 {:?}", vec2.len());
// println!("lenght Vec2 prop {:?}", vec2.pop());


// println!("vec2 {:?}", vec2);
//Todo: functions

//let (va1, va2) = get_2(2) 

//let num_list = vec![1,2,3,4,5];
 // fn get_2 (x: &[i32] ) ->i32  {
 
//     let mut sum = 0;
//     for &i in x {
//         sum += i;
//     }
//     sum
    
//}
 

// fn get_2 (x:i32) -> (i32,i32){

//     (x +1, x -2)
// } 
//println!("Vec {:?} ", get_2(&num_list) );

// Todo:Genericos
// fn get_sum_gen<T:Add<Output = T>>(x:T, y:T) -> T {
//     return x + y
// }


// println!("5 + 4 =  {}", get_sum_gen(5,4));
// println!("5.2 + 4.6 =  {}", get_sum_gen(5.2,4.6));
//Todo : propiedad la pila y el montòn
//Todo: la pila almacena valores en un formato de entrada  y salida del 
//Todo: ùltimo en entrar primero en salir y deben de tener un tamaño fijo
//Todo: ahora el montòn, solicita cierta cantidad de memoria de espacio
//Todo: y luego el sistema encontrarà espacio que está disponible para almacenar
//Todo: y luego devolverá una direcciòn para ese espacio y esa referencia 
//Todo: al espacio en la memoria se llamará puntero es como una direcciòn.
//Todo: REglas para el manejo de la memoria 

//Todo: cada variable tendrá un propietario y siempre que el propietario	
//Todo: sale del alcance el valor desaparecerá. el compilador dirà si ese valor
//Todo: ya no se usa el compilador lo eliminará y esa memoria se liberará.
//Todo: el otro asunto es caundo es trata aquellode manera automatica la memoria. 
//Todo: la ubicaciòn de recurso es excelente y puede ocurrir problemas 


let stri1 = String::from("Hola");

// con la propiedad clone se puede clonar una variable o hacer copias de una variable
// el método clone se aplica a string, matrices, vectores, boleanos, caracteres flotantes
// tuplas  etc.
// let str2 = stri1.clone();
// let str3 = print_return_sttr(stri1);
// let mut str4 = print_return_sttr(str3.clone());

 
// println!("str3: {}", str3);
// change_string(&mut str4) 

// fn pront_stream(x: String) {
//   println!(" A string {}", x);
// }
// fn print_return_sttr(x:String) -> String {
//  println!("Un string: {}", x);
// x
// }

// fn change_string(name:&mut String )  {

// name.push_str("is happy");
// println!("Un Menssage: {}", name);

// }

//Todo: HashMap

// let mut herues = HashMap::new();

// herues.insert("Parce", 2013);
// herues.insert("Mora", 2014);
// herues.insert("Villa", 2016);
// herues.insert("Concha", 2016);
// herues.insert("Mocha", 2017);
// herues.insert("Ches", 2022);



// for (key, value) in &herues {
//     println!("Key: {}  : {}", key, value);


//   }

//  println!("Longitud del MApa : {}", herues.len() );

//  if herues.contains_key("Parce") {
//      println!("Parce esta en el mapa");
//  }
//  else {
//      println!("Parce no esta en el mapa");
//  }
 
//  if herues.contains_key("Parce") {

//   let the_parce  = herues.get(&"Parce"); 

//   match the_parce {
//       Some(x) => 
//       println!("Sì llegas aquì, aùn no te olvidas de aquello {}", x),
//       None => println!("Parce no esta en el mapa"),
//   }
// } 
// let carga =  herues.get_key_value("Ches"); 

// println!("Ches: {:?}", carga);
 

//Todo: Struct  

// struct Customer {
//     name: String,
//     age: u8,
// }

// let mut bob = Customer {
//     name: String::from("Bob"),
//     age: 22, }; 

//  bob.age = 32; 
 
//  println!("Bob  :{}", bob.age);
 
// const PI :f32 = 3.141592;
// struct Rectangle<T, U> {
//     width: T,
//     height: U,
// }

// let rect = Rectangle { width: 30, height: 50 }; 

// trait Shape {
//     fn area(lenght:f32, width:f32) -> Self;
//     fn perimeter(&self) -> f32;
//     fn metodo(width:f32) -> f32;

// }

// struct Rectangule1 {
//    length: f32,
//    width: f32,
//  };
//  struct Circle {
//     length: f32,
//     width: f32,
//   };
 

//  impl Shape for Rectangule1 {
//     fn area(length:f32, width:f32) -> Rectangule1 {
//         return Rectangule1{ length: length,  width };
//     }

//     fn perimeter(&self) -> f32 {
//         return self.length * self.width;
//     }
//     fn metodo( width:f32 ) -> f32 {
//         return width;
//       }
// }
// impl Shape for Circle {
//     fn area(length:f32, width:f32) -> Circle {
//         return Circle{ length: length,  width };
//     }
//     fn perimeter(&self) -> f32 {
//         return (self.width /2.0).powf(2.0) * PI;
//     }
//   fn metodo( width:f32 ) -> f32 {
//     return width;
//   }
// }

// let rectangulo:Rectangule1 = Shape::area(10.0, 10.0);
// let circle:Circle = Shape::area(10.0, 10.0);
 

// println!("Rentangule Are: {}", rectangulo.perimeter() );
// println!("Circle Are: {}", circle.perimeter() );


//Todo: Modules

//order_food();
//prueba_ejemplo();

 //Todo:Manejo de excepciones 
 
//  let path:&str = "lines.text";
//  // creamos la ruta del archivo con el path
//  let output  = File::create(path);
//  // manejamos la excepciòn con la coincidencia de patrones
//  let mut output = match output {
//   Ok(file)=> file,
//   Err(error)=> {
//    panic!("problmen creating file: {:?}", error);
//   }
//  };
//   // escribimos en el archivo con su respectivo manejador de excepciones
//  write!(output, "Hey, ho hay problemas para escribir en el archivo")
//  .expect("Failed to write to file")
//  ; 
//   // abrimos el archivo y expobnes su contenid con el unwrap
//  let input = File::open(path).unwrap();
//  // leemos el archuivo 
//     let buffered =  BufReader::new(input);

//  //  println!(" buffered {:?}", buffered.lines()  );
  
//  // lo iteramos y lo mostramos en pantalla el contenido
//   for line in buffered.lines() { 
//     println!(" Unwreap {}", line.unwrap() );
//   }


  //Todo: Manejanos los errores con màs detalles
// let otuput2 = File::create( "rride.text");

// let valor = match otuput2 {
//     Ok(file) => file,
//     // mostramos el error con el tipo de error
//     Err(e) => match e.kind() {
//         ErrorKind::NotFound => match File::create("rride.text") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {:?}", e),
//         },
//         other_error => {
//             panic!("Problem opening the file: {:?}", other_error)
//         }
//     },
// };

//Todo: manejo de iteradores 

// let mut arr_it = [1,2,3,4,5,6,7,8,9,10]; 

// for  i in arr_it.iter(){
//   println!(" imprimir Elemento: {}" , i ); 
// }



// let mut iter1 = arr_it.iter();

// println!("iter1 {:?}, {:?}", iter1.next(), iter1.next() );


//Todo: funciones sin nombres  o Clousre 

let can_vote  = |age:i32 | {

  age >= 18
};

println!("can_vote {}", can_vote(12) );
}
 
