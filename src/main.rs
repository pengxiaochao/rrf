use rrf::{fuse, fuse_weighted};

fn main() {
    let bm25   = vec!["D3", "D1", "D2", "D5"];
    let vector = vec!["D2", "D4", "D1"];
    let rules  = vec!["D5", "D2", "D6"];

    println!("=== 普通 RRF ===");
    for (doc, score) in fuse(&[bm25.clone(), vector.clone(), rules.clone()], 60) {
        println!("{} -> {:.6}", doc, score);
    }

    println!("\n=== 加权 RRF ===");
    for (doc, score) in fuse_weighted(&[bm25, vector, rules], &[1.0, 2.0, 0.5], 60) {
        println!("{} -> {:.6}", doc, score);
    }
}