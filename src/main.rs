use rustls::{server::NoClientAuth, ServerConfig};
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio_rustls::{Accept, TlsAcceptor};
use tokio_rustls::server::TlsStream;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server certificates
    let certs = rustls_pemfile::certs(&mut std::io::Cursor::new(include_str!("../cert.pem"))).flatten().collect();
    let keys = rustls_pemfile::private_key(&mut std::io::Cursor::new(include_str!("../key.pem"))).unwrap().unwrap();
    // Create server configuration
    let config = ServerConfig::builder().with_no_client_auth().with_single_cert(certs, keys).unwrap();

    // Create TCP listener and TLS acceptor
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_client(stream, acceptor.clone()));
    }
}
async fn handle_client(stream: tokio::net::TcpStream, acceptor: TlsAcceptor) -> Result<(),()> {
    let mut stream = match acceptor.accept(stream).await {
        Ok(stream) => stream,
        Err(err) => 
        {
            println!("{}", err);
            return Err(());
        } 
    };
    // Handle the stream (read/write)
    
    stream.write_all(
        &b"HTTP/1.0 200 ok\r\n\
    Connection: close\r\n\
    Content-length: 12\r\n\
    \r\n\
    Hello world!"[..],
    )
    .await.expect("write did something wrong");

    Ok(())
}