//! 🎲️ Weighted draw: the Fenwick tree over vortex weights and the weighted pick and draw that consume the per-run random stream.

use super::*;

pub(super) fn fenwick_add(tree: &mut [f64], index: usize, delta: f64) {
    let mut cursor = index + 1;
    while cursor < tree.len() {
        tree[cursor] += delta;
        cursor += cursor & cursor.wrapping_neg();
    }
}

fn fenwick_total(tree: &[f64]) -> f64 {
    let mut cursor = tree.len().saturating_sub(1);
    let mut total = 0.0;
    while cursor > 0 {
        total += tree[cursor];
        cursor &= cursor - 1;
    }
    total
}

fn fenwick_pick(tree: &[f64], target: f64) -> usize {
    let mut index = 0;
    let mut prefix = 0.0;
    let mut bit = 1usize;
    while bit < tree.len() {
        bit <<= 1;
    }
    let mut step = bit >> 1;
    while step > 0 {
        let next = index + step;
        if next < tree.len() && prefix + tree[next] < target {
            prefix += tree[next];
            index = next;
        }
        step >>= 1;
    }
    index.min(tree.len().saturating_sub(2))
}

fn fenwick_prefix(tree: &[f64], count: usize) -> f64 {
    let mut cursor = count.min(tree.len().saturating_sub(1));
    let mut total = 0.0;
    while cursor > 0 {
        total += tree[cursor];
        cursor &= cursor - 1;
    }
    total
}

/// ➕️ Appends one weight to a Fenwick tree in O(log n): the new node covers `(i - lowbit(i), i]`.
pub(super) fn fenwick_push(tree: &mut Vec<f64>, weight: f64) {
    let index = tree.len();
    let covered = weight + fenwick_prefix(tree, index - 1) - fenwick_prefix(tree, index - (index & index.wrapping_neg()));
    tree.push(covered);
}

/// 🎲️ One index drawn with probability proportional to its weight, the weights left as they are.
pub(super) fn weighted_draw(weights: &[f64], tree: &[f64], rng_state: &mut u32) -> Option<usize> {
    let total = fenwick_total(tree);
    if total <= 0.0 {
        return None;
    }
    let index = fenwick_pick(tree, (fill_rng(rng_state) * total).max(f64::MIN_POSITIVE));
    if weights.get(index).is_some_and(|weight| *weight > 0.0) {
        return Some(index);
    }
    weights.iter().position(|weight| *weight > 0.0)
}

pub(super) fn weighted_pick(weights: &mut [f64], tree: &mut [f64], remaining: usize, rng_state: &mut u32) -> Option<usize> {
    if remaining == 0 {
        return None;
    }
    let total = fenwick_total(tree);
    if total <= 0.0 {
        return None;
    }
    let target = if weights.len() == 1 { f64::MIN_POSITIVE } else { (fill_rng(rng_state) * total).max(f64::MIN_POSITIVE) };
    let index = fenwick_pick(tree, target);
    let weight = std::mem::replace(&mut weights[index], 0.0);
    fenwick_add(tree, index, -weight);
    Some(index)
}
