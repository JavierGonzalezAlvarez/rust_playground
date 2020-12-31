//doc = https://serde.rs/

#[macro_use]
extern crate serde_derive;

use serde_json::{Result, Value};

#[derive(Deserialize, Debug)]
//los campos deben tener formato camelCase;
#[serde(rename_all = "camelCase")]
struct PersonalDetails {
    first_name: String,
    last_name: String,
    primary_address: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BusinessDetails {
    name: String,
    company_role: String,
    primary_address: i32
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
          "firstName": "Juliano",
          "lastName": "Alves",
          "primaryAddress": 7777777
        }
      },
      {
        "id": 2,
        "type": "datosBusiness",
        "details": {
          "name": "Juliano Business",
          "companyRole": "OWNER",
          "primaryAddress": 8888888
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



