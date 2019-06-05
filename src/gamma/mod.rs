extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::time::Instant;
use strlib::gamma;

pub fn encode(z: &Vec<u8>, g: &Vec<(u32, u32)>, s: &mut Vec<u32>, bv: &mut BitVec) -> () {
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
    for e in z {v.push(*e as u32);}
    for e in g {v.push((*e).0); v.push((*e).1);}
    v.append(s);

    u_to_bv(z.len() as u32, 8, bv);
    u_to_bv(g.len() as u32, 32, bv);
    gamma::encode(&v, bv);
    let end = start.elapsed();

    println!("[Result: bit encoding]");
    println!("Bit length        : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

}

pub fn decode(bv: &mut BitVec, w: &mut Vec<u8>) -> () {
    let mut v: Vec<u32> = Vec::new();
    let mut zlen = 0;
    let mut glen = 0;
    let mut d: BitVec = BitVec::new();

    for i in 0..bv.len() {
        if i < 8 {
            zlen <<= 1; if bv[i] {zlen += 1;}
        }
        else if i < 40 {
            glen <<= 1; if bv[i] {glen += 1;}
        }
        else {d.push(bv[i]);}
    }
    gamma::decode(&d, &mut v);

    let mut z: Vec<u8> = Vec::new();
    let mut g: Vec<(u32, u32)> = Vec::new();
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

    for i in 0..v.len() {
        if i < zlen {
            z.push(v[i] as u8);
        }
        else if i < zlen + glen * 2 {
            if (i - zlen) % 2 == 0 {
                g.push((v[i] as u32, v[i + 1] as u32));
            }
            else {continue;}
        }
        else {
            drv(v[i] as usize, &z, &g, w);
        }
    }
}

