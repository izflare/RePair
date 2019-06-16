extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use strlib::fixed;
use super::super::{cfg::*};

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    fixed::to_bv(g.terminal.len() as u32, 32, bv);
    for e in &g.terminal {fixed::to_bv(*e as u32, 8, bv);}
    fixed::to_bv(g.rule.len() as u32, 32, bv);
    for e in &g.rule {fixed::to_bv(e[0] as u32, 32, bv); fixed::to_bv(e[1] as u32, 32, bv);}
    for e in &g.sequence {fixed::to_bv(*e, 32, bv);}

}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut zlen = 0;
    let mut z: u8 = 0;
    let mut vlen = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut first = true;
    let mut s: u32 = 0;
    let mut sum = 40;

    for i in 8..bv.len() {
        if i < 8 + 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 8 + 32 + zlen * 8 {
            z <<= 1; if bv[i] {z += 1;}
            if (i - sum) % 8 == 7 {g.terminal.push(z); z = 0; sum = i + 1;}
        }
        else if i < 8 + 32 * 2 + zlen * 8 {vlen <<= 1; if bv[i] {vlen += 1;}}
        else if i < 8 + 32 * 2 + zlen * 8 + vlen * 64 {
            if first {
                a <<= 1; if bv[i] {a += 1;}
                if (i - sum) % 32 == 31 {first = false;}
            }
            else {
                b <<= 1; if bv[i] {b += 1;}
                if (i - sum) % 32 == 31 {g.rule.push(vec![a, b]); a = 0; b = 0; first = true; sum = i + 1;}
            }
        }
        else {
            s <<= 1; if bv[i] {s += 1;}
            if (i - sum) % 32 == 31 {g.sequence.push(s);}
        }
    }

}
