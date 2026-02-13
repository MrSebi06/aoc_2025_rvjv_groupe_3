mod d1;
mod d2;
mod d3;

fn main() {
    // D1
    // println!("d1p1: {}", d1::p1(include_str!("d1/d1.txt")));
    // println!("d1p2: {}", d1::p2(include_str!("d1/d1.txt")));

    // D2
    // println!("d2p1: {}", d2::p1(include_str!("d2/d2.txt")));
    // println!("d2p2: {}", d2::p2(include_str!("d2/d2.txt")));

    // D3
    // println!("d3p1: {}", d3::p1(include_str!("d3/d3.txt")));
    println!("d3p2: {}", d3::p2_mika(include_str!("d3/d3.txt")));
    // println!("d3p2: {}", d3::p2_sacha(include_str!("d3/d3.txt"), 12));
    // println!("d3p2: {}", d3::p2_seb(include_str!("d3/d3.txt")));
}
