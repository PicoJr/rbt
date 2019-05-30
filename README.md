# Rust Binary Tool

A yet somehow _failed_ attempt to replace bits periodically in a binary file efficiently.

## Benchmarks

build

```
cargo build --release
```

generate `10MB` nullbytes10M

```
head -c 10M < /dev/zero > nullbytes10M
```

benchmark using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./target/release/rbt nullbytes10M out 65,255,8,0` | 320.2 ± 9.1 | 309.3…341.0 |

what it does: (replace every 8 bytes by 0x41 ie 'A')

runs at: `30 MB/s`

```
Intel Celeron N2840 (2) @ 2.582GHz
disk: SSD
```
