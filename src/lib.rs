#![allow(unused_variables)]

mod traits;

pub use traits::*;

include!(concat!(env!("OUT_DIR"), "/rust.rs"));
