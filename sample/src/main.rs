fn main() {
    println!("Hello, world!");

    //variables
    let mut a = 1;
    print!("valor de variable es {}", a);
    a = 2;
    println!("valor de variable es {}", a);
    
    //scalar types
    //integer,float,boolean,char  

    //integer
    let mut a = 412342523;
    a = a + 1;
    println!("El valor es {}", a);

    //float - decimal points
    let a = 5.12;    
    println!("El valor es {}", a);

    //boolean - dos formas de definir anotación
    let a = true;        
    println!("El valor es {}", a);
    let b:bool = false;
    println!("El valor es {}", b);
    
    //tuplas 
    //    => agrupa juntos una variedad de tipos. Acceso con .
    //    => i32, i64 (integer)
    let tup: (i32,i32,i32) = (1,2,3);
    let (x,y,z) = tup;
    println!("Valores: X = {}, Y = {}, Z = {}", x,y,z);
   
    //arrays 
    //    => 
    let numbers = [1, 2 ,3 ,4 ,5, 6];
    println!("Primer valor es {}", numbers[0]);  //resultado 1
    println!("Segundo valor es {}", numbers[1]);  //resultado 2  

}
