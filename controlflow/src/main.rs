fn main() {
    
    let a = 10;
    if a < 20 {
        println!("el nº es menor de 20 y es: {}", a);
    }else{
        println!("el nº es mayor de 20 y es: {}", a);
    }
    
    if a % 3 == 0 {   //resultado del resto
        println!("a can be divided by 3 to get 0: {}", a);
    }else{
        println!("a can't be divided by 3 to get 0: {}", a);
    }

    loop{
        println!("hola");
        break;   //para ejecutar una vez sólo
    }

    let mut a = 3;
    while a != 0 {      //distinto de 0
        println!("{}", a);
        a = a -1;
    }

    let a = [0 ,1 ,2 ,3 ,4 ,5];
    for value in a.iter() {
        println!("valor array1 {}", value);
    }
    
    let b = 0..10;
    for i in b { //aqui b es un iterador, siempre es un iterador ahí
        println!("valor array2 {}", i);        
    }
    

}


