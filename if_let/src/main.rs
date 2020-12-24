
fn main(){

    //Some(numero + u8)
    let numero_u8_value = Some(3u8);

    if let Some(3) = numero_u8_value {
        println!("tres");
    }
    else {
        println!("otro número");
    };
        
}