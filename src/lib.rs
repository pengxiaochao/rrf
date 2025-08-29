use std::collections::HashMap;

/// Reciprocal Rank Fusion (RRF) 融合
///
/// # Arguments
/// * `ranked_lists` - 多个候选排序结果，每个内部 Vec 是按相关度降序的文档ID
/// * `k` - 平滑参数，常用 60
///
/// # Returns
/// 返回一个 Vec<(文档ID, 分数)>，按分数降序排列
pub fn fuse<T: Clone + Eq + std::hash::Hash + Ord>(
    ranked_lists: &[Vec<T>],
    k: usize,
) -> Vec<(T, f64)> {
    let mut score: HashMap<T, f64> = HashMap::new();

    for list in ranked_lists {
        for (rank, doc) in list.iter().enumerate() {
            let contribution = 1.0 / ((k + rank + 1) as f64);
            *score.entry(doc.clone()).or_insert(0.0) += contribution;
        }
    }

    let mut result: Vec<(T, f64)> = score.into_iter().collect();
    result.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap()
            .then_with(|| a.0.cmp(&b.0))
    });
    result
}

/// 带权重的 RRF 融合
pub fn fuse_weighted<T: Clone + Eq + std::hash::Hash + Ord>(
    ranked_lists: &[Vec<T>],
    weights: &[f64],
    k: usize,
) -> Vec<(T, f64)> {
    assert_eq!(
        ranked_lists.len(),
        weights.len(),
        "ranked_lists 和 weights 长度必须一致"
    );

    let mut score: HashMap<T, f64> = HashMap::new();

    for (list, &w) in ranked_lists.iter().zip(weights.iter()) {
        for (rank, doc) in list.iter().enumerate() {
            let contribution = w * (1.0 / ((k + rank + 1) as f64));
            *score.entry(doc.clone()).or_insert(0.0) += contribution;
        }
    }

    let mut result: Vec<(T, f64)> = score.into_iter().collect();
    result.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap()
            .then_with(|| a.0.cmp(&b.0))
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf_basic() {
        let bm25   = vec!["D3", "D1", "D2", "D5"];
        let vector = vec!["D2", "D4", "D1"];
        let rules  = vec!["D5", "D2", "D6"];

        let fused = fuse(&[bm25.clone(), vector.clone(), rules.clone()], 60);
        assert!(fused.iter().any(|(d, _)| d == &"D2"));
    }

    #[test]
    fn test_rrf_weighted() {
        let bm25   = vec!["D3", "D1", "D2", "D5"];
        let vector = vec!["D2", "D4", "D1"];
        let rules  = vec!["D5", "D2", "D6"];

        let fused_w = fuse_weighted(
            &[bm25, vector, rules],
            &[1.0, 2.0, 0.5],
            60,
        );
        assert!(fused_w[0].0 == "D2"); // D2 应该最靠前
    }
}