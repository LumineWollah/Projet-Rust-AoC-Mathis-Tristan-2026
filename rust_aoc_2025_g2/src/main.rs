use crate::d1::{d1p1,d1p2};
use crate::d2::{d2p1,d2p2};
use crate::d4::{d4p1,d4p2};
use crate::d6::{d6p1,d6p2};
use crate::d8::{d8p1,d8p2};

mod d1;
mod d2;
mod d4;
mod d6;
mod d8;


fn main() {
    println!("D1P1 : {}", d1p1(include_str!("d1/d1.txt"))); // 1123
    println!("D1P2 : {}", d1p2(include_str!("d1/d1.txt"))); // 6695
    println!("D2P1 : {}", d2p1(include_str!("d2/d2.txt"))); // 64215794229
    println!("D2P2 : {}", d2p2(include_str!("d2/d2.txt"))); // 85513235135
    println!("D4P1 : {}", d4p1(include_str!("d4/d4.txt"))); // 1486
    println!("D4P2 : {}", d4p2(include_str!("d4/d4.txt"))); // 9024
    println!("D6P1 : {}", d6p1(include_str!("d6/d6.txt"))); // 4722948564882
    println!("D6P2 : {}", d6p2(include_str!("d6/d6.txt"))); // 9581313737063
    println!("D8P1 : {}", d8p1(include_str!("d8/d8.txt"),1000)); // 181584
    println!("D8P2 : {}", d8p2(include_str!("d8/d8.txt"))); // 8465902405
}
