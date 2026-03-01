#![allow(unused_variables)]

mod traits;

mod rust {
    include!(concat!(env!("OUT_DIR"), "/rust.rs"));
}

pub use rust::*;
pub use traits::*;
