#[macro_use]
extern crate clap;
extern crate bit_vec;

use clap::{App, Arg, ArgGroup, AppSettings};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::time::Instant;
use bit_vec::BitVec;
use rp::module::compress;
use rp::module::{cfg::*};
use rp::module::encode;

fn main() {

    // args
    let app = App::new("RePair")
        //{{{
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::DeriveDisplayOrder)
        .args_from_usage("-c 'Compression mode'
                         -d 'Decompression mode'")
        .group(ArgGroup::with_name("mode").args(&["c", "d"]).required(true))
        .arg(Arg::from_usage("-i --input [FILE] 'Input sourse file'").required(true))
        .arg(Arg::from_usage("-m --minfreq [INTEGER] 'Sets minimum frequency of pairing operation'").default_value("2"))
        .arg(Arg::from_usage("-e --encode [MODE] 'Sets encoding mode'")
             .possible_values(&["u32bits", "fixed", "sorting"])
             .default_value("sorting"))
        .arg(Arg::from_usage("-p --print 'Prints the detail of constructed grammar'"));
        //}}}
    let matches = app.get_matches();

    // read
    let mut s = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");

    // compression
    if matches.is_present("c") {
        let start = Instant::now();

        let minfreq = 
                std::cmp::max(2, match matches.value_of("minfreq") {Some(x) => (*x).parse::<usize>().unwrap(), None => 2,});
        let mode = matches.value_of("encode").unwrap();

        let mut g: Grammar = Grammar::new();
        compress::compress(&s, &mut g, minfreq, mode == "sorting");

        let end = start.elapsed();
        println!("[Grammar construction]");
        //{{{
        println!("Alphabet size        : {:?}", g.terminal.len());
        println!("Rule number          : {:?}", g.rule.len());
        println!("Dictionary size      : {:?}", g.rule.len() * 2);
        println!("Sequence length      : {:?}", g.sequence.len());
        println!("Total grammar size   : {:?}", g.terminal.len() + g.rule.len() * 2 + g.sequence.len());
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
        //}}}

        // encode
        let mut bv: BitVec = BitVec::new();
        encode::encode(&g, mode, &mut bv);

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".rp").unwrap());
        f.write(&bv.to_bytes()).unwrap();

        println!("[Compression]");
        //{{{
        println!("Input data           : {:?} [bytes]", s.len());
        println!("Compressed data      : {:?} [bytes]", bv.len() / 8 + if bv.len() % 8 > 0 {1} else {0});
        println!("Compression ratio    : {:.3} [%]", 100.0 * bv.len() as f64 / 8.0 / s.len() as f64);
        if matches.is_present("print") {
            println!("\n[Grammar detail]");
            println!("Alphabet   :\n {:?}", g.terminal);
            println!("Dictionary :\n {:?}", g.rule);
            println!("Sequence   :\n {:?}", g.sequence);
        }
        //}}}

    }

    // decompression
    else if matches.is_present("d") {

        println!("[Decompression]");
        let start = Instant::now();

        let bv: BitVec = BitVec::from_bytes(&s);
        let mut g: Grammar = Grammar::new();
        encode::decode(&bv, &mut g);

        let mut u: Vec<u8> = Vec::new();
        g.derive(&mut u);

        let end = start.elapsed();
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".d").unwrap());
        f.write(&u).unwrap();
    }
    else {
        panic!("mode error");
    }

}
