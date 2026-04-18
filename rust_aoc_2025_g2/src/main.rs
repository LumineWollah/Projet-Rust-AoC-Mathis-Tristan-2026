use crate::d1::{d1p1,d1p2};
use crate::d2::{d2p1,d2p2};
use crate::d3::{d3p1,d3p2};
use crate::d4::{d4p1,d4p2};
use crate::d5::{d5p1,d5p2};
use crate::d6::{d6p1,d6p2};
use crate::d7::{d7p1,d7p2};
use crate::d8::{d8p1,d8p2};
use crate::d9::{d9p1,d9p2};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;


fn main() {
    println!("D1P1 : {}", d1p1(include_str!("d1/d1.txt"))); // 1123
    println!("D1P2 : {}", d1p2(include_str!("d1/d1.txt"))); // 6695
    println!("D2P1 : {}", d2p1(include_str!("d2/d2.txt"))); // 64215794229
    println!("D2P2 : {}", d2p2(include_str!("d2/d2.txt"))); // 85513235135
    println!("D3P1 : {}", d3p1(include_str!("d3/d3.txt")));
    println!("D3P2 : {}", d3p2(include_str!("d3/d3.txt")));
    println!("D4P1 : {}", d4p1(include_str!("d4/d4.txt"))); // 1486
    println!("D4P2 : {}", d4p2(include_str!("d4/d4.txt"))); // 9024
    println!("D5P1 : {}", d5p1(include_str!("d5/d5.txt")));
    println!("D5P2 : {}", d5p2(include_str!("d5/d5.txt")));
    println!("D6P1 : {}", d6p1(include_str!("d6/d6.txt"))); // 4722948564882
    println!("D6P2 : {}", d6p2(include_str!("d6/d6.txt"))); // 9581313737063
    println!("D7P1 : {}", d7p1(include_str!("d7/d7.txt")));
    println!("D7P2 : {}", d7p2(include_str!("d7/d7.txt")));
    println!("D8P1 : {}", d8p1(include_str!("d8/d8.txt"),1000)); // 181584
    println!("D8P2 : {}", d8p2(include_str!("d8/d8.txt"))); // 8465902405
    println!("D9P1 : {}", d9p1(include_str!("d9/d9.txt")));
    println!("D9P2 : {}", d9p2(include_str!("d9/d9.txt")));
}
