use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;

    println!("start client: {}", addr);

    loop {
        let mut msg = String::new();
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        if msg == "exit\r\n" {
            break;
        } else {
            stream.write(msg.as_bytes()).await?;

            let mut buf = Vec::with_capacity(4096);
            stream.read_buf(&mut buf).await?;

            let received = String::from_utf8(buf).unwrap();
            println!("received: {}", received);
        }
    }
    Ok(())
}
