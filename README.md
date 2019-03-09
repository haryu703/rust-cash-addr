# cash_addr
[![CircleCI](https://circleci.com/gh/haryu703/rust-cash-addr/tree/master.svg?style=svg)](https://circleci.com/gh/haryu703/rust-cash-addr/tree/master)
[![codecov](https://codecov.io/gh/haryu703/rust-cash-addr/branch/master/graph/badge.svg)](https://codecov.io/gh/haryu703/rust-cash-addr)

cash_addr format implementation inspired by [cashaddrjs](https://github.com/bitcoincashjs/cashaddrjs).  

[cash_addr specification](https://github.com/bitcoincashorg/bitcoincash.org/blob/master/spec/cashaddr.md)

## Usage
```rust
use cash_addr::{encode, decode, AddressType};

let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];
let prefix = "bitcoincash";
let addr_type = AddressType::P2PKH;

let address = encode(prefix, addr_type, &data).unwrap();
assert_eq!(address, "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2");

let (prefix, addr_type, hash) = decode(&address).unwrap();
assert_eq!(prefix, "bitcoincash");
assert_eq!(addr_type, AddressType::P2PKH);
assert_eq!(hash, data);
```
