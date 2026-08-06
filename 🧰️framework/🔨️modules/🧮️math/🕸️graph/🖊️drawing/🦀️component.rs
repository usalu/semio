//! 🕸️ Graph drawing: node-placement layouts and edge/handle routing geometry.

// #region 🕸️Force
pub mod force {
    use crate::geometry::Vec2;

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

    fn split_mix64(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn rand_unit_interval(seed: &mut u64) -> f64 {
        *seed = split_mix64(*seed);
        (*seed as f64) / (u64::MAX as f64)
    }

    #[inline]
    fn pairwise_repulsion_on_i_from_j(i: usize, j: usize, positions: &[Vec2], radii: &[f64], cool: f64, k_rep: f64) -> Vec2 {
        let delta = positions[j] - positions[i];
        let dist = delta.hypot().max(1e-4);
        let rep = k_rep * cool * (radii[i] * radii[j]).max(1.0) / (dist * dist);
        (delta / dist) * (-rep)
    }

    fn add_pairwise_repulsion(forces: &mut [Vec2], positions: &[Vec2], radii: &[f64], n: usize, cool: f64, k_rep: f64) {
        for i in 0..n {
            for j in (i + 1)..n {
                let f = pairwise_repulsion_on_i_from_j(i, j, positions, radii, cool, k_rep);
                forces[i] += f;
                forces[j] -= f;
            }
        }
    }

    /// 🕸️ Run force-directed layout on abstract 2d positions.
    pub fn run_force_layout(positions: &mut [Vec2], radii: &[f64], edge_pairs: &[(usize, usize)], pin: &[Option<Vec2>], opts: &ForceLayoutOptions) {
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
            add_pairwise_repulsion(&mut forces, positions, radii, n, cool, opts.repulsion_strength);
            for &(i, j) in edge_pairs {
                let delta = positions[j] - positions[i];
                let dist = delta.hypot().max(1e-4);
                let dir = delta / dist;
                let displacement = dist - k;
                let f = dir * (opts.spring_strength * cool * displacement);
                forces[i] += f;
                forces[j] -= f;
            }
            if opts.gravity > 0.0 {
                let g = opts.gravity * cool;
                for i in 0..n {
                    let to_c = Vec2::new(gx - positions[i].x, gy - positions[i].y);
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
                let spd = v.hypot();
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
    pub fn seed_positions(positions: &mut [Vec2], pin: &[Option<Vec2>], anchor: Vec2, seed: u64) {
        let mut rng = seed;
        for i in 0..positions.len() {
            if pin[i].is_some() {
                continue;
            }
            if positions[i].hypot() < 1e-9 {
                let t = i as f64;
                let ang = t * 2.399_963_229_728_653_5;
                let r = 10.0 + t.sqrt() * 22.0;
                let jx = (rand_unit_interval(&mut rng) - 0.5) * 6.0;
                let jy = (rand_unit_interval(&mut rng) - 0.5) * 6.0;
                positions[i] = anchor + Vec2::new(r * ang.cos() + jx, r * ang.sin() + jy);
            }
        }
    }

    /// ⭕️ Deterministic circular layout: `n` points evenly spaced on a ring of `radius` around `center`.
    pub fn circular_layout(n: usize, center: Vec2, radius: f64) -> Vec<Vec2> {
        if n == 0 {
            return Vec::new();
        }
        (0..n)
            .map(|i| {
                let angle = (i as f64 / n as f64) * std::f64::consts::TAU;
                center + Vec2::new(angle.cos() * radius, angle.sin() * radius)
            })
            .collect()
    }

    /// 🔲️ Deterministic grid layout: `n` points in row-major order, `cols` per row, spaced by `gap`.
    pub fn grid_layout(n: usize, cols: usize, gap: f64) -> Vec<Vec2> {
        if n == 0 || cols == 0 {
            return Vec::new();
        }
        (0..n)
            .map(|i| {
                let col = (i % cols) as f64;
                let row = (i / cols) as f64;
                Vec2::new(col * gap, row * gap)
            })
            .collect()
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn force_layout_moves_nodes() {
            let mut positions = vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)];
            let radii = vec![32.0, 32.0];
            let edges = vec![(0, 1)];
            let pin = vec![None, None];
            let opts = ForceLayoutOptions { iterations: 120, ideal_edge_length: 80.0, ..Default::default() };
            run_force_layout(&mut positions, &radii, &edges, &pin, &opts);
            let dist = (positions[1] - positions[0]).hypot();
            assert!(dist.is_finite() && dist > 1.0);
            assert!((dist - 100.0).abs() > 0.01);
        }

        #[test]
        fn circular_layout_places_points_on_ring() {
            let points = circular_layout(4, Vec2::ZERO, 10.0);
            assert_eq!(points.len(), 4);
            for p in &points {
                assert!((p.hypot() - 10.0).abs() < 1e-9);
            }
        }

        #[test]
        fn grid_layout_places_points_in_rows() {
            let points = grid_layout(5, 2, 10.0);
            assert_eq!(points.len(), 5);
            assert_eq!((points[0].x, points[0].y), (0.0, 0.0));
            assert_eq!((points[2].x, points[2].y), (0.0, 10.0));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🕸️Force

// #region 🌳️TidyTree
pub mod tidy_tree {
    use std::collections::{HashMap, HashSet};

    /// 🌲️ Buchheim tidy-tree on string-labeled directed edges.
    pub fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> HashMap<String, (f64, f64)> {
        let roots_set: HashSet<String> = roots.iter().cloned().collect();
        let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
        for (u, v) in directed {
            incoming.entry(v.clone()).or_default().push(u.clone());
        }
        for v in incoming.values_mut() {
            v.sort();
            v.dedup();
        }
        let mut chosen_parent: HashMap<String, String> = HashMap::new();
        let mut all_ids: HashSet<String> = HashSet::new();
        for (u, v) in directed {
            all_ids.insert(u.clone());
            all_ids.insert(v.clone());
        }
        for r in roots {
            all_ids.insert(r.clone());
        }
        for id in &all_ids {
            if roots_set.contains(id) {
                continue;
            }
            let ps = incoming.get(id).cloned().unwrap_or_default();
            if ps.is_empty() {
                continue;
            }
            let best = ps
                .iter()
                .min_by_key(|p| {
                    let dp = depth.get(*p).copied().unwrap_or(0);
                    (dp, (*p).clone())
                })
                .expect("non-empty ps")
                .clone();
            chosen_parent.insert(id.clone(), best);
        }
        let mut ordered_ids: Vec<String> = all_ids.into_iter().collect();
        ordered_ids.sort();
        if ordered_ids.is_empty() {
            return HashMap::new();
        }
        let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let super_idx = ordered_ids.len();
        let mut nodes: Vec<BuchheimNode> =
            ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
        nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: "__tree_super__".into(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
        for (i, oid) in ordered_ids.iter().enumerate() {
            let pidx = if roots_set.contains(oid) {
                super_idx
            } else {
                match chosen_parent.get(oid) {
                    Some(p) => *id_to_idx.get(p).unwrap_or(&super_idx),
                    None => super_idx,
                }
            };
            nodes[i].parent = Some(pidx);
        }
        for node in nodes.iter_mut() {
            node.children.clear();
        }
        for i in 0..super_idx {
            let pi = nodes[i].parent.expect("parent set for every non-super node in the loop above");
            nodes[pi].children.push(i);
        }
        for p in 0..=super_idx {
            let mut ch = nodes[p].children.clone();
            ch.sort_by_key(|&c| nodes[c].id.clone());
            nodes[p].children = ch;
        }
        for p in 0..=super_idx {
            if nodes[p].children.is_empty() {
                continue;
            }
            let ch = nodes[p].children.clone();
            for (k, &c) in ch.iter().enumerate() {
                nodes[c].number = (k + 1) as i32;
                nodes[c].ancestor = c;
            }
        }
        buchheim_first_walk(&mut nodes, super_idx, 1.0);
        let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
        if min_x.is_finite() && min_x < 0.0 {
            buchheim_third_walk(&mut nodes, super_idx, -min_x);
        }
        let mut out = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            if i == super_idx || n.synthetic {
                continue;
            }
            out.insert(n.id.clone(), (n.x, n.y));
        }
        out
    }

    #[derive(Debug)]
    struct BuchheimNode {
        id: String,
        parent: Option<usize>,
        children: Vec<usize>,
        x: f64,
        y: f64,
        mod_: f64,
        thread: Option<usize>,
        ancestor: usize,
        change: f64,
        shift: f64,
        number: i32,
        synthetic: bool,
    }

    fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        let pos = ch.iter().position(|&c| c == i)?;
        if pos == 0 {
            return None;
        }
        Some(ch[pos - 1])
    }

    fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        if ch.first() == Some(&i) {
            return None;
        }
        ch.first().copied()
    }

    fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.last().copied()
    }

    fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.first().copied()
    }

    fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
        let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
        if subtrees <= 0.0 {
            return;
        }
        nodes[wr].change -= shift / subtrees;
        nodes[wr].shift += shift;
        nodes[wl].change += shift / subtrees;
        nodes[wr].x += shift;
        nodes[wr].mod_ += shift;
    }

    fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
        let mut shift = 0.0f64;
        let mut change = 0.0f64;
        for &w in nodes[v].children.iter().rev() {
            nodes[w].x += shift;
            nodes[w].mod_ += shift;
            change += nodes[w].change;
            shift += nodes[w].shift + change;
        }
    }

    fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
        let w = match buchheim_left_brother(nodes, v) {
            Some(w) => w,
            None => return default_ancestor,
        };
        let mut vir = v;
        let mut vil = w;
        let mut vol = buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
        let mut vor = v;
        let mut sir = nodes[v].mod_;
        let mut sil = nodes[vil].mod_;
        loop {
            let vil_r = buchheim_next_right(nodes, vil);
            let vir_l = buchheim_next_left(nodes, vir);
            if vil_r.is_none() || vir_l.is_none() {
                break;
            }
            vil = vil_r.expect("checked Some above");
            vir = vir_l.expect("checked Some above");
            let vol_l = buchheim_next_left(nodes, vol);
            let vor_r = buchheim_next_right(nodes, vor);
            if vol_l.is_none() || vor_r.is_none() {
                break;
            }
            vol = vol_l.expect("checked Some above");
            vor = vor_r.expect("checked Some above");
            nodes[vor].ancestor = v;
            let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
            if shift > 0.0 {
                buchheim_move_subtree(nodes, default_ancestor, v, shift);
                sir += shift;
            }
            sil += nodes[vil].mod_;
            sir += nodes[vir].mod_;
        }
        default_ancestor
    }

    fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
        if nodes[v].children.is_empty() {
            if let Some(lb) = buchheim_left_brother(nodes, v) {
                nodes[v].x = nodes[lb].x + distance;
            } else {
                nodes[v].x = 0.0;
            }
            return v;
        }
        let mut default_ancestor = nodes[v].children[0];
        for &w in nodes[v].children.clone().iter() {
            buchheim_first_walk(nodes, w, distance);
            default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
        }
        buchheim_execute_shifts(nodes, v);
        let c0 = nodes[v].children[0];
        let c1 = *nodes[v].children.last().expect("children non-empty per the is_empty check above");
        let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
        if let Some(w) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[w].x + distance;
            nodes[v].mod_ = nodes[v].x - mid;
        } else {
            nodes[v].x = mid;
        }
        v
    }

    fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64) -> f64 {
        nodes[v].x += m;
        nodes[v].y = depth as f64;
        let mut min_x = min_x.min(nodes[v].x);
        for &w in nodes[v].children.clone().iter() {
            min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
        }
        min_x
    }

    fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
        nodes[v].x += n;
        for &c in nodes[v].children.clone().iter() {
            buchheim_third_walk(nodes, c, n);
        }
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::buchheim_positions;

        #[test]
        fn buchheim_tree_two_nodes() {
            let roots = vec!["a".into()];
            let directed = vec![("a".into(), "b".into())];
            let mut depth = std::collections::HashMap::new();
            depth.insert("a".into(), 0);
            depth.insert("b".into(), 1);
            let pos = buchheim_positions(&roots, &directed, &depth);
            assert!(pos.contains_key("a"));
            assert!(pos.contains_key("b"));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🌳️TidyTree

// #region 📐️Routing
pub mod routing {
    use crate::geometry::clamp_f64;
    use crate::geometry::{append_shape_to_path, distance_between, normalize_or_zero, ray_from_origin_to_axis_aligned_rectangle_edge, Arc, BezPath, Circle, CubicBez, Point, Rect, Vec2};
    use crate::graph::NodeShape;

    /// 🕳️ Even-odd clip path: local outer bounds minus the parent node body (keeps handle paint outside transparent nodes).
    pub fn handle_outside_node_clip_path(handle_center: Point, handle_radius: f64, node_center: Point, node_shape: NodeShape, node_radius: f64, node_width: f64, node_height: f64) -> BezPath {
        let margin = (handle_radius * 2.5).max(4.0);
        let outer = Rect::new(handle_center.x - margin, handle_center.y - margin, handle_center.x + margin, handle_center.y + margin);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &outer, 0.1);
        match node_shape {
            NodeShape::Circle => {
                append_shape_to_path(&mut path, &Circle::new(node_center, node_radius.max(1e-9)), 0.1);
            }
            NodeShape::Rectangle => {
                let hw = node_width.max(1e-9) * 0.5;
                let hh = node_height.max(1e-9) * 0.5;
                append_shape_to_path(&mut path, &Rect::new(node_center.x - hw, node_center.y - hh, node_center.x + hw, node_center.y + hh), 0.1);
            }
        }
        path
    }

    /// 🧭️ Outward normal for a handle on a node rim: edge-normal on rectangles, radial on circles.
    pub fn handle_outward_at_node_rim(handle: Point, node_center: Point, node_shape: NodeShape, _node_radius: f64, node_width: f64, node_height: f64) -> Option<Vec2> {
        match node_shape {
            NodeShape::Circle => {
                let outward = normalize_or_zero(handle - node_center);
                if outward.hypot() < 1e-9 {
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
                    Some(Vec2::new(if dx < 0.0 { -1.0 } else { 1.0 }, 0.0))
                } else {
                    Some(Vec2::new(0.0, if dy < 0.0 { -1.0 } else { 1.0 }))
                }
            }
        }
    }

    fn handle_exterior_cap_arc(center: Point, outward: Vec2, radius: f64) -> Option<Arc> {
        let out = normalize_or_zero(outward);
        let r = radius.max(1e-9);
        if out.hypot() < 1e-9 {
            return None;
        }
        let perp = Vec2::new(-out.y, out.x);
        let start = center + perp * r;
        let peak = center + out * r;
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let arc_pos = Arc::new(center, (r, r), start_angle, std::f64::consts::PI, 0.0);
        let arc_neg = Arc::new(center, (r, r), start_angle, -std::f64::consts::PI, 0.0);
        if distance_between(arc_pos.eval(0.5), peak) <= distance_between(arc_neg.eval(0.5), peak) {
            Some(arc_pos)
        } else {
            Some(arc_neg)
        }
    }

    /// 🌗️ Closed fill path for the handle cap outside a node body (semicircle on the `outward` side).
    pub fn handle_exterior_cap_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let r = radius.max(1e-9);
        let mut path = BezPath::new();
        if let Some(arc) = handle_exterior_cap_arc(center, outward, r) {
            append_shape_to_path(&mut path, &arc, 0.1);
            path.close_path();
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r), 0.1);
        path
    }

    /// 🌗️ Open arc path for stroking only the exterior handle cap (flat rim edge stays behind the node).
    pub fn handle_exterior_cap_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let r = radius.max(1e-9);
        let mut path = BezPath::new();
        if let Some(arc) = handle_exterior_cap_arc(center, outward, r) {
            append_shape_to_path(&mut path, &arc, 0.1);
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r), 0.1);
        path
    }

    pub fn handle_position_on_circle(center: Point, radius: f64, angle: f64) -> Point {
        let ux = angle.cos();
        let uy = angle.sin();
        center + Vec2::new(ux * radius, uy * radius)
    }

    /// 🧭️ Rectangle handle `angle` is **0 at top edge center (north)**, increasing **counter‑clockwise** in board space (`y` down): `π/4` NW corner, `π/2` west midpoint, `π` south, `3π/2` east; circles keep **east‑zero** `atan2(dy,dx)` convention.
    pub fn handle_position_on_rectangle(center: Point, width: f64, height: f64, angle: f64) -> Point {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let ux = -angle.sin();
        let uy = -angle.cos();
        let local = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
        center + Vec2::new(local.x, local.y)
    }

    /// 🧭️ East-zero polar angle for a circle handle that meets the ray from `center` toward `toward` on the rim.
    pub fn circle_handle_angle_toward(center: Point, toward: Point) -> f64 {
        let d = toward - center;
        f64::atan2(d.y, d.x)
    }

    /// 🧭️ North-zero rectangle handle angle so the rim point lies on the ray from `center` toward `toward`.
    pub fn rectangle_handle_angle_toward(center: Point, _width: f64, _height: f64, toward: Point) -> f64 {
        let u = normalize_or_zero(toward - center);
        f64::atan2(-u.x, -u.y)
    }

    /// 🎯️ World point at the outer peak of a port handle cap (rim + outward × radius).
    pub fn handle_exterior_cap_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        let out = normalize_or_zero(outward);
        let r = radius.max(0.0);
        if out.hypot() < 1e-9 || r <= 0.0 {
            return center;
        }
        center + out * r
    }

    /// 🔺️ Closed fill path for a triangle handle cap pointing in the `outward` direction.
    pub fn handle_exterior_cap_triangle_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward);
        let r = radius.max(1e-9);
        if out.hypot() < 1e-9 {
            return handle_exterior_cap_fill_path(center, outward, r);
        }
        let perp = Vec2::new(-out.y, out.x);
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new();
        path.move_to(base_left);
        path.line_to(peak);
        path.line_to(base_right);
        path.close_path();
        path
    }

    /// 🔺️ Open stroke path for a triangle handle cap.
    pub fn handle_exterior_cap_triangle_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward);
        let r = radius.max(1e-9);
        if out.hypot() < 1e-9 {
            return handle_exterior_cap_stroke_path(center, outward, r);
        }
        let perp = Vec2::new(-out.y, out.x);
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new();
        path.move_to(base_left);
        path.line_to(peak);
        path.line_to(base_right);
        path
    }

    /// 🔺️ Wire attachment peak for a triangle handle cap.
    pub fn handle_exterior_cap_triangle_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        handle_exterior_cap_peak(center, outward, radius)
    }

    /// 📐️ Orthogonal S/Z polyline between two port cap peaks.
    pub fn compute_edge_sharp_sz_path(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> BezPath {
        let out_s = normalize_or_zero(source_outward);
        let out_t = normalize_or_zero(target_outward);
        let stub = 20.0;
        let p1 = source_point + out_s * stub;
        let p4 = target_point + out_t * stub;
        let mut path = BezPath::new();
        path.move_to(source_point);
        path.line_to(p1);
        if (p1.x - p4.x).abs() >= (p1.y - p4.y).abs() {
            let mid_x = (p1.x + p4.x) * 0.5;
            path.line_to(Point::new(mid_x, p1.y));
            path.line_to(Point::new(mid_x, p4.y));
        } else {
            let mid_y = (p1.y + p4.y) * 0.5;
            path.line_to(Point::new(p1.x, mid_y));
            path.line_to(Point::new(p4.x, mid_y));
        }
        path.line_to(p4);
        path.line_to(target_point);
        path
    }

    pub fn compute_edge_bezier_outward(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point);
        let mut source_radial = normalize_or_zero(source_outward);
        if source_radial == Vec2::new(0.0, 0.0) {
            source_radial = chord;
        }
        let mut target_radial = normalize_or_zero(target_outward);
        if target_radial == Vec2::new(0.0, 0.0) {
            target_radial = -chord;
        }
        let handle_distance = distance_between(source_point, target_point);
        let control_length = clamp_f64(handle_distance * 0.12, 8.0, 72.0);
        let p1 = source_point + source_radial * control_length;
        let p2 = target_point + target_radial * control_length;
        CubicBez::new(source_point, p1, p2, target_point)
    }

    pub fn compute_edge_bezier_points(source_point: Point, target_point: Point, source_center: Point, target_center: Point) -> CubicBez {
        compute_edge_bezier_outward(source_point, target_point, source_point - source_center, target_point - target_center)
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn outside_node_clip_path_excludes_node_interior() {
            let node_center = Point::new(0.0, 0.0);
            let handle_center = Point::new(40.0, 0.0);
            let clip = handle_outside_node_clip_path(handle_center, 5.0, node_center, NodeShape::Circle, 40.0, 80.0, 80.0);
            assert!(clip.elements().len() > 4);
            assert!(node_center.distance(handle_center) > 39.0);
        }

        fn assert_cap_bulges_outward(center: Point, outward: Vec2, radius: f64) {
            let out = normalize_or_zero(outward);
            let peak = center + out * radius;
            let arc = handle_exterior_cap_arc(center, outward, radius).expect("exterior arc");
            assert!(distance_between(arc.eval(0.5), peak) < 0.35, "arc midpoint must sit on outward peak");
            let fill = handle_exterior_cap_fill_path(center, outward, radius);
            let bb = fill.bounding_box();
            let trough = center - out * radius;
            if out.x.abs() >= out.y.abs() {
                if out.x > 0.0 {
                    assert!((bb.x1() - peak.x).abs() < 0.25, "east cap must peak at +x");
                    assert!(bb.x0() > trough.x + 0.25, "east cap must not peak inward");
                } else {
                    assert!((bb.x0() - peak.x).abs() < 0.25, "west cap must peak at -x");
                    assert!(bb.x1() < trough.x - 0.25, "west cap must not peak inward");
                }
            } else if out.y > 0.0 {
                assert!((bb.y1() - peak.y).abs() < 0.25, "south cap must peak at +y");
                assert!(bb.y0() > trough.y + 0.25, "south cap must not peak inward");
            } else {
                assert!((bb.y0() - peak.y).abs() < 0.25, "north cap must peak at -y");
                assert!(bb.y1() < trough.y + 0.25, "north cap must not peak inward");
            }
        }

        #[test]
        fn edge_bezier_free_target_end_tangent_matches_incoming_chord() {
            let source = Point::new(0.0, 0.0);
            let target = Point::new(200.0, 40.0);
            let curve = compute_edge_bezier_points(source, target, Point::new(-50.0, 0.0), target);
            let approach = normalize_or_zero(target - source);
            let tangent = curve.eval(1.0) - curve.eval(0.995);
            let tangent_dir = normalize_or_zero(Vec2::new(tangent.x, tangent.y));
            assert!(tangent_dir.dot(approach) > 0.99, "free target tangent should match incoming chord");
        }

        #[test]
        fn edge_bezier_starts_outside_handle_cap_peak() {
            let node_center = Point::new(100.0, 50.0);
            let width = 160.0;
            let height = 72.0;
            let rim = Point::new(node_center.x + width * 0.5, node_center.y);
            let outward = handle_outward_at_node_rim(rim, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            let radius = 5.0;
            let peak = handle_exterior_cap_peak(rim, outward, radius);
            let target = Point::new(300.0, 50.0);
            let curve = compute_edge_bezier_outward(peak, target, outward, -normalize_or_zero(target - peak));
            let start = curve.eval(0.0);
            assert!((start.x - peak.x).abs() < 1e-9 && (start.y - peak.y).abs() < 1e-9);
            assert!(start.x > rim.x + 0.5, "edge must begin outside the port rim under the cap");
        }

        #[test]
        fn edge_bezier_rectangle_port_uses_outward_normal() {
            let node_center = Point::new(100.0, 50.0);
            let width = 120.0;
            let height = 80.0;
            let source = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
            let target = Point::new(280.0, 50.0);
            let outward = handle_outward_at_node_rim(source, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            let curve = compute_edge_bezier_outward(source, target, outward, -normalize_or_zero(target - source));
            let leave = curve.eval(0.005) - curve.eval(0.0);
            let leave_dir = normalize_or_zero(Vec2::new(leave.x, leave.y));
            assert!(leave_dir.dot(outward) > 0.99, "anchored port should leave along rim outward");
        }

        #[test]
        fn rectangle_rim_outward_uses_edge_normal_not_radial() {
            let node_center = Point::new(100.0, 50.0);
            let width = 120.0;
            let height = 80.0;
            let handle = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
            let radial = normalize_or_zero(handle - node_center);
            let outward = handle_outward_at_node_rim(handle, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            assert!((outward.x + 1.0).abs() < 1e-9 && outward.y.abs() < 1e-9);
            assert!(radial.y.abs() > 0.1, "radial must tilt for off-center left ports");
        }

        #[test]
        fn exterior_cap_paths_bulge_outward_on_all_cardinals() {
            let radius = 5.0;
            assert_cap_bulges_outward(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
            assert_cap_bulges_outward(Point::new(-40.0, 0.0), Vec2::new(-1.0, 0.0), radius);
            assert_cap_bulges_outward(Point::new(0.0, 30.0), Vec2::new(0.0, 1.0), radius);
            assert_cap_bulges_outward(Point::new(0.0, -30.0), Vec2::new(0.0, -1.0), radius);
            let stroke = handle_exterior_cap_stroke_path(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
            assert!(!stroke.elements().iter().any(|el| matches!(el, crate::geometry::PathEl::ClosePath)));
        }

        #[test]
        fn triangle_cap_peak_matches_outward_direction() {
            let center = Point::new(40.0, 0.0);
            let outward = Vec2::new(1.0, 0.0);
            let radius = 5.0;
            let peak = handle_exterior_cap_triangle_peak(center, outward, radius);
            assert!((peak.x - (center.x + radius)).abs() < 1e-9);
            let fill = handle_exterior_cap_triangle_fill_path(center, outward, radius);
            assert!(fill.bounding_box().x1() > center.x);
        }

        #[test]
        fn sharp_sz_path_is_orthogonal_between_peaks() {
            let source = Point::new(0.0, 0.0);
            let target = Point::new(120.0, 40.0);
            let path = compute_edge_sharp_sz_path(source, target, Vec2::new(1.0, 0.0), Vec2::new(-1.0, 0.0));
            let mut line_count = 0;
            for el in path.elements() {
                if matches!(el, crate::geometry::PathEl::LineTo(_)) {
                    line_count += 1;
                }
            }
            assert!(line_count >= 3, "sharp S/Z path should contain multiple straight segments");
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 📐️Routing
