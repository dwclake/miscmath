use mathlib_rust;
use mathlib_rust::{mathlib, vector2};

fn main() {
    println!("Hello, world!");
    let mut x = 42;
    println!("X = {}.", x);
    x = vector2::add(3,4);
    println!("X = {}.", x);

    let x: vector2;
    x.add();
}
