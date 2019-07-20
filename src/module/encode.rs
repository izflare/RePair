extern crate bit_vec;
extern crate strlib;

pub mod text_32bit;
pub mod text_fble;
pub mod text_huffman_coding;
pub mod ps_pge;

use bit_vec::BitVec;
use std::time::Instant;
use strlib::fble;
use crate::module::{cfg::*};
use super::encode;

pub fn encode(g: &Grammar, mode: &str, bv: &mut BitVec) -> () {

    println!("[Bit encoding]");
    println!("Encoding mode        : {}", mode);

    let start = Instant::now();

    if mode == "32bit" {
        fble::to_bv(0, 8, bv);
        encode::text_32bit::encode(g, bv);
    }
    else if mode == "FBLE" {
        fble::to_bv(1, 8, bv);
        encode::text_fble::encode(g, bv);
    }
    else if mode == "Huffman_coding" {
        fble::to_bv(2, 8, bv);
        encode::text_huffman_coding::encode(g, bv);
    }
    else if mode == "PS+PGE" {
        fble::to_bv(3, 8, bv);
        encode::ps_pge::encode(g, bv);
    }
    else if mode == "POPPT+IFBLE" {
        fble::to_bv(4, 8, bv);
        // encode::sorting::encode(g, bv);
    }
    else if mode == "POPPT+PGE" {
        fble::to_bv(5, 8, bv);
        // encode::sorting::encode(g, bv);
    }
    else {panic!("encoding mode error");}

    let end = start.elapsed();

    println!("Bit length           : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut mode = "";
    let mut mode_number: u32 = 0;
    for i in 0..8 {mode_number <<= 1; if bv[i] {mode_number += 1;}}
    assert!(mode_number <= 5, "Unknown encoding mode");

    if mode_number == 0 {
        mode = "32bit";
        encode::text_32bit::decode(bv, g);
    }
    else if mode_number == 1 {
        mode = "FBLE";
        encode::text_fble::decode(bv, g);
    }
    else if mode_number == 2 {
        mode = "Huffman_coding";
        encode::text_huffman_coding::decode(bv, g);
    }
    else if mode_number == 3 {
        mode = "PS+PGE";
        encode::ps_pge::decode(bv, g);
    }
    else if mode_number == 4 {
        mode = "POPPT+IFBLE";
        // encode::sorting::decode(bv, g);
    }
    else if mode_number == 5 {
        mode = "POPPT+PGE";
        // encode::sorting::decode(bv, g);
    }
    // else {panic!("encoding mode error");}

    println!("Encoding mode : {}", mode);

}
