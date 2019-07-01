## RePair

### Description

This is an implementation of RePair, a famous grammar compression method proposed in

> N. Jesper Larsson and Alistair Moffat: _Off-line dictionary-based compression._ Proceedings of the IEEE, 88(11):1722-1732, 2000.


### Download and Compile

This code has been tested under linux compiling with rust (cargo) ver 1.35.0.

```
git clone https://github.com/izflare/RePair.git
cd RePair
cargo build --release
```

### Run

```
USAGE:
    cd target/release
    ./rp [FLAGS] [OPTIONS] --input <FILE> <-c|-d>

FLAGS:
    -c               Compression mode
    -d               Decompression mode
    -p, --print      Prints the detail of constructed grammar
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>    Input sourse text file
    -m, --minfreq <INTEGER>    Sets minimum frequency of pairing operation [default: 2]
    -e, --encode <MODE>        Sets encoding mode [default: sorting]  
                               [possible values: u32bits, fixed, sorting]
```

The command with `-c` flag produces the compressed file `<FILE>.rp`.  
The command with `-d` flag produces the decompressed file `<FILE>.d`.

