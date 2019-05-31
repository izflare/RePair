extern crate bit_vec;
extern crate hc;
use bit_vec::BitVec;
use hc::huffman_coding;

pub fn encode(z: &Vec<u8>, g: &Vec<(u32, u32)>, s: &Vec<u32>, bv: &mut BitVec) -> () {
    //{{{

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

    fn u32_to_bv(x: u32, bv: &mut BitVec) -> () {
        //{{{
        let mut z = x;
        for _ in 0..32 {
            z = z.rotate_left(1);
            bv.push(z % 2 == 1);
        }
        //}}}
    }
    for _ in 0..(7 - (b.len() % 8)) {bv.push(false);}
    bv.push(true);
    u32_to_bv(b.len() as u32, bv);
    for bit in &b {bv.push(bit);}

    let mut w: Vec<u32> = Vec::new();
    for e in z {w.push(*e as u32);}
    w.push(0);
    w.append(&mut l);
    let mut encoded: BitVec = BitVec::new();
    huffman_coding::encode(&w, &mut encoded);
    for bit in &encoded {bv.push(bit);}
    //}}}
}

pub fn decode(bv: &BitVec, u: &mut Vec<u8>) -> () {
    //{{{
    
    let mut mode = 0;
    let mut t = 0;
    let mut i = 0;
    let mut b: BitVec = BitVec::new();
    let mut w: Vec<u32> = Vec::new();
    let mut d: BitVec = BitVec::new();
    for bit in bv {
        if mode == 0 && bit {mode = 1;}
        else if mode == 1 {
            if i < 32 {t <<= 1; if bit {t += 1;} i += 1;}
            else {
                if t > 0 {b.push(bit); t -= 1;}
                else {mode = 2;}
            }
        }
        else {d.push(bit);}
    }
    huffman_coding::decode(&d, &mut w);

    let mut z: Vec<u8> = Vec::new();
    for e in &w {if *e == 0 {break;} z.push(*e as u8)}
    let mut dec_g: Vec<(u32, u32)> = Vec::new();
    fn dec_drv(x: u32, dec_g: &Vec<(u32, u32)>, z: &Vec<u8>, u: &mut Vec<u8>) -> () {
        if x as usize <= z.len() {u.push(z[x as usize -1]);}
        else {
            let bg = dec_g[x as usize - z.len() -1];
            dec_drv(bg.0, dec_g, z, u);
            dec_drv(bg.1, dec_g, z, u);
        }
    }

    let mut dec_i = z.len() + 1;
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
            stack.push(w[dec_i]);
            dec_drv(w[dec_i], &dec_g, &z, u);
            dec_i += 1;
        }
    }
    //}}}
}

