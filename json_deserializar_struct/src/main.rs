//doc => https://docs.serde.rs/serde_json/index.html

/*
deserializacion => json a datos

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
    untyped_json_struct().unwrap();
}

//al colocar la marco ?, uso serde_json::Result<()>, traigo un dato, da igual el tipo
fn untyped_json_struct() -> serde_json::Result<()> {
    #[macro_use]
extern crate serde_derive;

use serde_json::Result;

fn main() -> Result<()>  {
    let data = r#"
    [
      {
        "id": 1,
        "type": "personal",
        "details": {
          "firstName": "Juliano",
          "lastName": "Alves",
          "primaryAddress": 7777777
        }
      },
      {
        "id": 2,
        "type": "business",
        "details": {
          "name": "Juliano Business",
          "companyRole": "OWNER",
          "primaryAddress": 8888888
        }
      }
    ]
    "#;

    let profiles: Vec<Profile> = serde_json::from_str(data)?;
    println!("{:#?}", profiles);

    Ok(())
}

#[derive(Deserialize, Debug)]
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
enum Profile {
    Personal {
        id: i32,
        details: PersonalDetails,
    },
    Business {
        id: i32,
        details: BusinessDetails,
    },
}

}

