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
  address: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BusinessDetails {
  name: String,
  post: String,
  address: String,
  phone: i32,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum EnumEstructura {
  DatosPersonales { id: i32, details: PersonalDetails },
  DatosBusiness { id: i32, details: BusinessDetails },
}

fn untyped_json_data() -> Result<()> {
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

  //----------------------------------------------
  // otra forma de sacar sin comillas el json
  #[derive(Debug, Deserialize)]
  struct Contact {
    name: String,
    address: String,
    age: u8,
    phones: Vec<String>,
  }

  let data = r#"
  {
      "name": "john",
      "address": "street 34",
      "age": 22,
      "phones": [
          "+34 21213232367",
          "+34 82837826476"
      ]
  }
  "#;

  let datajson: Contact = serde_json::from_str(data).unwrap();
  println!("name: {} ", datajson.name);
  println!("age: {} ", datajson.age);
  //----------------------------------------------
  //----------------------------------------------

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

fn main() {
  untyped_json_data().unwrap();
}
