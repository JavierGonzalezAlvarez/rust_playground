//enum_2
enum Monedas {
    Dolar,
    Euro,
}

// #[allow(dead_code)]  -> en casod e que no se use la funcion
fn monedas(moneda: Monedas) {
 match moneda {
     Monedas::Dolar => println!("la moneda es dolar"),
     Monedas::Euro => println!("la moneda es euro"),
     }    
}

fn main(){    
    let _salida = monedas(Monedas::Dolar);    
    let _salida = monedas(Monedas::Euro);    
}

