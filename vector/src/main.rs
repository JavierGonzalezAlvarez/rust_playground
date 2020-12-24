fn main() {
    //crear un vector con valores que serán i32
    //los vectores han de ser del mismo valor
    
    //forma de crear vectores:1
    println!("Crear y ver vectores");
    let v2 = vec![1,2,3,4,5];
    for i in &v2 {
        println!("vector: {}", i);
    }
    
    //forma de crear vectores:2
    let mut v1: Vec<i32> = Vec::new();
    println!("Crear y añadir valores a un vector");
    //añadir un valor a un vector
    v1.push(7);
    v1.push(9);
    v1.push(18);
    v1.push(54);
    //ver valores: opcion 1 con elemento for
    for i in v1 {
        println!("{}", i);
    }
    
    //ver valores: opcion 2, sin for
    println!("Crear y ver valores de un vector");
    let v2 = vec![1,2,3,4,5];
    let ver: &i32 = &v2[2];
    println!("Es vector escogido es {}", ver);
    
    //ver valores: opcion 3, con match y get
    match v2.get(0) {
        //Some(ver) => println!("{}", v2[3]),
        Some(resultado) => println!("El resultado match es: {}", resultado),
        None => println!("No hay elemento"),
    }
    
    //ver elemento añadido
    let mut v3 = vec![1,2,3,4,5];
    v3.push(6);
    let s1 = &v3[3];
    println!("Es vector es {}", s1);
    
    //no funciona ya que el vector esta en memoria y al añadir un elemento
    //puede requerir añadir una nueva posicion en memoria. Borrowing previene
    /*
    let mut v4 = vec![1,2,3,4,5];
    let s2 = &v4[4];
    v4.push(6);
    println!("Es vector escogido es {}", s2);
    */
    
    //cambiar el valor del vector usamos DEREFENCE
    let mut v5 = vec![23,43,67];
    for i in &mut v5 {
        *i += 50;
        println!("Vector: {}", *i);
    }
    
}

/* salida
Crear y ver vectores
vector: 1
vector: 2
vector: 3
vector: 4
vector: 5
Crear y añadir valores a un vector
7
9
18
54
Crear y ver valores de un vector
Es vector escogido es 3
El resultado match es: 1
Es vector es 4
Vector: 73
Vector: 93
Vector: 117
*/