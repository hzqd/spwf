use std::sync::Arc;
use aoko::{no_std::{pipelines::tap::Tap, functions::{ext::Utf8Ext, fun::s as str, monoid::{StrDot, NotMonoid}}}, val};
use tokio::{net::TcpStream, io::AsyncWriteExt, sync::Mutex, fs};
use crate::{SharedData, http::response::{Response, HttpStatus, ContentType}};

pub struct Index;
pub struct NotFound;
pub struct VisitCount;
pub struct StaticFile<'a> {
    pub puth_buf: &'a [u8]
}

macro_rules! headers {
    ($ct:expr, $body:expr) => {
        [   ("Content-Type".into(), $ct.to_string()),
            ("Content-Length".into(), $body.len().to_string())
        ].into()
    };
}

macro_rules! exe_stream {
    ($s:ident, $r:ident) => {
        $s.write_all(&$r).await.unwrap();
        $s.flush().await.unwrap();
    };
}

pub trait Handler {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>);
}

impl Handler for Index {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let res_byt = Response::new()
            .tap_mut(|r| {
                r.body = "Index Page".as_bytes();
                r.headers = headers!(ContentType::Html, r.body);
            }).as_bytes();

        exe_stream!(stream, res_byt);
    }
}

impl Handler for VisitCount {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        shared_data.lock().await.visit_count += 1;
        val! {
            visit_count = shared_data.lock().await.visit_count;
            body = format!("{visit_count} Times!");
            res_byt = Response::new()
                .tap_mut(|r| {
                    r.body = body.as_bytes();
                    r.headers = headers!(ContentType::Html, r.body);
                }).as_bytes();
        }
        exe_stream!(stream, res_byt);
    }
}

impl Handler for NotFound {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let res_byt = Response::new()
            .tap_mut(|r| {
                r.status = HttpStatus::NotFound;
                r.body = "404 Not Found".as_bytes();
                r.headers = headers!(ContentType::Html, r.body)
            }).as_bytes();

        exe_stream!(stream, res_byt);
    }
}

impl Handler for StaticFile<'_> {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        for s in self.puth_buf.to_str_lossy().split_whitespace() {
            if s.contains("/static") {
                val! {
                    path = s.split('/').enumerate().filter(|&(u, _)| u != 0 && u != 1).map(|(_, s)| s).collect::<String>();
                    file = fs::read(str("static/") + &path).await;
                }
                let Ok(file) = file else {
                    NotFound.handle(stream, shared_data.clone()).await;
                    return;
                };
                
                let res_byt = Response::new()
                    .tap_mut(|r| {
                        r.body = &file;
                        r.headers = headers!(parse_content_type(&path), file);
                    }).as_bytes();
                
                exe_stream!(stream, res_byt);
            }
        }
        fn parse_content_type(req: &str) -> ContentType {
            use ContentType::*;
            
            macro_rules! dot {
                ($s:expr) => {
                    &StrDot::merge("", $s)
                };
            }

            if req.contains(dot!("htm")) {
                Html
            } else if req.contains(dot!("txt")) {
                PlainText
            } else if req.contains(dot!("css")) {
                Css
            } else if req.contains(dot!("png")) || req.contains(dot!("jpg")) || req.contains(dot!("ico")) {
                AvifImage
            } else {
                Html
            }
        }
    }
}