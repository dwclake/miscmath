use miscmath::prelude::*;

fn main() {
    let x = vector::Vec2::unit();
    dbg!(&x);

    let y = vector::Vec2::create(&0.5, &7.2);
    dbg!(&y);
 
    dbg!(y == y);
}
