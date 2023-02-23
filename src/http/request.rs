use std::collections::HashMap;
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
    pub fn parse_params(&self) -> Vec<String> {
        self.path.as_str().if_not_then(|s| s.trim().is_empty(), |s| s.split('/').map(|s| str(s)).collect::<Vec<_>>()).unwrap_or_default()
    }

    pub fn parse_queries(&self) -> HashMap<String, String> {
        HashMap::new().tap_mut(|map| self.parse_params().iter().for_each(|s| {
            if s.contains('?') { s.split('?').collect::<Vec<_>>()[1..].iter().for_each(|s| {
                s.split('=').collect::<Vec<_>>()
                    .pipe(|query| (query.first().map(|&x| x).unwrap_or_default(), query.get(1).map(|&y| y).unwrap_or_default()))
                    .pipe(|(k, v)| map.insert(str(k), str(v)));
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