fn main() {
    /*
    un String es un vector => Vec<v8>
    */
    
    //forma1 de crear un string
    let s = "hola mundo1";
    let string1 = s.to_string();
    println!("{}", string1);
    
    //forma2 de crear un string
    let string2 = String::from("hola mundo2");
    println!("{}", string2);
    
    //añadir un string
    let mut string3 = String::from("hola mundo3");
    string3.push_str(" str");
    println!("{}", string3);
    
    //añadir un string como vector. Solo se puede añadir un char
    let mut string4 = String::from("hola mundo");
    string4.push('4');
    println!("{}", string4);
    
    //uso de + para concatenar
    let string5 = String::from("hola");
    let string6 = String::from(" mundo7");
    let string7 = string5 + &string6;
    println!("{}", string7);
    /*
    + usa la funcion add definida por rust.
    fn add(self, s: &str) -> String {}
    &str hace referencia al segundo valor que 
    es pasado por parametro
    el primer valor de "add" es self y este no tiene &
    */
    
    //imprimir por consola un solo vector de un String
    let string8 = String::from("javier");
    // println!("{}", string8[0]);  //no funcnionará ya que no es indexable por integer
    let ver1 = string8.chars();
    println!("{:?}", ver1);
    //imprimir los bytes en memoria
    let bytes = string8.bytes();
    println!("{:?}", bytes);
    
    //enumerar chars
    fn takes_str(s: &str) {
        let ver3 = s;
        let ver4 = ver3.chars();
        let ver5 = ver3.chars().enumerate();
        let ver6 = ver3.chars().enumerate();
        
        for i in ver4 {
            println!("{:?}", &i);    
        }
        
        println!("salida de valores");
        println!("----------------------------");
        for (j, z) in ver5 {
            match (j, z) {
                (0, 'j') => println!("posicion: {:?}", &z),
                _ => println!("posicion: {} - {}", &j, &z),
            }
        }        

        println!("salida de un valor");
        println!("----------------------------");
        for (j, z) in ver6 {
            if j == 0 && z =='j' {
                println!("posicion: {:?}", &z);
            }            
        }        
        
    }
    takes_str(&string8);
    
}
