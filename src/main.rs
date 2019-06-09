extern crate clap;
extern crate bit_vec;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use bit_vec::BitVec;
use rp::encode;
use rp::{ds::*};
use rp::{cfg::*};

fn main() {

    // args
    let app = App::new("RePair")
        //{{{
        .version("0.1.1")
        .author("flare")
        .about("RePair compress/decompressor")
        .arg(Arg::with_name("input")
            .help("Input sourse text file")
            .short("i")
            .long("input")
            .takes_value(true)                  
            .required(true)                     
        )
        .arg(Arg::with_name("decompress")
            .help("Decompress")
            .short("d")
            .long("dcp")
        )
        .arg(Arg::with_name("minfreq")
            .help("Set minimum frequency of pairing operation (default: 3)")
            .short("m")
            .long("min")
            .takes_value(true)
        )
        .arg(Arg::with_name("print")
            .help("Print the detail of constructed grammar")
            .short("p")
            .long("print")
        );
        //}}}
    let matches = app.get_matches();

    // file io
    let mut s = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");

    if !matches.is_present("decompress") {
        let start = Instant::now();

        // preprocessing
        let mut a: Vec<Bucket> = vec![Bucket {val: None, prev: None, next: None}; s.len()];
        let mut h: HashMap<Bigram, *mut Record> = HashMap::with_capacity(s.len());
        let mut g: Grammar = Grammar::new();
        let mut f: usize = a.create(&mut h, &mut g.terminal, &s);
        let mut q: Vec<List> = vec![List {head: None, tail: None}; f + 1];
        q.create(&h);
        let minfreq = 
            std::cmp::max(2, match matches.value_of("minfreq") {Some(x) => (*x).parse::<usize>().unwrap(), None => 3,});

        // algorithm
        let mut v: usize = g.terminal.len() + 1;
        loop {
            if f < minfreq {break;}
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
            else {f -= 1;}
        }

        let end = start.elapsed();
        for c in &a {match (*c).val {Some(x) => g.sequence.push(x), None => ()}}


        println!("[Result: grammar construction]");
        //{{{
        println!("Alphabet size     : {:?}", g.terminal.len());
        println!("Rule number       : {:?}", g.rule.len());
        println!("Dictionary size   : {:?}", g.rule.len() * 2);
        println!("Sequence length   : {:?}", g.sequence.len());
        println!("Total size        : {:?}", g.terminal.len() + g.rule.len() * 2 + g.sequence.len());
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
        //}}}

        // encode
        let mut bv: BitVec = BitVec::new();
        encode::encode(&g, &mut bv);
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".rp").unwrap());
        f.write(&bv.to_bytes()).unwrap();

        println!("[Result: compression]");
        //{{{
        println!("Input data        : {:?} [bytes]", a.len());
        println!("Compressed data   : {:?} [bytes]", bv.len() / 8 + if bv.len() % 8 > 0 {1} else {0});
        println!("Compression ratio : {:.3} [%]", 100.0 * bv.len() as f64 / 8.0 / a.len() as f64);
        if matches.is_present("print") {
            println!("\n[Grammar detail]");
            println!("Alphabet   :\n {:?}", g.terminal);
            println!("Dictionary :\n {:?}", g.rule);
            println!("Sequence   :\n {:?}", g.sequence);
        }
        //}}}

    }
    else {
        let mut bv: BitVec = BitVec::from_bytes(&s);
        let mut u: Vec<u8> = Vec::new();
        encode::decode(&mut bv, &mut u);

        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".dcp").unwrap());
        f.write(&u).unwrap();
    }

}
