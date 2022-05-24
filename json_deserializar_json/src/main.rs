/*
importar serde, traemos value
deserializacion => json a objeto/valor
*/
use serde_json::json;

fn main() {
    println!("Serde");
    println!("-----");
    /*
        un result me puede lanzar errores,
        indicar un unwrap() dado que espero que el codigo este ok
        unwrap() puede ser ok() o result()
    */
    untyped_json_str().unwrap();
}

//al colocar la marco ?, uso serde_json::Result<()>, traigo un dato, da igual el tipo
fn untyped_json_str() -> serde_json::Result<()> {
    /*
    creo variable data,     
    */
    let data = json!({
        "name": "javier",
        "address": "street oxford nº 34",
        "age": 22,
        "phones": [
            "+34 21213232367",
            "+34 82837826476"
            ]        
    });
    
    let valor = serde_json::to_string(&data)?;

    println!("json");
    println!("---------");
    //macro println
    println!("  valor = {}", valor);
    //macro con debug
    println!("  value = {:?}", valor);
    //valores deserializados
    println!("  Llamar a  {} al teléfono {}", data["name"], data["phones"][1]);
    
    //si está ok
    Ok(())        

}