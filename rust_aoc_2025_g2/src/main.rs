use crate::d1::{d1p1,d1p2};

mod d1;

fn main() {
    println!("D1P1 : {}", d1p1(include_str!("d1/d1.txt"))); // 1123
    println!("D1P2 : {}", d1p2(include_str!("d1/d1.txt"))); // 6695
}
