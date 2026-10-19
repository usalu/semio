//! 🎯️ Selection and cumulative distribution of sampling: the partial top-k selection, the descending quickselect, the cumulative distribution binary search and the cumulative sums of probabilities. See [`super`].

use super::*;

// #region 🔖️Numerics
/// 📐️ Reorders `live[..]` so the top `k` (by `logits[live[i]]` descending, ties by ascending token
/// id for determinism) occupy `live[..k]`, unordered within that prefix — a Hoare-partition
/// quickselect, `O(n)` average instead of an `O(n log n)` full sort. `scratch` must have length
/// `>= live.len()` and its contents are overwritten.
pub async fn partial_select_top_k(logits: &[f32], live: &mut [u32], k: usize, scratch: &mut [f32]) {
    let n = live.len();
    if k >= n {
        return;
    }
    debug_assert!(scratch.len() >= n);
    let key = |token: u32| -> (f32, core::cmp::Reverse<u32>) { (logits[token as usize], core::cmp::Reverse(token)) };
    for (slot, &t) in live.iter().enumerate() {
        scratch[slot] = logits[t as usize];
    }
    quickselect_desc(live, scratch, 0, n - 1, k, &key).await;
}

async fn quickselect_desc(live: &mut [u32], scratch: &mut [f32], mut lo: usize, mut hi: usize, k: usize, key: &impl Fn(u32) -> (f32, core::cmp::Reverse<u32>)) {
    loop {
        if lo >= hi {
            return;
        }
        let pivot = key(live[(lo + hi) / 2]);
        let mut i = lo;
        let mut j = hi;
        loop {
            while key(live[i]) > pivot {
                i += 1;
            }
            while key(live[j]) < pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            live.swap(i, j);
            scratch.swap(i, j);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        if k <= j {
            hi = j;
        } else if k > j + 1 {
            lo = j + 1;
        } else {
            return;
        }
    }
}

/// 📐️ Smallest index `i` such that `cdf[i] >= u`, via binary search over a nondecreasing CDF.
/// Falls back to the last index when floating-point rounding leaves `u` fractionally above the
/// final (nominally `1.0`) entry.
pub async fn cdf_binary_search(cdf: &[f64], u: f64) -> usize {
    if cdf.is_empty() {
        return 0;
    }
    let mut lo = 0usize;
    let mut hi = cdf.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if cdf[mid] < u {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
// #endregion 🔖️Numerics

pub async fn cumulative_from_probs(probs: &[f32]) -> Vec<f64> {
    let mut out = Vec::with_capacity(probs.len());
    let mut sum = KahanSum::new().await;
    for &p in probs {
        sum.add(p as f64).await;
        out.push(sum.value().await);
    }
    if let Some(last) = out.last_mut() {
        *last = 1.0;
    }
    out
}
