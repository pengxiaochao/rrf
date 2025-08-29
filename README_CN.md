# RRF - Rust 实现的倒数排名融合算法

[![Crate](https://img.shields.io/crates/v/rrf.svg)](https://crates.io/crates/rrf)
[![Documentation](https://docs.rs/rrf/badge.svg)](https://docs.rs/rrf)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

RRF 是一个轻量、高效的 Rust 实现的倒数排名融合 (Reciprocal Rank Fusion) 算法。

## 什么是 RRF？

倒数排名融合 (RRF) 是一种简单而有效的方法，用于合并多个排序列表。它广泛应用于信息检索、搜索引擎和推荐系统，用来融合来自不同排序算法的结果。

RRF 通过基于文档在每个排序列表中的位置分配分数，然后组合这些分数来创建最终排名。公式为：

```
RRF_score(document) = ∑ 1/(k + rank_i)
```

其中 `k` 是一个常数（通常为 60），用于减轻单个列表中高排名的影响。

## 特性

- 标准 RRF 算法实现
- 带权重的 RRF 变体，可为不同排序源分配不同重要性
- 通用实现，适用于任何可哈希、可比较的类型
- 零外部依赖
- 经过全面测试

## 安装

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
rrf = "0.1.0"
```

## 使用方法

### 基本用法

```rust
use rrf::fuse;

fn main() {
    // 三种不同的排序算法产生这些结果
    let bm25   = vec!["D3", "D1", "D2", "D5"];
    let vector = vec!["D2", "D4", "D1"];
    let rules  = vec!["D5", "D2", "D6"];

    // 使用 RRF 融合它们 (k=60)
    let fused_results = fuse(&[bm25, vector, rules], 60);
    
    // 输出结果（文档ID和分数）
    for (doc, score) in fused_results {
        println!("{} -> {:.6}", doc, score);
    }
}
```

### 带权重的 RRF

如果某些排序算法比其他算法更可信，您可以分配权重：

```rust
use rrf::fuse_weighted;

fn main() {
    let bm25   = vec!["D3", "D1", "D2", "D5"];
    let vector = vec!["D2", "D4", "D1"];
    let rules  = vec!["D5", "D2", "D6"];

    // 为每个排序算法分配权重
    let fused_results = fuse_weighted(
        &[bm25, vector, rules],
        &[1.0, 2.0, 0.5],  // 向量搜索获得双倍权重，规则获得半权重
        60
    );
    
    for (doc, score) in fused_results {
        println!("{} -> {:.6}", doc, score);
    }
}
```

## 许可证

该项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献！请随时提出问题或拉取请求。
