use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:5747").await.unwrap();

    loop {
        let (client, _) = listener.accept().await.unwrap();
        spawn(async move {
            proxy(client).await;
        });
    }
}

async fn proxy(client: TcpStream) {
    let server = TcpStream::connect("localhost:5746").await.unwrap();

    let (mut c_read, mut c_write) = client.into_split();
    let (mut s_read, mut s_write) = server.into_split();

    tokio::spawn(async move { io::copy(&mut c_read, &mut s_write).await });
    tokio::spawn(async move { io::copy(&mut s_read, &mut c_write).await });

    println!("Done");
}
