extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::cmp::{min, max};
use strlib::{fixed, block_fixed};
use super::super::{cfg::*};

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    let mut deltas: Vec<u32> = Vec::new();
    let mut starting_values: Vec<u32> = Vec::new();
    let mut starting_points: Vec<u32> = Vec::new();
    let mut deltas_minimums: Vec<u32> = Vec::new();
    let mut max_first: BitVec = BitVec::new();
    let mut last_max: u32 = 0;
    let mut last_incr_seq: u32 = 0;

    for i in 0..g.rule.len() {
        let ab = &g.rule[i];
        let a = max(ab[0], ab[1]);
        deltas_minimums.push(a - min(ab[0], ab[1]));
        max_first.push(a == ab[0]);

        if a >= last_max {
            deltas.push(a - last_max);
        }
        else {
            starting_values.push(a);
            starting_points.push(i as u32 - last_incr_seq);
            last_incr_seq = i as u32;
        }
        last_max = a;
    }

    let mut z = BitVec::new();
    fixed::encode(&g.terminal.iter().map(|x| *x as u32).collect::<Vec<u32>>(), &mut z);
    fixed::to_bv(z.len() as u32, 32, bv);
    for b in &z {bv.push(b);}

    let mut v: Vec<u32> = Vec::new();
    let blocksize = 6;
    v.push(deltas.len() as u32);
    v.push(starting_values.len() as u32);
    for e in &deltas {v.push(*e);}
    for e in &starting_values {v.push(*e);}
    for e in &starting_points {v.push(*e);}
    for e in &deltas_minimums {v.push(*e);}

    let mut v_bits = BitVec::new();
    block_fixed::encode(&v, blocksize, &mut v_bits);
    fixed::to_bv(v_bits.len() as u32, 32, bv);
    for b in &v_bits {bv.push(b);}

    fixed::to_bv(max_first.len() as u32, 32, bv);
    for b in &max_first {bv.push(b);}

    block_fixed::encode(&g.sequence, blocksize, bv);

    println!("Increasing sequences : {:?}", starting_points.len());
}

pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut zlen = 0;
    let mut z = BitVec::new();
    let mut vbitslen = 0;
    let mut vbits = BitVec::new();
    let mut dlen = 0;
    let mut max_first = BitVec::new();
    let mut s = BitVec::new();

    for i in 8..bv.len() {
        if i < 8 + 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 8 + 32 + zlen {z.push(bv[i]);}
        else if i < 8 + 32 * 2 + zlen {vbitslen <<= 1; if bv[i] {vbitslen += 1;}}
        else if i < 8 + 32 * 2 + zlen + vbitslen {vbits.push(bv[i]);}
        else if i < 8 + 32 * 3 + zlen + vbitslen {dlen <<= 1; if bv[i] {dlen += 1;}}
        else if i < 8 + 32 * 3 + zlen + vbitslen + dlen {max_first.push(bv[i]);}
        else {s.push(bv[i]);}
    }

    let mut zvec: Vec<u32> = Vec::new();
    fixed::decode(&z, &mut zvec);
    g.terminal = zvec.iter().map(|x| *x as u8).collect::<Vec<u8>>();

    block_fixed::decode(&s, &mut g.sequence);
    if let Some(last) = g.sequence.last() {if *last == 0 {g.sequence.pop();}}

    let mut v: Vec<u32> = Vec::new();
    block_fixed::decode(&vbits, &mut v);

    let mut deltas: Vec<u32> = Vec::new();
    let mut starting_values: Vec<u32> = Vec::new();
    let mut starting_points: Vec<u32> = Vec::new();
    let mut deltas_minimums: Vec<u32> = Vec::new();

    let deltas_len = v[0] as usize;
    let incr_len = v[1] as usize;
    for i in 2..v.len() {
        if i < 2 + deltas_len {deltas.push(v[i]);}
        else if i < 2 + deltas_len + incr_len {starting_values.push(v[i]);}
        else if i < 2 + deltas_len + incr_len * 2 {starting_points.push(v[i]);}
        else {deltas_minimums.push(v[i]);}
    }

    let mut last_max: u32 = 0;
    let mut start_pos = starting_points[0] as usize;

    let mut deltas_i = 0;
    let mut incr_i = 0;
    for i in 0..dlen {
        if i == start_pos {
            if let Some(lm) = starting_values.get(incr_i) {
                last_max = *lm;
                incr_i += 1; 
                if let Some(spd) = starting_points.get(incr_i) {start_pos += *spd as usize;}
            }
        }
        else {last_max += deltas[deltas_i]; deltas_i += 1;}
        g.rule.push(if max_first[i] {vec![last_max, last_max - deltas_minimums[i]]} 
                    else {vec![last_max - deltas_minimums[i], last_max]});
    }

}
