#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod http;
pub mod server;

pub struct SharedData {
    pub visit_count: u32,
}