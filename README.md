# RePair

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
cd target/release
./rp --input <input> [--print]
```

`<input>` is your input text data file.  
If you execute with `--print` option, constructed grammar will also be displayed.

