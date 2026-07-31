//! 🔬️ Graph motif extraction via iterative color refinement (1-dimensional Weisfeiler-Leman):
//! assigns each node a signature summarizing its rooted neighborhood out to `rounds` hops — own
//! label plus the iteratively-refined multiset of neighbor signatures — so two nodes with
//! isomorphic local neighborhood structure always end up with identical signatures (the classical
//! soundness guarantee of color refinement: it never conflates non-isomorphic neighborhoods
//! within `rounds` hops into the same color unless a genuine coarser 1-WL merge is unavoidable —
//! most real graphs distinguish all non-isomorphic bounded neighborhoods within a few rounds).
//!
//! **Scope, stated explicitly**: this provides the canonicalization primitive, not the full
//! "motifs as higher-order patterns" pipeline the original design sketched — turning a canonical
//! signature into a `PatternId` (and handling the automorphism-group / port-permutation
//! bookkeeping a *directed*, multi-relation graph's motifs would need, since this module's
//! neighbor-color multiset intentionally ignores which relation each arc used) is deferred until
//! a concrete consumer defines exactly what a "motif pattern" should look like for their model.

use crate::ids::NodeId;
use crate::topology::Topology;
use std::collections::HashMap;

// #region 🔖️Refine
/// 🔬️ Runs `rounds` steps of color refinement starting from `initial_labels` (one per node,
/// typically each node's own base pattern/tag as a `u64`). Each round replaces every node's color
/// with a hash of its own current color and the sorted multiset of its out-neighbors' current
/// colors — sorting is what makes the result depend only on the neighborhood's *structure*, not
/// on arc enumeration order.
#[allow(dead_code)] // no consumer yet turns signatures into model patterns (see this module's scope note); exercised today only by this module's own tests
pub(crate) fn refine_colors<T: Topology>(topo: &T, initial_labels: &[u64], rounds: usize) -> Vec<u64> {
    let n = topo.node_count();
    debug_assert_eq!(initial_labels.len(), n);
    let mut colors = initial_labels.to_vec();
    for _ in 0..rounds {
        let mut next = Vec::with_capacity(n);
        for i in 0..n {
            let node = NodeId::from_index(i);
            let mut neighbor_colors = Vec::new();
            topo.for_each_out_arc(node, |m, _r| neighbor_colors.push(colors[m.index()]));
            neighbor_colors.sort_unstable();
            next.push(signature_hash(colors[i], &neighbor_colors));
        }
        colors = next;
    }
    colors
}

/// 🔬️ FNV-1a-style mixing (matching `CompiledModel::fingerprint`'s own convention), folding in a
/// node's own color, its neighbor count, and every neighbor color in sorted order.
fn signature_hash(own: u64, neighbor_colors: &[u64]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |x: u64| {
        for b in x.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    mix(own);
    mix(neighbor_colors.len() as u64);
    for &c in neighbor_colors {
        mix(c);
    }
    h
}

/// 🔬️ Relabels arbitrary `u64` colors into dense small integers `0..k`, in first-seen order (so
/// the mapping is deterministic given a deterministic input order) — convenient for turning raw
/// signature hashes into compact motif ids. Returns the relabeled colors and `k`, the number of
/// distinct colors found.
#[allow(dead_code)] // exercised today only by this module's own tests; see refine_colors' note
pub(crate) fn canonicalize(colors: &[u64]) -> (Vec<u32>, usize) {
    let mut map: HashMap<u64, u32> = HashMap::new();
    let mut out = Vec::with_capacity(colors.len());
    for &c in colors {
        let next_id = map.len() as u32;
        let id = *map.entry(c).or_insert(next_id);
        out.push(id);
    }
    (out, map.len())
}
// #endregion 🔖️Refine

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::GraphTopologyBuilder;
    use crate::ids::RelationId;

    #[test]
    fn isomorphic_rooted_neighborhoods_get_identical_signatures() {
        // Two disjoint triangles: every node in each triangle has the exact same rooted
        // 1-hop neighborhood shape (two same-labeled neighbors), so 1 round of refinement must
        // make all six nodes' signatures identical.
        let mut b = GraphTopologyBuilder::new(6);
        let r = RelationId(0);
        for &(a, c) in &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)] {
            b.arc(NodeId(a), NodeId(c), r);
            b.arc(NodeId(c), NodeId(a), r);
        }
        let topo = b.build().unwrap();
        let labels = vec![7u64; 6]; // uniform base label
        let colors = refine_colors(&topo, &labels, 1);
        assert!(colors.iter().all(|&c| c == colors[0]));
    }

    #[test]
    fn structurally_different_neighborhoods_get_different_signatures() {
        // A 4-node "star" (node0 connects to 1,2,3; they don't connect to each other): the hub
        // has 3 neighbors, the leaves have 1 each — must not collide after refinement.
        let mut b = GraphTopologyBuilder::new(4);
        let r = RelationId(0);
        for leaf in [1, 2, 3] {
            b.arc(NodeId(0), NodeId(leaf), r);
            b.arc(NodeId(leaf), NodeId(0), r);
        }
        let topo = b.build().unwrap();
        let labels = vec![1u64; 4];
        let colors = refine_colors(&topo, &labels, 1);
        assert_ne!(colors[0], colors[1], "hub (degree 3) must differ from a leaf (degree 1)");
        assert_eq!(colors[1], colors[2]);
        assert_eq!(colors[2], colors[3], "the three leaves are structurally identical");
    }

    #[test]
    fn different_base_labels_propagate_into_different_signatures() {
        let mut b = GraphTopologyBuilder::new(2);
        let r = RelationId(0);
        b.arc(NodeId(0), NodeId(1), r);
        b.arc(NodeId(1), NodeId(0), r);
        let topo = b.build().unwrap();
        let same_labels = refine_colors(&topo, &[1, 1], 1);
        let different_labels = refine_colors(&topo, &[1, 2], 1);
        assert_eq!(same_labels[0], same_labels[1]);
        assert_ne!(different_labels[0], different_labels[1]);
    }

    #[test]
    fn zero_rounds_is_the_identity_on_labels_modulo_hashing() {
        // With 0 rounds, colors are exactly `initial_labels` unchanged (no hashing applied at all).
        let b = GraphTopologyBuilder::new(3);
        let topo = b.build().unwrap();
        let labels = vec![10u64, 20, 10];
        assert_eq!(refine_colors(&topo, &labels, 0), labels);
    }

    #[test]
    fn refine_colors_is_deterministic() {
        let mut b = GraphTopologyBuilder::new(5);
        let r = RelationId(0);
        for &(a, c) in &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)] {
            b.arc(NodeId(a), NodeId(c), r);
            b.arc(NodeId(c), NodeId(a), r);
        }
        let topo = b.build().unwrap();
        let labels = vec![1u64, 2, 1, 2, 3];
        assert_eq!(refine_colors(&topo, &labels, 3), refine_colors(&topo, &labels, 3));
    }

    #[test]
    fn canonicalize_produces_dense_first_seen_ids() {
        let (ids, k) = canonicalize(&[100, 200, 100, 300, 200]);
        assert_eq!(ids, vec![0, 1, 0, 2, 1]);
        assert_eq!(k, 3);
    }

    #[test]
    fn canonicalize_of_empty_input_is_empty() {
        let (ids, k) = canonicalize(&[]);
        assert!(ids.is_empty());
        assert_eq!(k, 0);
    }
}
// #endregion 🔖️Tests
