/*
importar serde, traemos value
serializacion => raw a json
*/
use serde_json::Value;

fn main() {
    println!("Serde");
    println!("-----");
    /*
        un result me puede lanzar errores,
        indicar un unwrap() dado que espero que el codigo este ok
        unwrap() puede ser ok() o result()
    */
    untyped_raw_str().unwrap();
}

//al colocar la marco ?, uso serde_json::Result<()>, traigo un dato, da igual el tipo
fn untyped_raw_str() -> serde_json::Result<()> {
    /*
    creo variable data,
    y dentro haby un string raw, se inicia con r#" lo que hay dentro es string "#
    */
    let data = r#"
    {
        "name": "javier",
        "address": "street oxford nº 34",
        "age": 22,
        "phones": [
            "+34 21213232367",
            "+34 82837826476"
            ]        
    }
    "#;

    /*
    declaro una variable, de tipo "Value" que es un objeto de rust
    <Value> significa que retorna un Value (genérico)
    le pongo la macro ?, pero para que este habilitada es necesario
    que haga uso de un tipo de dato "serde_json"
    */
    let valor: Value = serde_json::from_str::<Value>(data)?;

    println!("Str crudo");
    println!("---------");
    //macro println
    println!("  valor = {}", valor);
    //macro con debug
    println!("  value = {:?}", valor);
    println!(
        "  Llamar a  {} al teléfono {}",
        valor["name"], valor["phones"][1]
    );
    println!("Llamar a {} ", valor["name"].as_str().unwrap());

    //si está ok
    Ok(())
}
