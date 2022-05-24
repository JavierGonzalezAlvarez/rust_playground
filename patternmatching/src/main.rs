fn main() {   
    let variable_name = String::from("Javier");
    println!("caracter en posicion 3 es {}", 
        match variable_name.chars().nth(3){
            Some(v) => v.to_string(),
            None => ": no hay caracter esta posicion".to_string(),
        });
}
