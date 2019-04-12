#[macro_use]
extern crate ctor;

use noisy_float::prelude::*;

#[ctor]
static HIGH_TIME: N64 = n64(1e50);

fn main() {
    HIGH_TIME.raw();
    println!("Hello, world!");
}
