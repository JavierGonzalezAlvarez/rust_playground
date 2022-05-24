/*
fn main() {  
    //rule 1
    let a = 1;  

    //rule 2: Así no funciona. B está fuera de rango
    {
        let b = 10;
    }
    println!("{}", b);

    //rule 3
    let s =  String::from("Javier"); //forma completa de declarar una variable
    main1(s);  //va a la funcion y se imprime
    //no funciona porque aqui hay dos propietarios de la variables
    println!("{}", s);
}

*/

/*
fn main1(x: String){
    println!("{}", x);
}
*/

//como se soluciona regla3?
fn main() {//ojo, no puedo haber 2 "fn main(()", es para dar el ejemplo
    //rule 3
    let s =  String::from("Javier"); 
    main1(&s);  //con una referencia que le mandamos a la funcion, con el signo &    
    println!("{}", s);
}

fn main1(x:&String){
    println!("{}", x);
}
