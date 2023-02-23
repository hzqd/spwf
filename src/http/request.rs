use std::{collections::HashMap, fmt};
use aoko::{structs_new_decl, no_std::{functions::{ext::{AnyExt1, Utf8Ext}, fun::s as str}, pipelines::{tap::Tap, pipe::Pipe}}};
use super::response::HttpVersion;

structs_new_decl! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Request {
        method: HttpMethod = HttpMethod::Get,
        path: String = str(""),
        version: HttpVersion = HttpVersion::V1_1,
    }
}

impl Request {
    pub fn parse_params(&self) -> Vec<&str> {
        // path/<param>/<param>/...
        self.path.as_str().if_not_then(|s| s.trim().is_empty(), |s| s.split('/').collect::<Vec<_>>()).unwrap_or_default()
    }

    pub fn parse_queries(&self) -> HashMap<&str, &str> {
        // <param>?<key>=<value>?...
        HashMap::new().tap_mut(|map| self.parse_params().iter().for_each(|s| {
            if s.contains('?') { s.split('?').collect::<Vec<_>>()[1..].iter().for_each(|s| {
                s.split('=').collect::<Vec<_>>()
                    .pipe(|query| (query.first().map(|&x| x).unwrap_or_default(), query.get(1).map(|&y| y).unwrap_or_default()))
                    .pipe(|(k, v)| map.insert(k, v));
            })}
        }))
    }
}

impl<'a> From<&'a [u8]> for Request {
    fn from(req: &'a [u8]) -> Self {
        req.to_str_lossy()
            .split_whitespace()
            .pipe_mut(|req| Self {
                method: req.next().unwrap_or_default().into(),
                path: req.next().unwrap_or_default().to_owned(),
                version: req.next().unwrap_or_default().into(),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Invalid,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Invalid,
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HttpMethod::*;
        let mut write = |s| write!(f, "{s}");

        match self {
            Get => write("GET"),
            Post => write("POST"),
            Invalid => write("INVALID"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use aoko::no_std::pipelines::tap::Tap;
    use super::Request;

    #[test]
    fn test_parse_req() {
        let expected = Request::new().tap_mut(|r| r.path = "/test".into());
        let req = b"GET /test HTTP/1.1".as_slice().into();
        assert_eq!(expected, req);
    }

    #[test]
    fn test_parse_params() {
        let expected = vec!["", "a", "b"];
        let req: Request = b"GET /a/b HTTP/1.1".as_slice().into();
        assert_eq!(expected, req.parse_params())
    }

    #[test]
    fn test_parse_queries() {
        let expected = HashMap::from([("a", "b"), ("aa", "bb")]);
        let req = Request::new().tap_mut(|r| r.path = "get?a=b?aa=bb".into());
        assert_eq!(expected, req.parse_queries());
    }
}