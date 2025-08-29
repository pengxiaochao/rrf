# RRF - Reciprocal Rank Fusion in Rust

[![Crate](https://img.shields.io/crates/v/rrf.svg)](https://crates.io/crates/rrf)
[![Documentation](https://docs.rs/rrf/badge.svg)](https://docs.rs/rrf)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A lightweight, efficient implementation of the Reciprocal Rank Fusion (RRF) algorithm in Rust.

## What is RRF?

Reciprocal Rank Fusion (RRF) is a simple but effective method for combining multiple ranked lists. It's commonly used in information retrieval, search engines, and recommendation systems to fuse results from different ranking algorithms.

RRF works by assigning a score to each document based on its position in each ranked list, then combining these scores to create a final ranking. The formula is:

```
RRF_score(document) = ∑ 1/(k + rank_i)
```

where `k` is a constant (commonly 60) that mitigates the impact of high rankings in individual lists.

## Features

- Standard RRF algorithm implementation
- Weighted RRF variant for assigning different importance to different ranking sources
- Generic implementation that works with any hashable, comparable type
- Zero external dependencies
- Thoroughly tested

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rrf = "0.1.0"
```

## Usage

### Basic Usage

```rust
use rrf::fuse;

fn main() {
    // Three different ranking algorithms produced these results
    let bm25   = vec!["D3", "D1", "D2", "D5"];
    let vector = vec!["D2", "D4", "D1"];
    let rules  = vec!["D5", "D2", "D6"];

    // Fuse them with RRF (k=60)
    let fused_results = fuse(&[bm25, vector, rules], 60);
    
    // Print results (document ID and score)
    for (doc, score) in fused_results {
        println!("{} -> {:.6}", doc, score);
    }
}
```

### Weighted RRF

If some ranking algorithms are more trustworthy than others, you can assign weights:

```rust
use rrf::fuse_weighted;

fn main() {
    let bm25   = vec!["D3", "D1", "D2", "D5"];
    let vector = vec!["D2", "D4", "D1"];
    let rules  = vec!["D5", "D2", "D6"];

    // Assign weights to each ranking algorithm
    let fused_results = fuse_weighted(
        &[bm25, vector, rules],
        &[1.0, 2.0, 0.5],  // Vector search gets double weight, rules half weight
        60
    );
    
    for (doc, score) in fused_results {
        println!("{} -> {:.6}", doc, score);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or pull requests.
