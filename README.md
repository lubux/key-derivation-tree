# Key Derivation Tree 
[![Build Status](https://travis-ci.org/lubux/key-derivation-tree.svg?branch=master)](https://travis-ci.org/lubux/key-derivation-tree)


Rust implementation of the key derivation tree used in TimeCrypt([Link](https://www.usenix.org/conference/nsdi20/presentation/burkhalter)).


## Build

```
cargo build
```

## Usage

Initialize the Key Derivation Tree with the master secret `key` and 32 bit inputs.
```rust
let key = [0u8; 16];
let prf = ConstrainedPrf::init(32, key);
```
Derive the i-th key.
```rust
let key_out = prf.apply(i).unwrap();
```

Give access to the keys in the range `[1,15)`.
```rust
let node_keys = prf.constrain(1, 15).unwrap();
```

Initialize the Key Derivation Tree with the constrained nodes. 
```rust
let prf2 = ConstrainedPrf::new(32, node_keys);
// derive key ok
let key_out = prf.apply(2).unwrap();
// derive key error
let key_out = prf.apply(0).unwrap();
```


## Benchmark

```
cargo bench
```

## Disclaimer 
This is a research prototype.
