//doc = https://serde.rs/

#[macro_use]
extern crate serde_derive;

use serde_json::{Result, Value};

#[derive(Deserialize, Debug)]
//los campos deben tener formato camelCase;
#[serde(rename_all = "camelCase")]
struct PersonalDetails {
    name: String,
    surname: String,
    address: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BusinessDetails {
    name: String,
    post: String,
    address: String,
    phone: i32
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum EnumEstructura {
    DatosPersonales {
        id: i32,
        details: PersonalDetails,
    },
    DatosBusiness {
        id: i32,
        details: BusinessDetails,
    },
}

fn untyped_json_data() -> Result<()>  {
    //json para convertir a objeto/valores
    let data = r#"
    [
      {
        "id": 1,
        "type": "datosPersonales",
        "details": {
          "name": "Javier",
          "surname": "gonzalez",
          "address": "oxford street",
          "phone": 329382937
        }
      },
      {
        "id": 2,
        "type": "datosBusiness",
        "details": {
          "name": "javier",
          "post": "software",
          "address": "oxford steeet",
          "phone": 489384672
        }
      }
    ]
    "#;

    //estructura enum a una variable
    let estructura_json: Vec<EnumEstructura> = serde_json::from_str(data)?;
    println!("{:#?}", estructura_json);
    
    //valores a una variable
    let valor: Value = serde_json::from_str::<Value>(data)?;
    println!("---------");
    println!("json");
    println!("---------");    
    println!("  json = {}", valor);
    println!("---------");
    //macro con debug
    println!("  json objeto = {:?}", valor);
    println!("---------");

    Ok(())
}

fn main()  {
  untyped_json_data().unwrap();
}



