extern crate clap;
extern crate bit_vec;
extern crate rp;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use bit_vec::BitVec;
use rp::gamma;

fn main() {

    // 引数処理
    let app = App::new("RePair")
        //{{{
        .version("0.1.0")                       
        .author("flare")     
        .about("RePair compressor")
        .arg(Arg::with_name("input")
            .help("input sourse text file") 
            .short("i")
            .long("input")
            .takes_value(true)                  
            .required(true)                     
        )
        .arg(Arg::with_name("decompress")
            .help("decompression")
            .short("d")
            .long("dcp")
        )
        .arg(Arg::with_name("minfreq")
            .help("Minimum frequency")
            .short("m")
            .long("min")
            .takes_value(true)
        )
        .arg(Arg::with_name("print")
            .help("print the detail of constructed grammar")
            .short("p")
            .long("print")
        );
        //}}}
    let matches = app.get_matches();

    // 読み込み
    let mut s = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");

    if !matches.is_present("decompress") {
        let start = Instant::now();

        // preprocessing
        // 終端記号を変数に置換して，文字列を配列に格納
        // each tuple is (0: char, 1: prev, 2: next)
        let mut a: Vec<(Option<u32>, Option<usize>, Option<usize>)> = vec![(None, None, None); s.len()]; 
        let mut z: Vec<u8> = Vec::new();
        //{{{
        {
            // let mut d: HashMap<char, usize> = HashMap::new();
            let mut d: HashMap<u8, u32> = HashMap::new();
            let mut x = 1;
            for i in 0..s.len() {
                if d.contains_key(&s[i]) {
                    let e = d.get(&s[i]);
                    a[i] = (Some(*e.unwrap()), None, None);
                }
                else {
                    d.insert(s[i], x);
                    a[i] = (Some(x), None, None);
                    x += 1;
                    z.push(s[i]);
                }
            }
        }

        // a の関数
        // 右隣の空でない要素の番号を取得
        fn get_rt(a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>, i: usize) -> usize {
            //{{{
            if a[i+1].0 == None {
                match a[i+1].2 {Some(x) => x, None => 0}
            }
            else {
                i+1
            }
            //}}}
        }

        // 左隣の空でない要素の番号を取得
        fn get_lt(a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>, i: usize) -> usize {
            //{{{
            if a[i-1].0 == None {
                a[i-1].1.unwrap()
            }
            else {
                i-1
            }
            //}}}
        }

        // bigramを取得
        fn get_bg(a: &Vec<(Option<u32>, Option<usize>, Option<usize>)>, i: usize) -> (u32, u32) {
            //{{{
            (a[i].0.unwrap(), a[get_rt(&a, i)].0.unwrap())
            //}}}
        }

        //}}}

        // bigramの位置をつなぎながらハッシュ表を作成
        struct Rec {loc: usize, freq: usize, prev: Option<*mut Rec>, next: Option<*mut Rec>};
        let mut h: HashMap<(u32, u32), *mut Rec> = HashMap::new();
        let mut f: usize = 1;
        let mut k: Vec<(u32, u32)> = Vec::new();
        //{{{
        for i in (0..s.len()-1).rev() {
            let b = (a[i].0.unwrap(), a[i+1].0.unwrap());
            if h.contains_key(&b) {
                unsafe {
                    let mut r: &mut Rec = &mut **(h.get(&b).unwrap());
                    a[i].2 = Some(r.loc);
                    a[r.loc].1 = Some(i);
                    r.loc = i;
                    r.freq += 1;
                    if f < r.freq {f = r.freq;}
                }
            }
            else {
                let r = Box::new(Rec {loc: i, freq: 1, prev: None, next: None});
                let x: *mut Rec = Box::into_raw(r);
                h.insert(b, x);
                k.push(b);
            }
        }
        //}}}

        // 頻度表を作成
        // (head, tail)
        let mut q: Vec<(Option<*mut Rec>, Option<*mut Rec>)> = vec![(None, None); f+1];
        //{{{
        for e in &k {
            let v = h.get(e).unwrap();
            unsafe {
                let r: &mut Rec = &mut **v;
                in_rec(&mut q, r);
            }
        }

        // q の関数
        // Record をリストから切り離す
        fn out_rec(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, r: &mut Rec) {
            //{{{
            if r.prev == None {
                q[r.freq].0 = r.next;
            }
            else {
                unsafe {
                    let pr: &mut Rec = &mut *r.prev.unwrap();
                    pr.next = r.next;
                }
            }

            if r.next == None {
                q[r.freq].1 = r.prev;
            }
            else {
                unsafe {
                    let nx: &mut Rec = &mut *r.next.unwrap();
                    nx.prev = r.prev;
                }
            }
            r.prev = None;
            r.next = None;
            //}}}
        }

        // Record をリストの末尾に追加
        fn in_rec(q: &mut Vec<(Option<*mut Rec>, Option<*mut Rec>)>, r: &mut Rec) {
            //{{{
            let ptr: *mut Rec = &mut *r;
            if q[r.freq].1 != None {
                unsafe {
                    let tail: &mut Rec = &mut *q[r.freq].1.unwrap();
                    tail.next = Some(ptr);
                    r.prev = Some(tail);
                }
                r.next = None;
            }
            else {
                q[r.freq].0 = Some(ptr);
                r.prev = None;
            }
            q[r.freq].1 = Some(ptr);
            //}}}
        }
        //}}}

        // algorithm
        let mut v: usize = z.len() + 1;
        let mut g: Vec<(u32, u32)> = Vec::new();

        while f >= std::cmp::max(2, match matches.value_of("minfreq") {Some(x) => (*x).parse::<usize>().unwrap(), None => 2,}) {
            if q[f].0 == None {f -= 1; continue;}
            unsafe {
                // 最頻出ペアを同定
                let mut r: &mut Rec = &mut *q[f].0.unwrap();
                let b = get_bg(&a, r.loc);
                out_rec(&mut q, &mut r);
                g.push(b);

                // 置換・更新，順方向，既存ペアのデクリメント
                let mut i: usize = r.loc;
                let mut o: bool = false;
                loop {
                    //{{{
                    let rt_i = get_rt(&a, i);
                    // 右隣のペアの頻度をデクリメント
                    if i > 0 && !o {
                        //{{{
                        let lt_i = get_lt(&a, i);
                        let lt_b: (u32, u32) = get_bg(&a, lt_i);
                        let mut lt_r: &mut Rec = &mut **h.get(&lt_b).unwrap();
                        match a[lt_i].1 {Some(x) => a[x].2 = a[lt_i].2, None => ()}
                        match a[lt_i].2 {Some(x) => a[x].1 = a[lt_i].1, None => ()}
                        out_rec(&mut q, &mut lt_r);
                        lt_r.freq -= 1;
                        if lt_r.freq > 0 && lt_r.loc == lt_i {lt_r.loc = a[lt_i].2.unwrap()}
                        if lt_r.freq > 0 {in_rec(&mut q, &mut lt_r);}
                        else {h.remove(&lt_b);}
                        //}}}
                    }

                    // 左隣のペアの頻度をデクリメント
                    if i < a.len()-1 && rt_i != 0 && rt_i < a.len()-1 && get_rt(&a, rt_i) != 0 {
                        //{{{
                        let rt_b: (u32, u32) = get_bg(&a, rt_i);
                        match a[i].2 {
                            Some(x) => {
                                // fully overlap
                                if x == rt_i {
                                    let nx_rt_i = a[rt_i].2;
                                    a[i].2 = nx_rt_i;
                                    match nx_rt_i {
                                        Some(x) => {
                                            a[x].1 = Some(i);
                                            o = get_rt(&a, rt_i) == x;
                                        }, 
                                        None => {o = false;}
                                    }
                                }
                                else {
                                    let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                                    match a[rt_i].1 {Some(x) => a[x].2 = a[rt_i].2, None => ()}
                                    match a[rt_i].2 {Some(x) => a[x].1 = a[rt_i].1, None => ()}
                                    out_rec(&mut q, &mut rt_r);
                                    rt_r.freq -= 1;
                                    if rt_r.freq > 0 && rt_r.loc == rt_i {rt_r.loc = a[rt_i].2.unwrap()}
                                    if rt_r.freq > 0 {in_rec(&mut q, &mut rt_r);}
                                    else {h.remove(&rt_b);}
                                    // consider partially overlap
                                    o = x == get_rt(&a, rt_i);
                                }
                            },
                            None => {
                                let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                                match a[rt_i].1 {Some(x) => a[x].2 = a[rt_i].2, None => ()}
                                match a[rt_i].2 {Some(x) => a[x].1 = a[rt_i].1, None => ()}
                                out_rec(&mut q, &mut rt_r);
                                rt_r.freq -= 1;
                                if rt_r.freq > 0 && rt_r.loc == rt_i {rt_r.loc = a[rt_i].2.unwrap()}
                                if rt_r.freq > 0 {in_rec(&mut q, &mut rt_r);}
                                else {h.remove(&rt_b);}
                                o = false;
                            }
                        }
                        let nx_rt_i = get_rt(&a, rt_i);
                        if nx_rt_i != 0 {
                            a[nx_rt_i-1].1 = Some(i);
                            a[i+1].2 = Some(nx_rt_i);
                        }
                    }
                    else {
                        a[i+1].2 = None;
                        o = false;
                        //}}}
                    }

                    a[i].0 = Some(v as u32);
                    a[rt_i].0 = None;
                    if a[i].2 == None {break;}
                    i = a[i].2.unwrap();
                //}}}
                }

                // 置換・更新，逆方向，新出ペアのインクリメント
                o = false;
                loop {
                    //{{{
                    // 右隣のペアの頻度をインクリメント
                    if i < a.len()-1 && get_rt(&a, i) != 0 && !o {
                        //{{{
                        let rt_b: (u32, u32) = get_bg(&a, i);
                        if h.contains_key(&rt_b) {
                            let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                            a[rt_r.loc].1 = Some(i);
                            a[i].2 = Some(rt_r.loc);
                            rt_r.loc = i;
                            out_rec(&mut q, &mut rt_r);
                            rt_r.freq += 1;
                            in_rec(&mut q, &mut rt_r);
                        }
                        else {
                            let mut new_r = Box::new(Rec {loc: i, freq: 1, prev: None, next: None});
                            in_rec(&mut q, &mut new_r);
                            let x: *mut Rec = Box::into_raw(new_r);
                            h.insert(rt_b, x);
                            a[i].2 = None;
                        }
                        //}}}
                    }

                    // 左隣のペアの頻度をインクリメント
                    let mut pair_overlap = false;
                    if i > 0 {
                        //{{{
                        let lt_i = get_lt(&a, i);
                        o = match a[i].1 {Some(x) => if x == lt_i {true} else {false}, None => false};
                        if o && get_bg(&a, lt_i) == get_bg(&a, i) {pair_overlap = true;}
                        let lt_b: (u32, u32) = get_bg(&a, lt_i);
                        if h.contains_key(&lt_b) {
                            let mut lt_r: &mut Rec = &mut **h.get(&lt_b).unwrap();
                            a[lt_r.loc].1 = Some(lt_i);
                            if !o {a[lt_i].1 = None;}
                            a[lt_i].2 = Some(lt_r.loc);
                            out_rec(&mut q, &mut lt_r);
                            lt_r.freq += 1;
                            lt_r.loc = lt_i;
                            in_rec(&mut q, &mut lt_r);
                        }
                        else {
                            let mut new_r = Box::new(Rec {loc: lt_i, freq: 1, prev: None, next: None});
                            in_rec(&mut q, &mut new_r);
                            let x: *mut Rec = Box::into_raw(new_r);
                            h.insert(lt_b, x);
                            if !o {a[lt_i].1 = None;}
                            a[lt_i].2 = None;
                        }
                        //}}}
                    }

                    if a[i].1 == None {break;}
                    let ii = i;
                    i = a[i].1.unwrap();
                    if !pair_overlap {a[ii].1 = None;}
                //}}}
                }

                v += 1;
                if v >= std::u32::MAX as usize {break;}
                h.remove(&b);
            }
        }

        let end = start.elapsed();
        let mut s: Vec<u32> = Vec::new();
        for c in &a {match (*c).0 {Some(x) => s.push(x), None => ()}}

        // print
        println!("[Result: grammar construction]");
        println!("Alphabet size     : {:?}", z.len());
        println!("Rule number       : {:?}", g.len());
        println!("Dictionary size   : {:?}", g.len() * 2);
        println!("Sequence length   : {:?}", s.len());
        println!("Total size        : {:?}", z.len() + g.len() * 2 + s.len());
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

        // encode
        let mut bv: BitVec = BitVec::new();
        gamma::encode(&z, &g, &s, &mut bv);
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".rp").unwrap());
        f.write(&bv.to_bytes()).unwrap();

        println!("[Result: compression]");
        println!("Input data        : {:?} [bytes]", a.len());
        println!("Compressed data   : {:?} [bytes]", bv.len() / 8 + if bv.len() % 8 > 0 {1} else {0});
        println!("Compression ratio : {:.3} [%]", 100.0 * bv.len() as f64 / 8.0 / a.len() as f64);
        if matches.is_present("print") {
            println!("\n[Grammar detail]");
            println!("Alphabet   :\n {:?}", z);
            println!("Dictionary :\n {:?}", g);
            println!("Sequence   :\n {:?}", s);
        }

    }
    else {
        let mut bv: BitVec = BitVec::from_bytes(&s);
        let mut u: Vec<u8> = Vec::new();
        gamma::decode(&mut bv, &mut u);

        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".dcp").unwrap());
        f.write(&u).unwrap();
    }

}
