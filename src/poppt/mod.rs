extern crate bit_vec;

use bit_vec::BitVec;
use std::time::Instant;

pub fn encode(z: &Vec<u8>, g: &Vec<(u32, u32)>, s: &Vec<u32>, bv: &mut BitVec) -> () {
    //{{{
    let start = Instant::now();

    let mut b: BitVec = BitVec::new();
    let mut l: Vec<u32> = Vec::new();
    let mut i: usize = z.len() + 1;
    let mut f: Vec<Option<u32>> = Vec::new();
    for _ in 0..g.len() {f.push(None);}
    fn drv(x: u32, z: &Vec<u8>, g: &Vec<(u32, u32)>, b: &mut BitVec, 
           l: &mut Vec<u32>, i: &mut usize, f: &mut Vec<Option<u32>>) -> () {
        if x as usize <= z.len() {
            l.push(x);
            b.push(false);
        }
        else {
            if let Some(y) = f[x as usize - z.len() - 1] {
                l.push(y as u32);
                b.push(false);
            }
            else {
                let bg = g[x as usize - z.len() - 1];
                drv(bg.0, z, g, b, l, i, f);
                drv(bg.1, z, g, b, l, i, f);
                if x as usize > z.len() {
                    f[x as usize - z.len() - 1] = Some(*i as u32);
                    *i += 1;
                }
                b.push(true);
            }
        }
    }

    let mut r: bool = false;
    for x in s {
        drv(*x, z, g, &mut b, &mut l, &mut i, &mut f);
        if r {b.push(true); i += 1;} 
        r = true;
    }
    b.push(true);

    let logn = std::usize::MAX.count_ones() - l.len().leading_zeros();
    fn u_to_bv(x: u32, logn: u32, bv: &mut BitVec) -> () {
        //{{{
        let mut z = x;
        z = z.rotate_right(logn);
        for _ in 0..logn {
            z = z.rotate_left(1);
            bv.push(z % 2 == 1);
        }
        //}}}
    }
    for bit in &b {bv.push(bit);}
    for e in z {u_to_bv(*e as u32, 8, bv);}
    u_to_bv(0, 8, bv);
    u_to_bv(logn, 8, bv);
    for e in &l {u_to_bv(*e, logn, bv);}
    let end = start.elapsed();

    println!("[Result: bit encoding]");
    println!("B length          : {:?} [bits]", b.len());
    println!("L length          : {:?} [words]", l.len());
    println!("log (n + sigma)   : {:?}", logn);
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

    //}}}
}

pub fn decode(bv: &BitVec, w: &mut Vec<u8>) -> () {
    //{{{
    
    let mut mode = 1;
    let mut t = 0;
    let mut i = 0;
    let mut b: BitVec = BitVec::new();
    let mut u: u32 = 0;
    let mut z: Vec<u8> = Vec::new();
    let mut logn: u32 = 0;
    let mut l: Vec<u32> = Vec::new();
    for bit in bv {
        if mode == 1 {
            b.push(bit);
            if bit {t -= 1;} else {t += 1;}
            if t == 0 {mode = 2;}
        }
        else if mode == 2 {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= 8 {
                if u == 0 {mode = 3; i = 0;}
                else {z.push(u as u8); u = 0; i = 0;}
            }
        }
        else if mode == 3 {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= 8 {logn = u as u32; u = 0; mode = 4; i = 0;}
        }
        else {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= logn {l.push(u as u32); u = 0; i = 0;}
        }
    }
    
    let mut dec_g: Vec<(u32, u32)> = Vec::new();
    fn dec_drv(x: u32, dec_g: &Vec<(u32, u32)>, z: &Vec<u8>, w: &mut Vec<u8>) -> () {
        if x as usize <= z.len() {w.push(z[x as usize -1]);}
        else {
            let bg = dec_g[x as usize - z.len() -1];
            dec_drv(bg.0, dec_g, z, w);
            dec_drv(bg.1, dec_g, z, w);
        }
    }

    let mut dec_i = 0;
    let mut dec_x = z.len() as u32 + 1;
    let mut stack: Vec<u32> = Vec::new();
    for dec_b in &b {
        if dec_b {
            if let Some(rt) = stack.pop() {
                if let Some(lt) = stack.pop() {
                    dec_g.push((lt, rt));
                }
            }
            stack.push(dec_x);
            dec_x += 1;
        }
        else {
            stack.push(l[dec_i]);
            dec_drv(l[dec_i], &dec_g, &z, w);
            dec_i += 1;
        }
    }
    //}}}
}

