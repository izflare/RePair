# RePair (ver 0.1.1)

### Description

This is an implementation of RePair, which is a famous algorithm of grammar compression method.  
RePair is proposed in

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
	cd target/release
    ./rp [FLAGS] [OPTIONS] --input <input>

FLAGS:
    -d, --dcp        Decompress
    -h, --help       Prints help information
    -p, --print      Print the detail of constructed grammar
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input sourse text file
    -m, --min <minfreq>    Set minimum frequency of pairing operation (default: 3)
```


