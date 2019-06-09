extern crate clap;
extern crate bit_vec;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::time::Instant;
use bit_vec::BitVec;
use rp::encode;
use rp::comp;
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

    // read
    let mut s = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");

    // compression
    if !matches.is_present("decompress") {
        let start = Instant::now();

        let minfreq = 
                std::cmp::max(2, match matches.value_of("minfreq") {Some(x) => (*x).parse::<usize>().unwrap(), None => 3,});
        let mut g: Grammar = Grammar::new();
        comp::compression(&s, &mut g, &minfreq);

        let end = start.elapsed();


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

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".rp").unwrap());
        f.write(&bv.to_bytes()).unwrap();

        println!("[Result: compression]");
        //{{{
        println!("Input data        : {:?} [bytes]", s.len());
        println!("Compressed data   : {:?} [bytes]", bv.len() / 8 + if bv.len() % 8 > 0 {1} else {0});
        println!("Compression ratio : {:.3} [%]", 100.0 * bv.len() as f64 / 8.0 / s.len() as f64);
        if matches.is_present("print") {
            println!("\n[Grammar detail]");
            println!("Alphabet   :\n {:?}", g.terminal);
            println!("Dictionary :\n {:?}", g.rule);
            println!("Sequence   :\n {:?}", g.sequence);
        }
        //}}}

    }

    // decompression
    else {
        let bv: BitVec = BitVec::from_bytes(&s);
        let mut u: Vec<u8> = Vec::new();
        comp::decompression(&bv, &mut u);

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".dcp").unwrap());
        f.write(&u).unwrap();
    }

}
