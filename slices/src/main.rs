fn main() {
    let s = String::from("Hello a todos");
    let hello = &s[0..8];   //pasamos por referencia
    println!("Valor: {}", hello);
}
