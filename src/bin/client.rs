use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Client connected to the server");
    let request = b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";

    client.write_all(request).await?;

    let mut buffer = Vec::new();
    client.read_to_end(&mut buffer).await?;

    println!("Response:\n{}", String::from_utf8_lossy(&buffer));

    Ok(())
}
