fn main() {
    /*
    crear un vector con valores que serán i32
    los vectores han de ser del mismo valor, en el caso
    de que no lo sean hay que meterlos en un enum
    */
    
    //forma de crear vectores de diferentes valores
    // se añade esta macro
    #[derive(Debug)]
    enum Valores {
        Numero(i32),
        Texto(String),
        Float(f64),
        Array([i32; 2]),
    }
    
    let efila = vec![
        Valores::Numero(2),
        Valores::Texto(String::from("hola")),
        Valores::Float(5.3),
        Valores::Array([2,34]),
    ];
    
    /*
    it is not possible to derive Display because Display is aimed at displaying to humans and the compiler 
    annot automatically decide what is an appropriate style for that case. Debug is intended for programmers, 
    so an internals-exposing view can be automatically generated.
     */  
     
    println!("Crear y ver vectores con enum");
    let ver = Valores::Numero(2);
    println!("Es vector escogido es {:?}", ver);
    
    for i in &efila {
        println!("salida: {:?}", i);
    }
       
}

/* salida
Crear y ver vectores con enum
Es vector escogido es Numero(2)
salida: Numero(2)
salida: Texto("hola")
salida: Float(5.3)
salida: Array([2, 34])
*/