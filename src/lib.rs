#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

#[macro_use] extern crate nom;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate mio;
extern crate bytes;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate openssl;
extern crate pool;
extern crate uuid;
extern crate net2;

pub mod network;
pub mod parser;
pub mod messages;
