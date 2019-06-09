## RePair

### Description

This is an implementation of RePair, a famous grammar compression method proposed in

> N. Jesper Larsson and Alistair Moffat: _Off-line dictionary-based compression._ Proceedings of the IEEE, 88(11):1722-1732, 2000.


### Download

```
git clone https://github.com/izflare/RePair.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.34.0.  

```
cd RePair
cargo build --release
```

### Run

```
USAGE:
    ./target/release/rp [FLAGS] [OPTIONS] --input <input> <-c|-d>

FLAGS:
    -c               Compression mode
    -d               Decompression mode
    -h, --help       Prints help information
    -p, --print      Print the detail of constructed grammar
    -s, --sort       Enable bigram sorting
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input sourse text file
    -m, --min <minfreq>    Set minimum frequency of pairing operation (default: 3)

OUTPUTS:
    <input>.rp   Compressed file
    <input>.d    Decompressed file
```


