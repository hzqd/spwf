use std::sync::Arc;
use aoko::{standard::{functions::ext::StdAnyExt, asynchronies::tokio::AsyncExt}, no_std::functions::ext::Utf8Ext};
use spwf::{SharedData, server::handlers::{Index, Handler, StaticFile, VisitCount, NotFound}};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt}, sync::Mutex};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    let data = SharedData { visit_count: 0 }.into_tokio_arc_mutex();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        let data = data.clone();

        tokio::spawn(async move {
            let buf = &mut [0; 1024];

            stream.read(buf).await.unwrap();
            buf.to_str_lossy().echo();

            route(&mut stream, buf, data).await;
        });
    }
}

async fn route(stream: &mut TcpStream, buf: &[u8], shared_data: Arc<Mutex<SharedData>>) {
    if buf.starts_with(b"GET / HTTP/1.1") {
        // Index page
        Index.handle(stream, shared_data).await;
    } else if buf.starts_with(b"GET /static") {
        // Static file
        StaticFile { puth_buf: buf }.handle(stream, shared_data).await;
    } else if buf.starts_with(b"GET /count") {
        // Visit count
        VisitCount.handle(stream, shared_data).await
    } else {
        NotFound.handle(stream, shared_data).await;
    }
}