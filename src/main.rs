#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use tokio::net::UdpSocket;
use std::io;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[tokio::main]
async fn main() -> io::Result<()> {

    let mut ctx: *mut lwm2m_context_t = unsafe {
        lwm2m_init(ptr::null_mut())
    };

    println!("Start Server");

    let sock = UdpSocket::bind("0.0.0.0:8080").await?;

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
}
