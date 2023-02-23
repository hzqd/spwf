use std::{collections::HashMap, fmt::{Display, Result}};
use aoko::no_std::pipelines::pipe::Pipe;

#[derive(Debug)]
pub struct Response<'a> {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: &'a [u8],
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Self { version: HttpVersion::V1_1, status: HttpStatus::Ok, headers: HashMap::new(), body: b"" }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{} {}\r\n{:?}\r\n\r\n", self.version, self.status, self.headers)
            .as_bytes()
            .pipe(|b| [b, self.body])
            .concat()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpVersion {
    V1_1, Invalid
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        use HttpVersion::*;
        let mut write = |s| write!(f, "{s}");

        match self {
            V1_1 => write("HTTP/1.1"),
            Invalid => write("Invalid http version"),
        }
    }
}

impl From<&str> for HttpVersion {
    fn from(version: &str) -> Self {
        use HttpVersion::*;

        match version {
            "HTTP/1.1" => V1_1,
            _ => Invalid
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HttpStatus {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        use HttpStatus::*;
        let mut write = |s| write!(f, "{s}");

        match self {
            Ok => write("200 OK"),
            NotFound => write("404 Not Found"),
            BadRequest => write("400 Bad Request"),
            InternalServerError => write("500 Internal Server Error"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    Html,
    PlainText,
    AvifImage,
    Css,
}

impl From<&str> for ContentType {
    fn from(value: &str) -> Self {
        use ContentType::*;

        match value {
            "text/plain" => PlainText,
            "text/html" => Html,
            "text/css" => Css,
            "image/avif" => AvifImage,
            _ => PlainText,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        use ContentType::*;
        let mut write = |s| write!(f, "{s}");

        match self {
            Html => write("text/html; charset=utf-8"),
            Css => write("text/css; charset=utf-8"),
            AvifImage => write("image/avif"),
            PlainText => write("text/plain; charset=utf-8"),
        }
    }
}

#[cfg(test)]
mod test {
    use aoko::{val, var, no_std::{functions::{fun::s, ext::Utf8Ext}, pipelines::tap::Tap}};
    use super::*;

    #[test]
    fn test_status_to_string() {
        let expected = s("500 Internal Server Error");
        let res = HttpStatus::InternalServerError;
        assert_eq!(expected, res.to_string());
    }

    #[test]
    fn test_all() {
        val! {
            expected = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 3\r\n\r\nHi!".as_bytes();
            content = "Hi!";
            content_length = content.len().to_string();
            version = HttpVersion::V1_1;
            status = HttpStatus::Ok;
        }
        var! {
            res = Response::new();
        }

        let byt = res.tap_mut(|r| {
            r.status = status;
            r.version = version;
            r.body = content.as_bytes();
            r.headers = [
                ("Content-Type".into(), ContentType::Html.to_string()),
                ("Content-Length".into(), content_length)
            ].into();
        }).as_bytes();
        
        assert_eq!(expected.to_str(), byt.to_str());
    }
}