extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::time::Instant;
use std::cmp::{min, max};
use strlib::delta;
use strlib::gamma;
use strlib::ffenc;
use super::{cfg::*};

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    let start = Instant::now();

    let mut z = BitVec::new();
    ffenc::encode(&g.terminal.iter().map(|x| *x as u32).collect::<Vec<u32>>(), &mut z);

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
        deltas_minimums.push(a - min(ab[0], ab[1]) + 1);
        max_first.push(a == ab[0]);

        if a >= last_max {
            deltas.push(a - last_max + 1);
        }
        else {
            starting_values.push(a);
            starting_points.push(i as u32 - last_incr_seq);
            last_incr_seq = i as u32;
        }
        last_max = a;
    }

    ffenc::to_bits(z.len() as u32, 32, bv);
    for b in &z {bv.push(b);}

    let mut deltas_bits = BitVec::new();
    gamma::encode(&deltas, &mut deltas_bits);
    ffenc::to_bits(deltas_bits.len() as u32, 32, bv);
    for b in &deltas_bits {bv.push(b);}

    let mut starting_values_bits = BitVec::new();
    ffenc::encode(&starting_values, &mut starting_values_bits);
    ffenc::to_bits(starting_values_bits.len() as u32, 32, bv);
    for b in &starting_values_bits {bv.push(b);}

    let mut starting_points_bits = BitVec::new();
    gamma::encode(&starting_points, &mut starting_points_bits);
    ffenc::to_bits(starting_points_bits.len() as u32, 32, bv);
    for b in &starting_points_bits {bv.push(b);}

    let mut deltas_minimums_bits = BitVec::new();
    ffenc::encode(&deltas_minimums, &mut deltas_minimums_bits);
    ffenc::to_bits(deltas_minimums_bits.len() as u32, 32, bv);
    for b in &deltas_minimums_bits {bv.push(b);}

    ffenc::to_bits(max_first.len() as u32, 32, bv);
    for b in &max_first {bv.push(b);}

    ffenc::encode(&g.sequence, bv);

    let end = start.elapsed();
    println!("[Result: bit encoding]");
    println!("Increasing seq    : {:?}", starting_points.len() + 1);
    println!("Bit length        : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
}

// not impl yet
pub fn decode(bv: &BitVec, w: &mut Vec<u8>) -> () {
    let mut v: Vec<u32> = Vec::new();
    let mut g: Grammar = Grammar::new();
    let mut zlen = 0;
    let mut c: u32 = 0;
    let mut glen = 0;
    let mut sbitlen = 0;
    let mut u: Vec<u32> = Vec::new();
    let mut lr: BitVec = BitVec::new();
    let mut slen = 0;
    let mut d: BitVec = BitVec::new();

    for i in 0..bv.len() {
        if i < 8 {
            zlen <<= 1; if bv[i] {zlen += 1;}
        }
        else if i < 8 + zlen * 8 {
            c <<= 1; if bv[i] {c += 1;}
            if i % 8 == 7 {g.terminal.push(c as u8); c = 0;}
        }
        else if i < 40 + zlen * 8 {
            glen <<= 1; if bv[i] {glen += 1;}
        }
        else if i < 72 + zlen * 8 {
            sbitlen <<= 1; if bv[i] {sbitlen += 1;}
        }
        else if i < 72 + zlen * 8 + glen * sbitlen {
            c <<= 1; if bv[i] {c += 1;}
            if (i - (72 + zlen * 8)) % sbitlen == sbitlen - 1 {u.push(c as u32); c = 0;}
        }
        else if i < 72 + zlen * 8 + glen * sbitlen + glen {
            lr.push(bv[i]);
        }
        else if i < 104 + zlen * 8 + glen * sbitlen + glen {
            slen <<= 1; if bv[i] {slen += 1;}
        }
        else if i < 104 + zlen * 8 + glen * sbitlen + glen + slen * sbitlen {
            c <<= 1; if bv[i] {c += 1;}
            if (i - (104 + zlen * 8 + glen * sbitlen + glen)) % sbitlen == sbitlen - 1 {g.sequence.push(c as u32); c = 0;}
        }
        else {d.push(bv[i]);}
    }
    delta::decode(&d, &mut v);
    let mut prev = 0;
    let mut rpos = glen;
    let mut ren = v[rpos];
    for i in 0..glen {
        if ren > 1 {
            if lr[i] {g.rule.push(vec![prev + v[i] - 1, prev + v[i] - u[i]]);}
            else {g.rule.push(vec![prev + v[i] - u[i], prev + v[i] - 1]);}
            ren -= 1;
            prev = prev + v[i] - 1;
        }
        else {
            if lr[i] {g.rule.push(vec![v[i], v[i] - u[i] + 1]);}
            else {g.rule.push(vec![v[i] - u[i] + 1, v[i]]);}
            rpos += 1;
            ren = v[rpos];
            prev = v[i];
        }
    }
    g.derive(w);
}

