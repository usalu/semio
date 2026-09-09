use super::*;
use std::collections::HashMap;

fn lcg_next(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}

fn rand_unit(state: &mut u64) -> f64 {
    (lcg_next(state) >> 11) as f64 / (1u64 << 53) as f64
}

fn rand_range(state: &mut u64, lo: f64, hi: f64) -> f64 {
    lo + (hi - lo) * rand_unit(state)
}

fn make_cloud<const D: usize>(n: usize, seed: u64) -> Vec<[f64; D]> {
    let mut state = seed;
    let mut pts: Vec<[f64; D]> = (0..n)
        .map(|_| {
            let mut p = [0.0; D];
            for v in p.iter_mut() {
                *v = rand_range(&mut state, -50.0, 50.0);
            }
            p
        })
        .collect();
    for i in 0..n / 10 {
        let src = pts[(i * 13 + 7) % n];
        pts[i] = src;
    }
    pts
}

fn brute_all<const D: usize>(pts: &[[f64; D]], q: &[f64; D]) -> Vec<(u32, f64)> {
    let mut all: Vec<(u32, f64)> = pts.iter().enumerate().map(|(i, p)| (i as u32, dist_sq(p, q))).collect();
    all.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    all
}

fn make_query<const D: usize>(state: &mut u64, qi: usize) -> [f64; D] {
    let mut q = [0.0; D];
    for v in q.iter_mut() {
        *v = rand_range(state, -80.0, 80.0);
    }
    if qi.is_multiple_of(5) {
        for v in q.iter_mut() {
            *v += 300.0;
        }
    }
    q
}

fn check_kd_nearest_parity<const D: usize>(seed: u64) {
    let pts = make_cloud::<D>(2000, seed);
    let tree = KdTree::build(&pts);
    let mut state = seed ^ 0xABCD;
    for qi in 0..50 {
        let q = make_query::<D>(&mut state, qi);
        let expected = brute_all(&pts, &q);
        assert_eq!(tree.nearest(&q), Some(expected[0]));
        for &k in &[1usize, 7, 64, 2500] {
            let got = tree.k_nearest(&q, k);
            let want: Vec<(u32, f64)> = expected.iter().copied().take(k).collect();
            assert_eq!(got, want);
        }
    }
}

#[test]
fn kd_nearest_and_k_nearest_match_brute_force_d2() {
    check_kd_nearest_parity::<2>(11);
}

#[test]
fn kd_nearest_and_k_nearest_match_brute_force_d3() {
    check_kd_nearest_parity::<3>(23);
}

fn check_kd_radius_parity<const D: usize>(seed: u64) {
    let pts = make_cloud::<D>(2000, seed);
    let tree = KdTree::build(&pts);
    let mut state = seed ^ 0x5150;
    for qi in 0..30 {
        let q = make_query::<D>(&mut state, qi);
        for &r in &[0.0f64, 5.0, 20.0, 60.0] {
            let got = tree.radius(&q, r);
            let mut want: Vec<(u32, f64)> = brute_all(&pts, &q).into_iter().filter(|e| e.1 <= r * r).collect();
            want.sort_unstable_by_key(|e| e.0);
            assert_eq!(got, want);
        }
        assert!(tree.radius(&q, -1.0).is_empty());
    }
}

#[test]
fn kd_radius_matches_brute_force_d2() {
    check_kd_radius_parity::<2>(31);
}

#[test]
fn kd_radius_matches_brute_force_d3() {
    check_kd_radius_parity::<3>(41);
}

fn check_kd_aabb_parity<const D: usize>(seed: u64) {
    let pts = make_cloud::<D>(2000, seed);
    let tree = KdTree::build(&pts);
    let mut state = seed ^ 0xBEEF;
    for _ in 0..30 {
        let mut lo = [0.0; D];
        let mut hi = [0.0; D];
        for a in 0..D {
            let x = rand_range(&mut state, -60.0, 60.0);
            let y = rand_range(&mut state, -60.0, 60.0);
            lo[a] = x.min(y);
            hi[a] = x.max(y);
        }
        let mut got = Vec::new();
        tree.for_each_in_aabb(&lo, &hi, |id| got.push(id));
        got.sort_unstable();
        let want: Vec<u32> = pts.iter().enumerate().filter(|(_, p)| (0..D).all(|a| (lo[a]..=hi[a]).contains(&p[a]))).map(|(i, _)| i as u32).collect();
        assert_eq!(got, want);
    }
}

#[test]
fn kd_aabb_visits_match_brute_force_d2() {
    check_kd_aabb_parity::<2>(51);
}

#[test]
fn kd_aabb_visits_match_brute_force_d3() {
    check_kd_aabb_parity::<3>(61);
}

#[test]
fn kd_empty_tree_is_safe() {
    let tree = KdTree::<3>::build(&[]);
    let q = [0.0; 3];
    assert_eq!(tree.nearest(&q), None);
    assert!(tree.k_nearest(&q, 5).is_empty());
    assert!(tree.radius(&q, 10.0).is_empty());
    let mut visited = 0;
    tree.for_each_in_aabb(&[-1.0; 3], &[1.0; 3], |_| visited += 1);
    assert_eq!(visited, 0);
    let full = KdTree::<2>::build(&[[1.0, 2.0]]);
    assert!(full.k_nearest(&[0.0, 0.0], 0).is_empty());
}

#[test]
fn voxel_grid3_cell_of_floors_negative_coords() {
    let grid = VoxelGrid3::new(2.5);
    assert_eq!(grid.cell_of([-0.1, 0.0, 2.5]), (-1, 0, 1));
    assert_eq!(grid.cell_of([-2.5, -2.6, 4.9]), (-1, -2, 1));
}

#[test]
fn voxel_grid3_neighbors27_matches_brute_force() {
    let cell = 2.5;
    let mut grid = VoxelGrid3::new(cell);
    let mut state = 42u64;
    let mut pts: Vec<[f64; 3]> = (0..300).map(|_| [rand_range(&mut state, -12.0, 12.0), rand_range(&mut state, -12.0, 12.0), rand_range(&mut state, -12.0, 12.0)]).collect();
    for k in -2i32..=2 {
        for l in -2i32..=2 {
            for m in -2i32..=2 {
                pts.push([f64::from(k) * cell, f64::from(l) * cell, f64::from(m) * cell]);
            }
        }
    }
    for (i, p) in pts.iter().enumerate() {
        grid.insert(*p, i as u32);
    }
    let mut queries: Vec<[f64; 3]> = (0..40).map(|_| [rand_range(&mut state, -13.0, 13.0), rand_range(&mut state, -13.0, 13.0), rand_range(&mut state, -13.0, 13.0)]).collect();
    queries.extend_from_slice(&pts[300..330]);
    for q in &queries {
        let qc = grid.cell_of(*q);
        let want: Vec<u32> = pts
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let c = grid.cell_of(**p);
                (c.0 - qc.0).abs() <= 1 && (c.1 - qc.1).abs() <= 1 && (c.2 - qc.2).abs() <= 1
            })
            .map(|(i, _)| i as u32)
            .collect();
        assert_eq!(grid.neighbors27(*q), want);
    }
}

#[test]
fn grid2_neighbors9_matches_brute_force() {
    let cell = 4.0;
    let mut grid = Grid2::new(cell);
    let mut state = 77u64;
    let mut pts: Vec<[f64; 2]> = (0..200).map(|_| [rand_range(&mut state, -20.0, 20.0), rand_range(&mut state, -20.0, 20.0)]).collect();
    for k in -3i32..=3 {
        for l in -3i32..=3 {
            pts.push([f64::from(k) * cell, f64::from(l) * cell]);
        }
    }
    for (i, p) in pts.iter().enumerate() {
        grid.insert(*p, i as u32);
    }
    let queries: Vec<[f64; 2]> = (0..40).map(|_| [rand_range(&mut state, -22.0, 22.0), rand_range(&mut state, -22.0, 22.0)]).collect();
    for q in queries.iter().chain(pts[200..220].iter()) {
        let qc = grid.cell_of(*q);
        let want: Vec<u32> = pts
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let c = grid.cell_of(**p);
                (c.0 - qc.0).abs() <= 1 && (c.1 - qc.1).abs() <= 1
            })
            .map(|(i, _)| i as u32)
            .collect();
        assert_eq!(grid.neighbors9(*q), want);
    }
}

#[test]
fn morton3_round_trips() {
    assert_eq!(morton3_encode(0, 0, 0), 0);
    assert_eq!(morton3_encode(1, 0, 0), 1);
    assert_eq!(morton3_encode(0, 1, 0), 2);
    assert_eq!(morton3_encode(0, 0, 1), 4);
    let mut state = 5u64;
    for _ in 0..200 {
        let x = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
        let y = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
        let z = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
        assert_eq!(morton3_decode(morton3_encode(x, y, z)), (x, y, z));
    }
}

#[test]
fn octree_downsample_preserves_counts_and_centroids() {
    let pts = make_cloud::<3>(2000, 99);
    let tree = PointOctree::build(&pts, 8);
    let cell = 7.0;
    let ds = tree.downsample(cell);
    let total: usize = ds.iter().map(|e| e.0).sum();
    assert_eq!(total, pts.len());
    let mut origin = pts[0];
    for p in &pts {
        for a in 0..3 {
            origin[a] = origin[a].min(p[a]);
        }
    }
    let mut want: HashMap<u64, (usize, [f64; 3])> = HashMap::new();
    for p in &pts {
        let cx = (((p[0] - origin[0]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
        let cy = (((p[1] - origin[1]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
        let cz = (((p[2] - origin[2]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
        let slot = want.entry(morton3_encode(cx, cy, cz)).or_insert((0, [0.0; 3]));
        slot.0 += 1;
        for (acc, v) in slot.1.iter_mut().zip(p.iter()) {
            *acc += v;
        }
    }
    let mut want: Vec<(u64, usize, [f64; 3])> = want.into_iter().map(|(code, (count, sum))| (code, count, sum)).collect();
    want.sort_unstable_by_key(|e| e.0);
    assert_eq!(ds.len(), want.len());
    for (got, (_, count, sum)) in ds.iter().zip(want.iter()) {
        assert_eq!(got.0, *count);
        for (g, s) in got.1.iter().zip(sum.iter()) {
            assert!((g - s / *count as f64).abs() < 1e-9);
        }
    }
}

#[test]
fn octree_range_matches_brute_force() {
    let pts = make_cloud::<3>(2000, 7);
    for &depth in &[0u32, 4, 8, 30] {
        let tree = PointOctree::build(&pts, depth);
        let mut state = 1234u64 ^ u64::from(depth);
        for _ in 0..25 {
            let mut mn = [0.0; 3];
            let mut mx = [0.0; 3];
            for a in 0..3 {
                let x = rand_range(&mut state, -60.0, 60.0);
                let y = rand_range(&mut state, -60.0, 60.0);
                mn[a] = x.min(y);
                mx[a] = x.max(y);
            }
            let got = tree.range(mn, mx);
            let want: Vec<u32> = pts.iter().enumerate().filter(|(_, p)| (0..3).all(|a| (mn[a]..=mx[a]).contains(&p[a]))).map(|(i, _)| i as u32).collect();
            assert_eq!(got, want);
        }
        let all = tree.range([-1000.0; 3], [1000.0; 3]);
        assert_eq!(all, (0..pts.len() as u32).collect::<Vec<u32>>());
        assert!(tree.range([500.0; 3], [600.0; 3]).is_empty());
    }
}

#[test]
fn octree_empty_build_is_safe() {
    let tree = PointOctree::build(&[], 8);
    assert!(tree.range([-1.0; 3], [1.0; 3]).is_empty());
    assert!(tree.downsample(1.0).is_empty());
    let flat = PointOctree::build(&[[2.0, 2.0, 2.0], [2.0, 2.0, 2.0]], 8);
    assert_eq!(flat.range([1.0; 3], [3.0; 3]), vec![0, 1]);
    assert_eq!(flat.downsample(1.0), vec![(2, [2.0, 2.0, 2.0])]);
}
