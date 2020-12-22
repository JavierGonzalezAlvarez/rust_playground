-----------------
Rust
-----------------

instalar rust en linux
-------------------------------
$ sudo apt update 
$ sudo apt upgrade 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
$ source $HOME/.cargo/env 

compilar un fichero
--------------------
$ rustc nombre_del_fichero

ver resultado
--------------------------
$ ./nombre_del_fichero

si el permiso es denegado => cargo clean y cerrar VSC

crear un proyecto
-----------------------------
$ cargo new nombre_proyecto --bin

compilar con cargo
----------------------
cd nombre_proyecto
$ cargo build

ejecutar con cargo
----------------------
>name_project/cargo run

