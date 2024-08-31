use std::io::{stdin, stdout, Write};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;

    println!("start client: {}", addr);

    loop {
        // 入力受け付け
        print!("> ");
        stdout().flush().unwrap();

        let mut msg = String::new();
        stdin().read_line(&mut msg).expect("Failed to read line");

        // 入力値がexitか:qで終了
        if msg == "exit\r\n" || msg == ":q\r\n" {
            break;
        } else if msg == "\r\n" {
            continue;
        } else {
            // 入力値をサーバに送信し、受信データを表示
            stream.write(msg.as_bytes()).await?;

            let mut buf = Vec::with_capacity(4096);
            stream.read_buf(&mut buf).await?;

            let received = String::from_utf8(buf).unwrap();
            println!("server > {}", received);
        }
    }
    Ok(())
}
