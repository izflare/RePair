extern crate bit_vec;

use std::collections::HashMap;
use bit_vec::BitVec;
use super::encode;
use super::{ds::*};
use super::{cfg::*};

pub fn compression(s: &Vec<u8>, g: &mut Grammar, minfreq: usize, sorting: bool) -> () {

    // preprocessing
    let mut a: Vec<Bucket> = vec![Bucket {val: None, prev: None, next: None}; s.len()];
    let mut h: HashMap<Bigram, *mut Record> = HashMap::with_capacity(s.len());
    let mut f: usize = a.create(&mut h, &mut g.terminal, &s);
    let mut q: Vec<List> = vec![List {head: None, tail: None}; f + 1];
    q.create(&h);

    // algorithm
    let mut v: usize = g.terminal.len() + 1;
    let mut init: bool = true;
    loop {
        if f < minfreq {break;}
        if init && sorting {if let Some(_) = q.top(f) { unsafe {q.sort(f, &a); init = false;}}}
        if let Some(r) = q.top(f) { unsafe {
            // the most frequent bigram
            let bg: Bigram = a.rgh_bg((*r).loc).unwrap();
            q.dequeue(r);
            h.remove(&bg);
            g.rule.push(vec![bg.left, bg.right]);

            let mut il: usize = (*r).loc;
            let mut ir: usize = a.rgh_pos(il).unwrap();
            // decrement
            loop {
                //{{{
                // left bigram
                if let Some(lft_bg) = a.lft_bg(il) {
                    if let Some(lft_rec) = h.get(&lft_bg) {
                        let lft_rec = *lft_rec;
                        let lft_pos = a.lft_pos(il).unwrap();
                        if let Some(prev) = a[lft_pos].prev {a[prev].next = a[lft_pos].next;}
                        if let Some(next) = a[lft_pos].next {a[next].prev = a[lft_pos].prev;}
                        q.dequeue(lft_rec);
                        (*lft_rec).freq -= 1;
                        if (*lft_rec).freq < 2 {h.remove(&lft_bg);}
                        else {
                            if (*lft_rec).loc == lft_pos {(*lft_rec).loc = a[lft_pos].next.unwrap();}
                            q.enqueue(lft_rec);
                        }
                        a[lft_pos].prev = None;
                        a[lft_pos].next = None;
                    }
                }

                // right bigram
                if let Some(rgh_bg) = a.rgh_bg(ir) {
                    if let Some(rgh_rec) = h.get(&rgh_bg) {
                        let rgh_rec = *rgh_rec;
                        if let Some(prev) = a[ir].prev {a[prev].next = a[ir].next;}
                        if let Some(next) = a[ir].next {a[next].prev = a[ir].prev;}
                        q.dequeue(rgh_rec);
                        (*rgh_rec).freq -= 1;
                        if (*rgh_rec).freq < 2 {h.remove(&rgh_bg);}
                        else {
                            if (*rgh_rec).loc == ir {(*rgh_rec).loc = a[ir].next.unwrap();}
                            q.enqueue(rgh_rec);
                        }
                        a[ir].prev = None;
                        a[ir].next = None;
                    }
                }
                // replace bigram -> variable
                let jump = match a[il].next {Some(next) => a[next].next, None => None,};
                a[il].val = Some(v as u32);
                a[ir].val = None;
                a[il + 1].next = a.rgh_pos(ir);
                if let Some(rgh_ir) = a.rgh_pos(ir) {a[rgh_ir - 1].prev = Some(il);}
                if let Some(next) = a[il].next {
                    if next != ir {
                        il = next; ir = a.rgh_pos(il).unwrap();
                    }
                    else {
                        if let Some(skip) = jump {
                            a[il].next = Some(skip); a[skip].prev = Some(il);
                            il = skip; ir = a.rgh_pos(skip).unwrap();
                        }
                        else {break;}
                    }
                }
                else {break;}
                //}}}
            }

            // increment
            loop {
                //{{{
                // right bigram
                if let Some(rgh_bg) = a.rgh_bg(il) {
                    if let Some(rgh_rec) = h.get(&rgh_bg) {
                        let rgh_rec = *rgh_rec;
                        if (*rgh_rec).loc != il {
                            a[il].next = Some((*rgh_rec).loc);
                            a[(*rgh_rec).loc].prev = Some(il);
                            q.dequeue(rgh_rec);
                            (*rgh_rec).freq += 1;
                            (*rgh_rec).loc = il;
                            q.enqueue(rgh_rec);
                        }
                    }
                    else {
                        let rgh_rec = Box::into_raw(Box::new(Record {loc: il, freq: 1, prev: None, next: None}));
                        h.insert(rgh_bg, rgh_rec);
                        q.enqueue(rgh_rec);
                        a[il].next = None;
                    }
                }

                // left bigram
                if let Some(lft_bg) = a.lft_bg(il) {
                    let lft_pos = a.lft_pos(il).unwrap();
                    if let Some(lft_rec) = h.get(&lft_bg) {
                        let lft_rec = *lft_rec;
                        a[lft_pos].next = Some((*lft_rec).loc);
                        a[(*lft_rec).loc].prev = Some(lft_pos);
                        q.dequeue(lft_rec);
                        (*lft_rec).freq += 1;
                        (*lft_rec).loc = lft_pos;
                        q.enqueue(lft_rec);
                    }
                    else {
                        let lft_rec = Box::into_raw(Box::new(Record {loc: lft_pos, freq: 1, prev: None, next: None}));
                        h.insert(lft_bg, lft_rec);
                        q.enqueue(lft_rec);
                        a[lft_pos].next = None;
                    }
                }

                // go to prev occ
                if let Some(prev) = a[il].prev {
                    let old = il;
                    il = prev;
                    a[old].prev = None;
                }
                else {break;}
                //}}}
            }

            v += 1;
            if v >= std::u32::MAX as usize {break;}
        }}
        else {f -= 1; init = true;}
    }

    for c in &a {match (*c).val {Some(x) => g.sequence.push(x), None => ()}}
}

pub fn decompression(bv: &BitVec, u: &mut Vec<u8>) -> () {
    encode::decode(bv, u);
}
