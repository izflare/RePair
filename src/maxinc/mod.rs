extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::time::Instant;
use strlib::delta;

pub fn encode(z: &Vec<u8>, g: &Vec<(u32, u32)>, s: &Vec<u32>, bv: &mut BitVec) -> () {

    let start = Instant::now();

    fn u_to_bv(x: u32, logn: u32, bv: &mut BitVec) -> () {
        let mut z = x;
        z = z.rotate_right(logn);
        for _ in 0..logn {
            z = z.rotate_left(1);
            bv.push(z % 2 == 1);
        }
    }
    let mut v: Vec<u32> = Vec::new();

    let mut prev = 0;
    let mut lr = BitVec::new();
    let mut r: Vec<u32> = Vec::new();
    let mut u = Vec::new();
    let mut ren = 1;
    for e in g {
        let m = std::cmp::max((*e).0, (*e).1);
        if prev <= m {
            v.push(m - prev + 1);
            ren += 1;
        }
        else {
            v.push(prev - m + 1);
            r.push(ren);
            ren = 1;
        }
        lr.push((*e).0 >= (*e).1);
        prev = m;
        u.push(std::cmp::min((*e).0, (*e).1));
    }
    r.push(ren);
    let sbitlen = std::usize::MAX.count_ones() - (z.len() + g.len()).leading_zeros();

    u_to_bv(z.len() as u32, 8, bv);
    for e in z {u_to_bv(*e as u32, 8, bv);}
    u_to_bv(g.len() as u32, 32, bv);
    u_to_bv(sbitlen as u32, 32, bv);
    for e in &u {u_to_bv(*e, sbitlen, bv);}
    for bit in &lr {bv.push(bit);}
    u_to_bv(s.len() as u32, 32, bv);
    for e in s {u_to_bv(*e, sbitlen, bv);}
    delta::encode(&v, bv);
    delta::encode(&r, bv);
    let end = start.elapsed();

    println!("[Result: bit encoding]");
    println!("Increasing seq    : {:?}", r.len());
    println!("Bit length        : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
}

pub fn decode(bv: &mut BitVec, w: &mut Vec<u8>) -> () {
    let mut v: Vec<u32> = Vec::new();
    let mut zlen = 0;
    let mut z: Vec<u8> = Vec::new();
    let mut c: u32 = 0;
    let mut glen = 0;
    let mut sbitlen = 0;
    let mut u: Vec<u32> = Vec::new();
    let mut lr: BitVec = BitVec::new();
    let mut slen = 0;
    let mut s: Vec<u32> = Vec::new();
    let mut d: BitVec = BitVec::new();

    for i in 0..bv.len() {
        if i < 8 {
            zlen <<= 1; if bv[i] {zlen += 1;}
        }
        else if i < 8 + zlen * 8 {
            c <<= 1; if bv[i] {c += 1;}
            if i % 8 == 7 {z.push(c as u8); c = 0;}
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
            if (i - (104 + zlen * 8 + glen * sbitlen + glen)) % sbitlen == sbitlen - 1 {s.push(c as u32); c = 0;}
        }
        else {d.push(bv[i]);}
    }
    delta::decode(&d, &mut v);
    let mut prev = 0;
    let mut rpos = glen;
    let mut ren = v[rpos];
    let mut g: Vec<(u32, u32)> = Vec::new();
    for i in 0..glen {
        if ren > 1 {
            if lr[i] {g.push((prev + v[i] - 1, u[i]));}
            else {g.push((u[i], prev + v[i] - 1));}
            ren -= 1;
            prev = prev + v[i] - 1;
        }
        else {
            if lr[i] {g.push((prev - v[i] + 1, u[i]));}
            else {g.push((u[i], prev - v[i] + 1));}
            rpos += 1;
            ren = v[rpos];
            prev = prev - v[i] + 1;
        }
    }

    fn drv(i: usize, z: &Vec<u8>, g: &Vec<(u32, u32)>, w: &mut Vec<u8>) -> () {
        if i < z.len() + 1 {
            w.push(z[i - 1]);
        }
        else {
            let bg = g[i - z.len() - 1];
            drv(bg.0 as usize, z, g, w);
            drv(bg.1 as usize, z, g, w);
        }
    }

    for e in &s {
        drv(*e as usize, &z, &g, w);
    }
}

