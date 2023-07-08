use tokio::io::{copy, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

async fn clear_stream(c_read: &mut OwnedReadHalf) {
    let mut buffer = [0u8; 1024];

    c_read.read(&mut buffer).await.unwrap();
}

async fn proxy(client: TcpStream) {
    let server = TcpStream::connect("localhost:22").await.unwrap();

    let (mut c_read, mut c_write) = client.into_split();
    let (mut s_read, mut s_write) = server.into_split();

    c_write
        .write(
            "HTTP/1.1 101 Switching Protocols\r\nContent-Length: 1048576000000\r\n\r\n".as_bytes(),
        )
        .await
        .unwrap();

    clear_stream(&mut c_read).await;

    tokio::spawn(async move { copy(&mut s_read, &mut c_write).await });
    tokio::spawn(async move { copy(&mut c_read, &mut s_write).await });
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8880").await.unwrap();

    loop {
        let (client, _) = listener.accept().await.unwrap();
        spawn(async move {
            proxy(client).await;
        });
    }
}
