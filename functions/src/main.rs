fn main() {
    suma1();
    suma2();
    suma3(4);

    let c = retorno_forma1(1,2);   //envio dos valores por parámetros
    println!("resultado1 {}", c);

    let c = retorno_forma2(1);   //envio dos valores por parámetros
    println!("resultado2 {}", c);
}

fn suma1() {
    println!("funcion suma1");
}

fn suma2() {
     let a = 2 + 2;
     println!("resultado suma2 {}", a);
}

fn suma3(b:i32) {
    let c = 3;
    println!("resultado suma3 {}", b + c);
}

fn retorno_forma1(a:i32, b:i32) -> i32  //-Z indica que devuelve un valor i32
{
    a + b     
}

fn retorno_forma2(a:i32) -> i32  //-Z indica que devuelve un valor i32
{
    return a *10;  //con return indico algo explicito
}