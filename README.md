# Hash Finder 

## Overview 
A Rust console application that calculates SHA-256 hashes for consecutive integers, starting from 1. The program prints the hash and the number if the hash ends with N zeros. 

## Building

### Prerequisites
- [Rust](https://rustup.rs/) 1.70.0 or higher

### Build Steps 
```bash
# Clone the repository
git clone https://github.com/Netflate/hash_finder.git
cd hash-finder

# Build release version
cargo build --release

# The binary will be located at:
# ./target/release/hash_finder
```

## Usage
```bash
./target/release/hash_finder -N <zeros> -F <count>
```

### Parameters
- `-N` — number of trailing zeros
- `-F` — number of results to find

### Example
Find 10 numbers whose SHA-256 hashes end with 4 zeros:
```bash
$ ./target/release/hash_finder -N 4 -F 10
31214, "16b024b09ebcb9d66f6a9968858d7e01081e51a14a4922edf3c8e3c2009c0000"
88183, "1b8af662814e2b7c794ae26d5624f23886b6915fb3e9e2c0d4d62099731e0000"
112370, "4ff6b83cd5d3afa354d1ae9cf8923e9cf6caa199cc3b74ee3c53b47da1c20000"
140322, "ace3f7f0672635a4b3be1a036c2506d899ade343d6b799a22598c0a088d70000"
269105, "78e2aa509d7715e9bbf2247f9d56004a64fb3e7822d3acad456afed8d1a60000"
293382, "6781efaad70d988dfba19c165b4e09644cf6ed1ed5386a985aad2b5f12ff0000"
329700, "cfd8a622a5a604f24986f0523aec5062d38e1ebff5906c96b30c37e886190000"
336544, "74460cfb2ade1516b3f532da15f78ba0c498ee4db4c93812f840b1010bf50000"
386460, "59ac3d721d18d2224a1aefe28e5672f837de6c54b1409b8b55fae1b59efe0000"
464582, "6c20bdf75933d112376e2ffe2e6f5c727fd79f4a212c64c5f0491860b9620000"
```

### Running without building
```bash
cargo run --release -- -N 3 -F 6
```

## Implementation Details

### Concurrency 
* Uses chunk-based parallelism with configurable chunk size (can be modified in code via `chunk_size` variable)
  * Default value is 100 to maintain reasonable output ordering while preserving good performance; chunk size can be increased for better performance, but this will make console output unsorted
* Each thread claims chunks of numbers for processing to improve performance

### Thread Coordination
- `Arc<AtomicU64>` for work distribution
- `Arc<AtomicUsize>` for tracking remaining results
- `Arc<AtomicBool>` for early termination signal
