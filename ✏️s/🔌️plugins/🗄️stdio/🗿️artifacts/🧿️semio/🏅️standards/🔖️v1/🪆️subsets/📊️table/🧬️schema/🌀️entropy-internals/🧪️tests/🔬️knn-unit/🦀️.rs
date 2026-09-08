mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn random_points(n: usize, dim: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(seed);
        (0..n * dim).map(|_| rng.next_f64() * 10.0 - 5.0).collect()
    }

    #[test]
    fn build_rejects_empty_and_bad_shape() {
        assert!(KdTree::build(&[], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0, 3.0], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0], 0).is_err());
    }

    #[test]
    fn kd_tree_matches_brute_force_knn_euclidean() {
        let points = random_points(200, 3, 11);
        let tree = KdTree::build(&points, 3).unwrap();
        for q in 0..20 {
            let query = &points[q * 3..(q + 1) * 3];
            let tree_result = tree.k_nearest(query, 5, Metric::Euclidean, Some(q));
            let brute_result = brute_force_knn(&points, 3, query, 5, Metric::Euclidean, Some(q)).unwrap();
            let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
            let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
            for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                assert!((a - b).abs() < 1e-9, "query {q}: tree={tree_dists:?} brute={brute_dists:?}");
            }
        }
    }

    #[test]
    fn kd_tree_matches_brute_force_for_chebyshev_and_manhattan() {
        let points = random_points(150, 2, 22);
        let tree = KdTree::build(&points, 2).unwrap();
        for metric in [Metric::Chebyshev, Metric::Manhattan] {
            for q in 0..10 {
                let query = &points[q * 2..(q + 1) * 2];
                let tree_result = tree.k_nearest(query, 4, metric, Some(q));
                let brute_result = brute_force_knn(&points, 2, query, 4, metric, Some(q)).unwrap();
                let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
                let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
                assert_eq!(tree_dists.len(), brute_dists.len());
                for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                    assert!((a - b).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn radius_count_matches_brute_force() {
        let points = random_points(100, 2, 33);
        let tree = KdTree::build(&points, 2).unwrap();
        for q in 0..10 {
            let query = &points[q * 2..(q + 1) * 2];
            let radius = 1.5;
            let tree_count = tree.count_within_radius(query, radius, Metric::Chebyshev, Some(q));
            let brute_count = (0..100).filter(|&i| i != q).filter(|&i| distance(&points[i * 2..(i + 1) * 2], query, Metric::Chebyshev) < radius).count();
            assert_eq!(tree_count, brute_count, "query {q}");
        }
    }

    #[test]
    fn k_nearest_excludes_self_when_requested() {
        let points = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 2, Metric::Euclidean, Some(0));
        assert!(!result.iter().any(|&(i, _)| i == 0));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn k_nearest_on_single_point_tree() {
        let points = vec![5.0, 5.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 1, Metric::Euclidean, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 0);
    }

    mod quick {
        use super::*;

        #[test]
        fn kd_tree_matches_brute_force_on_larger_random_set() {
            let points = random_points(1000, 4, 999);
            let tree = KdTree::build(&points, 4).unwrap();
            for q in (0..1000).step_by(97) {
                let query = &points[q * 4..(q + 1) * 4];
                let tree_result = tree.k_nearest(query, 8, Metric::Euclidean, Some(q));
                let brute_result = brute_force_knn(&points, 4, query, 8, Metric::Euclidean, Some(q)).unwrap();
                for (a, b) in tree_result.iter().zip(brute_result.iter()) {
                    assert!((a.1 - b.1).abs() < 1e-9);
                }
            }
        }
    }
}
