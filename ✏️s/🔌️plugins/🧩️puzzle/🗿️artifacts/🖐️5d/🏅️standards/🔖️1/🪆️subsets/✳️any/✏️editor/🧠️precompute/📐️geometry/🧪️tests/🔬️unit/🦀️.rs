use super::*;

/// 🎲️ Deterministic points (a 64-bit LCG) spread over a box a few cells wide, with duplicates.
fn points(count: usize, span: f64, seed: u64) -> Vec<[f64; 3]> {
    let mut state = seed;
    let mut next = || {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (state >> 11) as f64 / (1u64 << 53) as f64 * span - span / 2.0
    };
    let mut result: Vec<[f64; 3]> = (0..count).map(|_| [next(), next(), next()]).collect();
    result.extend(result.clone().into_iter().step_by(17));
    result
}

/// ⚖️ Radius and nearest queries agree with the `rstar` R*-tree oracle on every query point, including after
/// removals, and with a brute-force scan.
#[test]
fn point_grid_radius_and_nearest_queries_agree_with_the_rstar_oracle() {
    let cloud = points(2_000, 40.0, 7);
    let mut grid = Puzzle5dPointGrid::new(0.75);
    for (index, point) in cloud.iter().enumerate() {
        grid.insert(*point, index as u32);
    }
    let mut removed = std::collections::BTreeSet::new();
    for index in (0..cloud.len()).step_by(5) {
        assert!(grid.remove(cloud[index], index as u32));
        removed.insert(index);
    }
    assert!(!grid.remove(cloud[0], 0), "an entry is removed once");
    let live: Vec<(usize, [f64; 3])> = cloud.iter().copied().enumerate().filter(|(index, _)| !removed.contains(index)).collect();
    assert_eq!(grid.len(), live.len());
    let tree = rstar::RTree::bulk_load(live.iter().map(|(index, point)| rstar::primitives::GeomWithData::new(*point, *index as u32)).collect());
    for query in points(300, 44.0, 11) {
        for radius in [0.0, 0.3, 0.75, 1.9, 4.0] {
            let mut ours: Vec<u32> = grid.within(query, radius).map(|(_, value)| value).collect();
            let mut oracle: Vec<u32> = tree.locate_within_distance(query, radius * radius).map(|entry| entry.data).collect();
            let mut brute: Vec<u32> = live.iter().filter(|(_, point)| distance_squared(*point, query) <= radius * radius).map(|(index, _)| *index as u32).collect();
            ours.sort_unstable();
            oracle.sort_unstable();
            brute.sort_unstable();
            assert_eq!(ours, oracle, "radius {radius} around {query:?}");
            assert_eq!(ours, brute);
        }
        let oracle = tree.nearest_neighbor(&query).map(|entry| distance_squared(*entry.geom(), query)).expect("a non-empty tree");
        match grid.nearest(query, 8.0) {
            Some((point, _)) => assert!((distance_squared(point, query) - oracle).abs() < 1e-12, "nearest around {query:?}"),
            None => assert!(oracle > 64.0, "nothing nearer than the search radius around {query:?}"),
        }
    }
}
