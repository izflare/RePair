# RePair

### Description

This is an implementation of RePair, which is a famous algorithm of grammar compression method.  
RePair is proposed in

> N. Jesper Larsson and Alistair Moffat: _Off-line dictionary-based compression._ Proceedings of the IEEE, 88(11):1722-1732, 2000.

The constructed grammar is encoded by Huffman coding.

### Download

```
git clone https://github.com/izflare/RePair.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.33.0.  
After download the repository, 

```
cd RePair
cargo build --release
```

### Run

After compiling,

```
cd target/release
./rp --input <input> [--print]
```

`<input>` is your input text data file.  
Size of constructed grammar and elapsed time for running will be displayed.  
If you execute with `--print` option, constructed grammar will also be displayed.

