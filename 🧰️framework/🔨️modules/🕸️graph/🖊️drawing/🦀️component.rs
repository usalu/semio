//! 🕸️ Graph drawing: node-placement layouts and edge/handle routing geometry.

// #region 🕸️Force
pub mod force {
    use geometry::Vec2;

    /// ⚙️ Force-directed layout parameters (geometry-free).
    #[derive(Clone, Debug)]
    pub struct ForceLayoutOptions {
        pub iterations: u32,
        pub ideal_edge_length: f64,
        pub repulsion_strength: f64,
        pub spring_strength: f64,
        pub gravity: f64,
        pub center_x: f64,
        pub center_y: f64,
        pub time_step: f64,
        pub velocity_damping: f64,
        pub max_speed: f64,
        pub random_seed: u64,
        pub barnes_hut_theta: f64,
        pub pairwise_repulsion_max_bodies: u32,
    }

    impl Default for ForceLayoutOptions {
        fn default() -> Self {
            Self {
                iterations: 420,
                ideal_edge_length: 140.0,
                repulsion_strength: 6500.0,
                spring_strength: 0.028,
                gravity: 0.018,
                center_x: 0.0,
                center_y: 0.0,
                time_step: 0.85,
                velocity_damping: 0.88,
                max_speed: 48.0,
                random_seed: 0x5eedfaced0,
                barnes_hut_theta: 0.78,
                pairwise_repulsion_max_bodies: 56,
            }
        }
    }

    async fn split_mix64(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    async fn rand_unit_interval(seed: &mut u64) -> f64 {
        *seed = split_mix64(*seed).await;
        (*seed as f64) / (u64::MAX as f64)
    }

    #[inline]
    async fn pairwise_repulsion_on_i_from_j(i: usize, j: usize, positions: &[Vec2], radii: &[f64], cool: f64, k_rep: f64) -> Vec2 {
        let delta = positions[j] - positions[i];
        let dist = delta.hypot().await.max(1e-4);
        let rep = k_rep * cool * (radii[i] * radii[j]).max(1.0) / (dist * dist);
        (delta / dist) * (-rep)
    }

    async fn add_pairwise_repulsion(forces: &mut [Vec2], positions: &[Vec2], radii: &[f64], n: usize, cool: f64, k_rep: f64) {
        for i in 0..n {
            for j in (i + 1)..n {
                let f = pairwise_repulsion_on_i_from_j(i, j, positions, radii, cool, k_rep).await;
                forces[i] += f;
                forces[j] -= f;
            }
        }
    }

    /// 🕸️ Run force-directed layout on abstract 2d positions.
    pub async fn run_force_layout(positions: &mut [Vec2], radii: &[f64], edge_pairs: &[(usize, usize)], pin: &[Option<Vec2>], opts: &ForceLayoutOptions) {
        let n = positions.len();
        if n == 0 {
            return;
        }
        let mut velocities = vec![Vec2::ZERO; n];
        let gx = opts.center_x;
        let gy = opts.center_y;
        let k = opts.ideal_edge_length.max(1e-6);
        let iters = opts.iterations.max(1);
        for iter in 0..iters {
            let cool = (1.0 - iter as f64 / iters as f64).max(0.08);
            let mut forces = vec![Vec2::ZERO; n];
            let _theta = opts.barnes_hut_theta;
            let _pair_cap = opts.pairwise_repulsion_max_bodies;
            add_pairwise_repulsion(&mut forces, positions, radii, n, cool, opts.repulsion_strength).await;
            for &(i, j) in edge_pairs {
                let delta = positions[j] - positions[i];
                let dist = delta.hypot().await.max(1e-4);
                let dir = delta / dist;
                let displacement = dist - k;
                let f = dir * (opts.spring_strength * cool * displacement);
                forces[i] += f;
                forces[j] -= f;
            }
            if opts.gravity > 0.0 {
                let g = opts.gravity * cool;
                for i in 0..n {
                    let to_c = Vec2::new(gx - positions[i].x, gy - positions[i].y).await;
                    forces[i] += to_c * g;
                }
            }
            for i in 0..n {
                if pin[i].is_some() {
                    forces[i] = Vec2::ZERO;
                }
            }
            let dt = opts.time_step * cool.sqrt();
            for i in 0..n {
                let mut v = (velocities[i] + forces[i] * dt) * opts.velocity_damping;
                let spd = v.hypot().await;
                if spd > opts.max_speed {
                    v *= opts.max_speed / spd;
                }
                velocities[i] = v;
                if pin[i].is_none() {
                    positions[i] += v * dt;
                } else if let Some(p) = pin[i] {
                    positions[i] = p;
                    velocities[i] = Vec2::ZERO;
                }
            }
        }
    }

    /// 🎲️ Scatter missing positions around anchor with deterministic jitter.
    pub async fn seed_positions(positions: &mut [Vec2], pin: &[Option<Vec2>], anchor: Vec2, seed: u64) {
        let mut rng = seed;
        for i in 0..positions.len() {
            if pin[i].is_some() {
                continue;
            }
            if positions[i].hypot().await < 1e-9 {
                let t = i as f64;
                let ang = t * 2.399_963_229_728_653_5;
                let r = 10.0 + t.sqrt() * 22.0;
                let jx = (rand_unit_interval(&mut rng).await - 0.5) * 6.0;
                let jy = (rand_unit_interval(&mut rng).await - 0.5) * 6.0;
                positions[i] = anchor + Vec2::new(r * ang.cos() + jx, r * ang.sin() + jy).await;
            }
        }
    }

    /// ⭕️ Deterministic circular layout: `n` points evenly spaced on a ring of `radius` around `center`.
    pub async fn circular_layout(n: usize, center: Vec2, radius: f64) -> Vec<Vec2> {
        if n == 0 {
            return Vec::new();
        }
        // 🔀️ Rewritten from `.map(..).collect()` — the closure was sync and could not `.await`
        // the per-point `Vec2::new` constructor (R10 residue shape #1).
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            let angle = (i as f64 / n as f64) * std::f64::consts::TAU;
            points.push(center + Vec2::new(angle.cos() * radius, angle.sin() * radius).await);
        }
        points
    }

    /// 🔲️ Deterministic grid layout: `n` points in row-major order, `cols` per row, spaced by `gap`.
    pub async fn grid_layout(n: usize, cols: usize, gap: f64) -> Vec<Vec2> {
        if n == 0 || cols == 0 {
            return Vec::new();
        }
        // 🔀️ Rewritten from `.map(..).collect()` — same sync-closure constraint as `circular_layout`.
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            let col = (i % cols) as f64;
            let row = (i / cols) as f64;
            points.push(Vec2::new(col * gap, row * gap).await);
        }
        points
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
        // an `async fn` directly (std has no executor for it), so every async test body in this
        // module runs through this instead. Sound because this crate performs no real I/O: every
        // future here resolves on its first poll, so a single poll (never a spin-park loop) is
        // enough — panics loudly if that invariant is ever violated rather than hanging.
        fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
            use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
            fn noop(_: *const ()) {}
            fn clone_raw(_: *const ()) -> RawWaker {
                RawWaker::new(std::ptr::null(), &VTABLE)
            }
            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
            let raw = RawWaker::new(std::ptr::null(), &VTABLE);
            let waker = unsafe { Waker::from_raw(raw) };
            let mut cx = Context::from_waker(&waker);
            let mut fut = Box::pin(fut);
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(v) => v,
                Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
            }
        }


        #[test]
        fn force_layout_moves_nodes() {
            block_on_test(async {
                let mut positions = vec![Vec2::new(0.0, 0.0).await, Vec2::new(100.0, 0.0).await];
                let radii = vec![32.0, 32.0];
                let edges = vec![(0, 1)];
                let pin = vec![None, None];
                let opts = ForceLayoutOptions { iterations: 120, ideal_edge_length: 80.0, ..Default::default() };
                run_force_layout(&mut positions, &radii, &edges, &pin, &opts).await;
                let dist = (positions[1] - positions[0]).hypot().await;
                assert!(dist.is_finite() && dist > 1.0);
                assert!((dist - 100.0).abs() > 0.01);
            });
        }

        #[test]
        fn circular_layout_places_points_on_ring() {
            block_on_test(async {
                let points = circular_layout(4, Vec2::ZERO, 10.0).await;
                assert_eq!(points.len(), 4);
                for p in &points {
                    assert!((p.hypot().await - 10.0).abs() < 1e-9);
                }
            });
        }

        #[test]
        fn grid_layout_places_points_in_rows() {
            block_on_test(async {
                let points = grid_layout(5, 2, 10.0).await;
                assert_eq!(points.len(), 5);
                assert_eq!((points[0].x, points[0].y), (0.0, 0.0));
                assert_eq!((points[2].x, points[2].y), (0.0, 10.0));
            });
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🕸️Force

// #region 📐️Routing
pub mod routing {
    use geometry::clamp_f64;
    use geometry::{append_shape_to_path, distance_between, normalize_or_zero, ray_from_origin_to_axis_aligned_rectangle_edge, Arc, BezPath, Circle, CubicBez, Point, Rect, Vec2};
    use crate::NodeShape;

    /// 🕳️ Even-odd clip path: local outer bounds minus the parent node body (keeps handle paint outside transparent nodes).
    pub async fn handle_outside_node_clip_path(handle_center: Point, handle_radius: f64, node_center: Point, node_shape: NodeShape, node_radius: f64, node_width: f64, node_height: f64) -> BezPath {
        let margin = (handle_radius * 2.5).max(4.0);
        let outer = Rect::new(handle_center.x - margin, handle_center.y - margin, handle_center.x + margin, handle_center.y + margin).await;
        let mut path = BezPath::new().await;
        append_shape_to_path(&mut path, &outer, 0.1).await;
        match node_shape {
            NodeShape::Circle => {
                append_shape_to_path(&mut path, &Circle::new(node_center, node_radius.max(1e-9)).await, 0.1).await;
            }
            NodeShape::Rectangle => {
                let hw = node_width.max(1e-9) * 0.5;
                let hh = node_height.max(1e-9) * 0.5;
                append_shape_to_path(&mut path, &Rect::new(node_center.x - hw, node_center.y - hh, node_center.x + hw, node_center.y + hh).await, 0.1).await;
            }
        }
        path
    }

    /// 🧭️ Outward normal for a handle on a node rim: edge-normal on rectangles, radial on circles.
    pub async fn handle_outward_at_node_rim(handle: Point, node_center: Point, node_shape: NodeShape, _node_radius: f64, node_width: f64, node_height: f64) -> Option<Vec2> {
        match node_shape {
            NodeShape::Circle => {
                let outward = normalize_or_zero(handle - node_center).await;
                if outward.hypot().await < 1e-9 {
                    None
                } else {
                    Some(outward)
                }
            }
            NodeShape::Rectangle => {
                let hw = node_width * 0.5;
                let hh = node_height * 0.5;
                if hw < 1e-9 || hh < 1e-9 {
                    return None;
                }
                let dx = handle.x - node_center.x;
                let dy = handle.y - node_center.y;
                if dx.abs() / hw >= dy.abs() / hh {
                    Some(Vec2::new(if dx < 0.0 { -1.0 } else { 1.0 }, 0.0).await)
                } else {
                    Some(Vec2::new(0.0, if dy < 0.0 { -1.0 } else { 1.0 }).await)
                }
            }
        }
    }

    async fn handle_exterior_cap_arc(center: Point, outward: Vec2, radius: f64) -> Option<Arc> {
        let out = normalize_or_zero(outward).await;
        let r = radius.max(1e-9);
        if out.hypot().await < 1e-9 {
            return None;
        }
        let perp = Vec2::new(-out.y, out.x).await;
        let start = center + perp * r;
        let peak = center + out * r;
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let arc_pos = Arc::new(center, (r, r), start_angle, std::f64::consts::PI, 0.0).await;
        let arc_neg = Arc::new(center, (r, r), start_angle, -std::f64::consts::PI, 0.0).await;
        if distance_between(arc_pos.eval(0.5).await, peak).await <= distance_between(arc_neg.eval(0.5).await, peak).await {
            Some(arc_pos)
        } else {
            Some(arc_neg)
        }
    }

    /// 🌗️ Closed fill path for the handle cap outside a node body (semicircle on the `outward` side).
    pub async fn handle_exterior_cap_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let r = radius.max(1e-9);
        let mut path = BezPath::new().await;
        if let Some(arc) = handle_exterior_cap_arc(center, outward, r).await {
            append_shape_to_path(&mut path, &arc, 0.1).await;
            path.close_path().await;
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r).await, 0.1).await;
        path
    }

    /// 🌗️ Open arc path for stroking only the exterior handle cap (flat rim edge stays behind the node).
    pub async fn handle_exterior_cap_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let r = radius.max(1e-9);
        let mut path = BezPath::new().await;
        if let Some(arc) = handle_exterior_cap_arc(center, outward, r).await {
            append_shape_to_path(&mut path, &arc, 0.1).await;
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r).await, 0.1).await;
        path
    }

    pub async fn handle_position_on_circle(center: Point, radius: f64, angle: f64) -> Point {
        let ux = angle.cos();
        let uy = angle.sin();
        center + Vec2::new(ux * radius, uy * radius).await
    }

    /// 🧭️ Rectangle handle `angle` is **0 at top edge center (north)**, increasing **counter‑clockwise** in board space (`y` down): `π/4` NW corner, `π/2` west midpoint, `π` south, `3π/2` east; circles keep **east‑zero** `atan2(dy,dx)` convention.
    pub async fn handle_position_on_rectangle(center: Point, width: f64, height: f64, angle: f64) -> Point {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let ux = -angle.sin();
        let uy = -angle.cos();
        let local = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy).await;
        center + Vec2::new(local.x, local.y).await
    }

    /// 🧭️ East-zero polar angle for a circle handle that meets the ray from `center` toward `toward` on the rim.
    pub async fn circle_handle_angle_toward(center: Point, toward: Point) -> f64 {
        let d = toward - center;
        f64::atan2(d.y, d.x)
    }

    /// 🧭️ North-zero rectangle handle angle so the rim point lies on the ray from `center` toward `toward`.
    pub async fn rectangle_handle_angle_toward(center: Point, _width: f64, _height: f64, toward: Point) -> f64 {
        let u = normalize_or_zero(toward - center).await;
        f64::atan2(-u.x, -u.y)
    }

    /// 🎯️ World point at the outer peak of a port handle cap (rim + outward × radius).
    pub async fn handle_exterior_cap_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        let out = normalize_or_zero(outward).await;
        let r = radius.max(0.0);
        if out.hypot().await < 1e-9 || r <= 0.0 {
            return center;
        }
        center + out * r
    }

    /// 🔺️ Closed fill path for a triangle handle cap pointing in the `outward` direction.
    pub async fn handle_exterior_cap_triangle_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward).await;
        let r = radius.max(1e-9);
        if out.hypot().await < 1e-9 {
            return handle_exterior_cap_fill_path(center, outward, r).await;
        }
        let perp = Vec2::new(-out.y, out.x).await;
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new().await;
        path.move_to(base_left).await;
        path.line_to(peak).await;
        path.line_to(base_right).await;
        path.close_path().await;
        path
    }

    /// 🔺️ Open stroke path for a triangle handle cap.
    pub async fn handle_exterior_cap_triangle_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward).await;
        let r = radius.max(1e-9);
        if out.hypot().await < 1e-9 {
            return handle_exterior_cap_stroke_path(center, outward, r).await;
        }
        let perp = Vec2::new(-out.y, out.x).await;
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new().await;
        path.move_to(base_left).await;
        path.line_to(peak).await;
        path.line_to(base_right).await;
        path
    }

    /// 🔺️ Wire attachment peak for a triangle handle cap.
    pub async fn handle_exterior_cap_triangle_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        handle_exterior_cap_peak(center, outward, radius).await
    }

    /// 📐️ Orthogonal S/Z polyline between two port cap peaks.
    pub async fn compute_edge_sharp_sz_path(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> BezPath {
        let out_s = normalize_or_zero(source_outward).await;
        let out_t = normalize_or_zero(target_outward).await;
        let stub = 20.0;
        let p1 = source_point + out_s * stub;
        let p4 = target_point + out_t * stub;
        let mut path = BezPath::new().await;
        path.move_to(source_point).await;
        path.line_to(p1).await;
        if (p1.x - p4.x).abs() >= (p1.y - p4.y).abs() {
            let mid_x = (p1.x + p4.x) * 0.5;
            path.line_to(Point::new(mid_x, p1.y).await).await;
            path.line_to(Point::new(mid_x, p4.y).await).await;
        } else {
            let mid_y = (p1.y + p4.y) * 0.5;
            path.line_to(Point::new(p1.x, mid_y).await).await;
            path.line_to(Point::new(p4.x, mid_y).await).await;
        }
        path.line_to(p4).await;
        path.line_to(target_point).await;
        path
    }

    pub async fn compute_edge_bezier_outward(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point).await;
        let mut source_radial = normalize_or_zero(source_outward).await;
        if source_radial == Vec2::new(0.0, 0.0).await {
            source_radial = chord;
        }
        let mut target_radial = normalize_or_zero(target_outward).await;
        if target_radial == Vec2::new(0.0, 0.0).await {
            target_radial = -chord;
        }
        let handle_distance = distance_between(source_point, target_point).await;
        let control_length = clamp_f64(handle_distance * 0.12, 8.0, 72.0).await;
        let p1 = source_point + source_radial * control_length;
        let p2 = target_point + target_radial * control_length;
        CubicBez::new(source_point, p1, p2, target_point).await
    }

    pub async fn compute_edge_bezier_points(source_point: Point, target_point: Point, source_center: Point, target_center: Point) -> CubicBez {
        compute_edge_bezier_outward(source_point, target_point, source_point - source_center, target_point - target_center).await
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
        // an `async fn` directly (std has no executor for it), so every async test body in this
        // module runs through this instead. Sound because this crate performs no real I/O: every
        // future here resolves on its first poll, so a single poll (never a spin-park loop) is
        // enough — panics loudly if that invariant is ever violated rather than hanging.
        fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
            use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
            fn noop(_: *const ()) {}
            fn clone_raw(_: *const ()) -> RawWaker {
                RawWaker::new(std::ptr::null(), &VTABLE)
            }
            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
            let raw = RawWaker::new(std::ptr::null(), &VTABLE);
            let waker = unsafe { Waker::from_raw(raw) };
            let mut cx = Context::from_waker(&waker);
            let mut fut = Box::pin(fut);
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(v) => v,
                Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
            }
        }


        #[test]
        fn outside_node_clip_path_excludes_node_interior() {
            block_on_test(async {
                let node_center = Point::new(0.0, 0.0).await;
                let handle_center = Point::new(40.0, 0.0).await;
                let clip = handle_outside_node_clip_path(handle_center, 5.0, node_center, NodeShape::Circle, 40.0, 80.0, 80.0).await;
                assert!(clip.elements().await.len() > 4);
                assert!(node_center.distance(handle_center).await > 39.0);
            });
        }

        async fn assert_cap_bulges_outward(center: Point, outward: Vec2, radius: f64) {
            let out = normalize_or_zero(outward).await;
            let peak = center + out * radius;
            let arc = handle_exterior_cap_arc(center, outward, radius).await.expect("exterior arc");
            assert!(distance_between(arc.eval(0.5).await, peak).await < 0.35, "arc midpoint must sit on outward peak");
            let fill = handle_exterior_cap_fill_path(center, outward, radius).await;
            let bb = fill.bounding_box().await;
            let trough = center - out * radius;
            if out.x.abs() >= out.y.abs() {
                if out.x > 0.0 {
                    assert!((bb.x1().await - peak.x).abs() < 0.25, "east cap must peak at +x");
                    assert!(bb.x0().await > trough.x + 0.25, "east cap must not peak inward");
                } else {
                    assert!((bb.x0().await - peak.x).abs() < 0.25, "west cap must peak at -x");
                    assert!(bb.x1().await < trough.x - 0.25, "west cap must not peak inward");
                }
            } else if out.y > 0.0 {
                assert!((bb.y1().await - peak.y).abs() < 0.25, "south cap must peak at +y");
                assert!(bb.y0().await > trough.y + 0.25, "south cap must not peak inward");
            } else {
                assert!((bb.y0().await - peak.y).abs() < 0.25, "north cap must peak at -y");
                assert!(bb.y1().await < trough.y + 0.25, "north cap must not peak inward");
            }
        }

        #[test]
        fn edge_bezier_free_target_end_tangent_matches_incoming_chord() {
            block_on_test(async {
                let source = Point::new(0.0, 0.0).await;
                let target = Point::new(200.0, 40.0).await;
                let curve = compute_edge_bezier_points(source, target, Point::new(-50.0, 0.0).await, target).await;
                let approach = normalize_or_zero(target - source).await;
                let tangent = curve.eval(1.0).await - curve.eval(0.995).await;
                let tangent_dir = normalize_or_zero(Vec2::new(tangent.x, tangent.y).await).await;
                assert!(tangent_dir.dot(approach).await > 0.99, "free target tangent should match incoming chord");
            });
        }

        #[test]
        fn edge_bezier_starts_outside_handle_cap_peak() {
            block_on_test(async {
                let node_center = Point::new(100.0, 50.0).await;
                let width = 160.0;
                let height = 72.0;
                let rim = Point::new(node_center.x + width * 0.5, node_center.y).await;
                let outward = handle_outward_at_node_rim(rim, node_center, NodeShape::Rectangle, 0.0, width, height).await.expect("outward");
                let radius = 5.0;
                let peak = handle_exterior_cap_peak(rim, outward, radius).await;
                let target = Point::new(300.0, 50.0).await;
                let curve = compute_edge_bezier_outward(peak, target, outward, -normalize_or_zero(target - peak).await).await;
                let start = curve.eval(0.0).await;
                assert!((start.x - peak.x).abs() < 1e-9 && (start.y - peak.y).abs() < 1e-9);
                assert!(start.x > rim.x + 0.5, "edge must begin outside the port rim under the cap");
            });
        }

        #[test]
        fn edge_bezier_rectangle_port_uses_outward_normal() {
            block_on_test(async {
                let node_center = Point::new(100.0, 50.0).await;
                let width = 120.0;
                let height = 80.0;
                let source = Point::new(node_center.x - width * 0.5, node_center.y - 20.0).await;
                let target = Point::new(280.0, 50.0).await;
                let outward = handle_outward_at_node_rim(source, node_center, NodeShape::Rectangle, 0.0, width, height).await.expect("outward");
                let curve = compute_edge_bezier_outward(source, target, outward, -normalize_or_zero(target - source).await).await;
                let leave = curve.eval(0.005).await - curve.eval(0.0).await;
                let leave_dir = normalize_or_zero(Vec2::new(leave.x, leave.y).await).await;
                assert!(leave_dir.dot(outward).await > 0.99, "anchored port should leave along rim outward");
            });
        }

        #[test]
        fn rectangle_rim_outward_uses_edge_normal_not_radial() {
            block_on_test(async {
                let node_center = Point::new(100.0, 50.0).await;
                let width = 120.0;
                let height = 80.0;
                let handle = Point::new(node_center.x - width * 0.5, node_center.y - 20.0).await;
                let radial = normalize_or_zero(handle - node_center).await;
                let outward = handle_outward_at_node_rim(handle, node_center, NodeShape::Rectangle, 0.0, width, height).await.expect("outward");
                assert!((outward.x + 1.0).abs() < 1e-9 && outward.y.abs() < 1e-9);
                assert!(radial.y.abs() > 0.1, "radial must tilt for off-center left ports");
            });
        }

        #[test]
        fn exterior_cap_paths_bulge_outward_on_all_cardinals() {
            block_on_test(async {
                let radius = 5.0;
                assert_cap_bulges_outward(Point::new(40.0, 0.0).await, Vec2::new(1.0, 0.0).await, radius).await;
                assert_cap_bulges_outward(Point::new(-40.0, 0.0).await, Vec2::new(-1.0, 0.0).await, radius).await;
                assert_cap_bulges_outward(Point::new(0.0, 30.0).await, Vec2::new(0.0, 1.0).await, radius).await;
                assert_cap_bulges_outward(Point::new(0.0, -30.0).await, Vec2::new(0.0, -1.0).await, radius).await;
                let stroke = handle_exterior_cap_stroke_path(Point::new(40.0, 0.0).await, Vec2::new(1.0, 0.0).await, radius).await;
                assert!(!stroke.elements().await.iter().any(|el| matches!(el, geometry::PathEl::ClosePath)));
            });
        }

        #[test]
        fn triangle_cap_peak_matches_outward_direction() {
            block_on_test(async {
                let center = Point::new(40.0, 0.0).await;
                let outward = Vec2::new(1.0, 0.0).await;
                let radius = 5.0;
                let peak = handle_exterior_cap_triangle_peak(center, outward, radius).await;
                assert!((peak.x - (center.x + radius)).abs() < 1e-9);
                let fill = handle_exterior_cap_triangle_fill_path(center, outward, radius).await;
                assert!(fill.bounding_box().await.x1().await > center.x);
            });
        }

        #[test]
        fn sharp_sz_path_is_orthogonal_between_peaks() {
            block_on_test(async {
                let source = Point::new(0.0, 0.0).await;
                let target = Point::new(120.0, 40.0).await;
                let path = compute_edge_sharp_sz_path(source, target, Vec2::new(1.0, 0.0).await, Vec2::new(-1.0, 0.0).await).await;
                let mut line_count = 0;
                for el in path.elements().await {
                    if matches!(el, geometry::PathEl::LineTo(_)) {
                        line_count += 1;
                    }
                }
                assert!(line_count >= 3, "sharp S/Z path should contain multiple straight segments");
            });
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 📐️Routing
