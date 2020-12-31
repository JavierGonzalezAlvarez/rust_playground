//doc => https://docs.serde.rs/serde_json/index.html

/*
importar serde, ahora usamos una macro llamada json
de un strn pasamos a un json
*/
//use serde_json::{json, Value};
use serde_json::{json};

fn main() {
    println!("Serde");
    println!("-----");
    /*
        un result me puede lanzar errores,
        indicar un unwrap() dado que espero que el codigo este ok
        unwrap() puede ser ok() o result()
    */
    untyped_json().unwrap();
}

//al colocar la marco ?, uso serde_json::Result<()>, traigo un dato, da igual el tipo
fn untyped_json() -> serde_json::Result<()> {
    /*
    creo variable data, 
    y dentro haby un string raw, se inicia con r#" lo que hay dentro es string "#
    */

    //usamos un macro llamada json -> !()
    /*
    este codigo tb works:
    let data: Value = json!(
    */
    let data = json!(
    {
        "name": "javier",
        "address": "street oxford nº 34",
        "age": 22,
        "phones": [
            "+34 21213232367",
            "+34 82837826476"
            ]        
    });    

    /*
    declaro una variable, de tipo "Value" que es un objeto de rust
    <Value> significa que retorna un Value (genérico)
    le pongo la macro ?, pero para que este habilitada es necesario
    que haga uso de un tipo de dato "serde_json"
    */
    let valor = serde_json::to_string(&data)?;
//    let valor: Value = serde_json::from_str::<Value>(data)?;

    println!("Json");
    println!("---------");
    //macro println
    println!("  valor = {}", valor);
    //macro con debug
    println!("  value = {:?}", valor);
    println!("  Llamar a  {} al teléfono {}", data["name"], data["phones"][1]);            
    //si está ok
    Ok(())        

}