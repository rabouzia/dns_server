#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
                break;
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
				let response= [
                    0b00000100, 0b11010010, 0b10000000, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0x0c, b'c',
                    b'o', b'd', b'e', b'c', b'r', b'a', b'f', b't', b'e', b'r', b's', 0x02, b'i',
                    b'o', 0x00, 0, 1, 0, 1,];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
