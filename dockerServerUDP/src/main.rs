use dockerServerUDP::Repository;

use std::{thread, time::Duration};
use std::net;
use std::io::*;
use std::net::UdpSocket;
use std::str;
use std::str::from_utf8;



//========Implementacion de socket UDP ==========//
//Espera como argumento la direccion del ip, para despues verificar si se hizo la conexion
pub fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let success = net::UdpSocket::bind(listen_on);
    let mut mySocketUDP;
    println!("{:?}",success);

    match success {
        //Mensaje de error en dado caso de no conectado
        Err(err) => panic!("No se pudo establecer la conexion: {}", err),

        Ok(sock) => {
            println!("Conectado a {}", listen_on);
            mySocketUDP = sock;
        }
    }
    mySocketUDP
}


fn leerMensaje(socket: net::UdpSocket) -> String {
    let mut buf = [0; 2048];
    println!("Reading data");
    let result = socket.recv_from(&mut buf);
    drop(socket);
    let mut req_msg ;
    let mut msg;
    match result {
      Ok((amt, src)) => {
          let buf = &mut buf[..amt];
        println!("Received data from {}", src);
        req_msg = str::from_utf8(&buf).unwrap();
        msg = String::from(req_msg);
      },
      Err(err) => panic!("Read error: {}", err)
    }
    msg
}
  
  
//Modificar para hacer una consulta en BD
pub fn enviarMensaje(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    println!("Mensaje enviado");
    let result = socket.send_to(&data, target);
    drop(socket);
    match result {
      Ok(amt) => println!("Se enviaron {} bytes", amt),
      Err(err) => panic!("Write error: {}", err)
    }
}

//Comunicacion entre el cliente y servidor
pub fn listen(listen_on: net::SocketAddr)-> String {
    let socket = socket(listen_on);
      let mut data = leerMensaje(socket);
      data
}


fn main(){
    println!("\n Inicio del programa");

    //=======socket=====
	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let my_dir = net::SocketAddrV4::new(ip, 1425);
	let send_dir = net::SocketAddrV4::new(ip, 1426);
	println!("Mi direccion {} ",my_dir);
	
	println!("{:}", "=".repeat(80));

    //Seccion: peticion del cliente
    //Hacer que el servidor le responda con un string
	let msg = listen(net::SocketAddr::V4(my_dir));
	println!("mensaje recibido del cliente: {:?}", msg);
	println!("{:}", "=".repeat(80));
    //Seccion: El servidor le devuelve la respuesta al cliente
	thread::sleep(Duration::from_millis(1000));
	let serverMensaje = msg;
    println!("enviandole respuesta al cliente");
	let data: Vec<u8> = serverMensaje.as_bytes().to_vec();
	enviarMensaje(net::SocketAddr::V4(my_dir), net::SocketAddr::V4(send_dir), data);

    // Mostrar los productos que empiecen con m
    // let mut repo = Repository::new();
    // let products = repo.products_start_withid(&msg);
    // print!("========== Productos que empiezan con M =========");
    // for item in products{
    //     println!("| {} | {:<10} |", item.id, item.name);
    // }

    println!("Fin del programa");
}