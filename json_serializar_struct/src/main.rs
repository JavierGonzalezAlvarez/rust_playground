//doc => https://docs.serde.rs/serde_json/index.html

/*
importar serde, ahora usamos una macro llamada json
de un str pasamos a un json con structura, struct
*/
//use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;

/*
a esta estructura le añadimos que serialice y desserialize
 y añadimos debug para imprimir
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Datos {
    name: String,
    address: String,
    age: i8,    
    phones: [i64; 2], //array
    //phones: v[String], //tb podemos usar un vector de string
    //HashMap como posibiliad
}

fn main() {
    println!("Serde");
    println!("-----");
    /*
        un result me puede lanzar errores,
        indicar un unwrap() dado que espero que el codigo este ok
        unwrap() puede ser ok() o result()
    */
    untyped_json_struct().unwrap();
}

//al colocar la marco ?, uso serde_json::Result<()>, traigo un dato, da igual el tipo
fn untyped_json_struct() -> Result<()> {
    /*
    creo variable data, 
    y dentro haby un string raw, se inicia con r#" lo que hay dentro es string "#
    */
        
    let data = Datos {
        //le metemos to_owned() => which just allocates a buffer and copies the literal into the buffer
        name: "javier".to_owned(),
        address: "street oxford nº 34".to_owned(),
        age: 21.to_owned(),
        phones: [
            1213232367,
            2837826476
        ].to_owned()        
    };    

    let valor = serde_json::to_string(&data)?;

    println!("Json");
    println!("---------");
    //macro println
    println!("  valor = {}", valor);
    //macro con debug
    println!("  value = {:?}", valor);
    println!("  Llamar a  {} al teléfono {}", data.name, data.phones[1]);            
    
    //si está ok
    Ok(())        

}
