use crate::d1::{d1p1,d1p2};
use crate::d2::{d2p1,d2p2};

mod d1;
mod d2;

fn main() {
    println!("D1P1 : {}", d1p1(include_str!("d1/d1.txt"))); // 1123
    println!("D1P2 : {}", d1p2(include_str!("d1/d1.txt"))); // 6695
    println!("D2P1 : {}", d2p1(include_str!("d2/d2.txt"))); // 64215794229
    println!("D2P2 : {}", d2p2(include_str!("d2/d2.txt"))); // 85513235135
}
