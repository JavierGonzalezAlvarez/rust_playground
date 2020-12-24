//another way to do the same
//enum_3
enum Monedas {
    Dolar,
    Euro,
}

// #[allow(dead_code)]  -> en caso de que no se use la funcion
fn monedas(moneda: Monedas) -> String {
 match moneda {
     Monedas::Dolar => String::from("Dolar"),
     Monedas::Euro => String::from("Euro"),
     }    
}

fn main(){    
    let salida1 = monedas(Monedas::Dolar);    
    let salida2 = monedas(Monedas::Euro);    
    println!("resultado {}", salida1);
    println!("resultado {}", salida2);
}

