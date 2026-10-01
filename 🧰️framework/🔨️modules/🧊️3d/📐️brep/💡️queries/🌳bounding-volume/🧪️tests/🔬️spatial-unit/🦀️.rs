mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_bvh_returns_no_matches() {
        let bvh: Bvh<u32> = Bvh::build(Vec::new());
        assert_eq!(bvh.query_point_nearest([0.0, 0.0, 0.0]), None);
        assert!(bvh.query_ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
        assert!(bvh.query_aabb_overlap(&aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn nearest_point_finds_closest_leaf() {
        let items = vec![(aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "near"), (aabb([10.0, 10.0, 10.0], [11.0, 11.0, 11.0]), "far")];
        let bvh = Bvh::build(items);
        assert_eq!(bvh.query_point_nearest([0.5, 0.5, 0.5]), Some(&"near"));
        assert_eq!(bvh.query_point_nearest([10.5, 10.5, 10.5]), Some(&"far"));
    }

    #[semio_framework_async_macros::async_test]
    async fn ray_hits_only_crossed_leaves() {
        let items = vec![(aabb([0.0, -1.0, -1.0], [1.0, 1.0, 1.0]), "hit"), (aabb([0.0, 10.0, 10.0], [1.0, 11.0, 11.0]), "miss")];
        let bvh = Bvh::build(items);
        let hits = bvh.query_ray([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(hits, vec![&"hit"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn aabb_overlap_finds_intersecting_leaves() {
        let items = vec![(aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "overlap"), (aabb([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]), "disjoint")];
        let bvh = Bvh::build(items);
        let mut hits = bvh.query_aabb_overlap(&aabb([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));
        hits.sort();
        assert_eq!(hits, vec![&"overlap"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn many_leaves_build_and_query_correctly() {
        let items: Vec<(Aabb, usize)> = (0..200).map(|i| (aabb([i as f64, 0.0, 0.0], [i as f64 + 0.5, 0.5, 0.5]), i)).collect();
        let bvh = Bvh::build(items);
        assert_eq!(bvh.query_point_nearest([100.2, 0.2, 0.2]), Some(&100));
    }

    #[semio_framework_async_macros::async_test]
    async fn query_ray_ordered_returns_near_to_far() {
        let items = vec![(aabb([5.0, -1.0, -1.0], [6.0, 1.0, 1.0]), "far"), (aabb([1.0, -1.0, -1.0], [2.0, 1.0, 1.0]), "near")];
        let bvh = Bvh::build(items);
        let hits = bvh.query_ray_ordered([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ordered: Vec<&&str> = hits.iter().map(|&(item, _)| item).collect();
        assert_eq!(ordered, vec![&"near", &"far"]);
        assert!(hits[0].1 < hits[1].1);
    }

    #[semio_framework_async_macros::async_test]
    async fn query_nearest_exact_prefers_true_distance_over_aabb_lower_bound() {
        let items = vec![(aabb([0.0, 0.0, 0.0], [3.0, 3.0, 3.0]), "big_far_corner"), (aabb([4.0, 4.0, 4.0], [4.2, 4.2, 4.2]), "small_near")];
        let bvh = Bvh::build(items);
        let target = [4.0, 4.0, 3.9];
        let got = bvh.query_nearest_exact(target, |item: &&str| if *item == "big_far_corner" { 10.0 } else { 0.3 });
        assert_eq!(got, Some(&"small_near"));
    }

    #[semio_framework_async_macros::async_test]
    async fn refit_updates_bounds_in_place_without_rebuilding() {
        // 🐛 FIX (ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` FX-5): item 0's OLD box
        // used to sit at `[0,1]³`, on the SAME `y=0.5, z=0.5` sightline the probe ray walks —
        // since `query_ray` casts a genuinely unbounded ray (`t ∈ [0, ∞)`, not a segment), that
        // ray already crosses `x ∈ [0,1]` at `t ∈ [9,10]` regardless of refit, so the "before"
        // assertion asked for something geometrically false (the ray legitimately hits item 0's
        // ORIGINAL box), not a validator/BVH bug. Moved item 0's initial box to `x ∈ [20,21]`
        // — behind the ray's origin in the `-X` direction it walks, so `t` would have to be
        // negative to reach it — so "before" is genuinely empty, and refit (unchanged) moves it
        // into the ray's actual path at `x ∈ [9,11]` for the "after" assertion to find.
        let items = vec![(aabb([20.0, 0.0, 0.0], [21.0, 1.0, 1.0]), 0usize), (aabb([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]), 1usize)];
        let mut bvh = Bvh::build(items);
        assert!(bvh.query_ray([10.0, 0.5, 0.5], [-1.0, 0.0, 0.0]).is_empty());
        bvh.refit(&|item: &usize| if *item == 0 { aabb([9.0, 0.0, 0.0], [11.0, 1.0, 1.0]) } else { aabb([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]) });
        assert_eq!(bvh.query_ray([10.0, 0.5, 0.5], [0.0, 0.0, 1.0]), vec![&0]);
    }
}
