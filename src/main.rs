#[allow(unused_imports)]
use std::net::UdpSocket;

use bytes::{BufMut, BytesMut};

pub struct DnsQuestion {
    pub domain_name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
}

impl DnsQuestion
{
    pub fn new() -> Self
    {
        DnsQuestion
        { 
            domain_name: Self::encoded_domain_name("codecrafters.io"), 
            record_type: 1, 
            class: 1, 
        }
    }
    pub fn encoded_domain_name(domain: &str) -> Vec<u8>
    {
        let mut encoded: Vec<u8> = Vec::new();

        for part in domain.split('.')
        {
            if !part.is_empty()
            {
                encoded.push(part.len() as u8);
                encoded.extend_from_slice(part.as_bytes());
            }
        }
        encoded.push(0);
        return encoded;
    }
}


fn main()
{
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf)
        {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response: [u8; 12]= [0x04, 0xd2, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

/*
dig @127.0.0.1 -p 2053 +noedns codecrafters.io
*/