enum IpVariants{
    IPV4(String),
    IPV6(String),
}

enum Message{
    Write(String),    
}

impl Message{
    fn call(&self){
        println!("dentro de implementacion");
    }
}

fn main() {
    //creamos una instancia de enums.
    let _ip1 = IpVariants::IPV4(String::from("192.168.0.1"));
    let _ip2 = IpVariants::IPV6(String::from("::1"));
        
    let variable = Message::Write(String::from("hola"));
    //llamada a la función call
    variable.call();       
}

