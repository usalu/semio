//! 🛤️ Tracks, global averaging and loop closure of the multi-view geometry: the track building, the rotation and translation averaging, the loop detection and the pose-graph optimisation. See [`super`].

use super::*;

// #region 🔖️Tracks
/// 🧵️ Iterative (no recursion, wasm32-safe) union-find over observation node indices, with path-halving.
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub(super) fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent[ra] = rb;
        }
    }
}

/// 🧵️ A feature track: one connected component of `(frame index, keypoint index in that frame)`
/// observations, linked transitively across pairwise matches.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct FeatureTracks {
    pub tracks: Vec<Vec<(usize, u32)>>,
}

/// 🧵️ Builds tracks via union-find over every `(frame, keypoint)` observation touched by
/// `pairwise_matches`: each `Match { a, b, .. }` in a `(frame_a, frame_b, matches)` triple unions
/// `(frame_a, a)` with `(frame_b, b)`. A track that would end up with two observations from the *same*
/// frame (an inconsistent match chain — the union merged two genuinely different points from that frame
/// via a bad transitive match) is dropped entirely rather than repaired, since picking a "correct"
/// sub-chain from a corrupted component isn't well-defined without additional geometric evidence; a
/// clean re-match/retriangulation later naturally recovers the good sub-tracks.
pub fn build_tracks(num_frames: usize, pairwise_matches: &[(usize, usize, Vec<Match>)]) -> FeatureTracks {
    let _ = num_frames;
    let mut node_of: std::collections::HashMap<(usize, u32), usize> = std::collections::HashMap::new();
    let mut obs_of: Vec<(usize, u32)> = Vec::new();
    for (frame_a, frame_b, matches) in pairwise_matches {
        for m in matches {
            node_of.entry((*frame_a, m.a)).or_insert_with(|| {
                obs_of.push((*frame_a, m.a));
                obs_of.len() - 1
            });
            node_of.entry((*frame_b, m.b)).or_insert_with(|| {
                obs_of.push((*frame_b, m.b));
                obs_of.len() - 1
            });
        }
    }
    let mut uf = UnionFind::new(obs_of.len());
    for (frame_a, frame_b, matches) in pairwise_matches {
        for m in matches {
            let na = node_of[&(*frame_a, m.a)];
            let nb = node_of[&(*frame_b, m.b)];
            uf.union(na, nb);
        }
    }
    let mut groups: std::collections::HashMap<usize, Vec<(usize, u32)>> = std::collections::HashMap::new();
    for (idx, &obs) in obs_of.iter().enumerate() {
        let root = uf.find(idx);
        groups.entry(root).or_default().push(obs);
    }
    let mut tracks: Vec<Vec<(usize, u32)>> = Vec::new();
    for (_, mut obs) in groups {
        obs.sort_unstable();
        let has_conflict = obs.windows(2).any(|w| w[0].0 == w[1].0);
        if !has_conflict {
            tracks.push(obs);
        }
    }
    tracks.sort();
    FeatureTracks { tracks }
}
// #endregion 🔖️Tracks

// #region 🔖️Global
/// 🕸️ Adjacency list from relative-rotation edges: `(i,j,Rij)` with `Rj = Rij ∘ Ri` contributes `i -> (j,
/// Rij)` and the inverse edge `j -> (i, Rij⁻¹)`.
fn rotation_adjacency(n: usize, edges: &[(usize, usize, So3)]) -> Vec<Vec<(usize, So3)>> {
    let mut adj = vec![Vec::new(); n];
    for &(i, j, rij) in edges {
        if i < n && j < n {
            adj[i].push((j, rij));
            adj[j].push((i, rij.inverse()));
        }
    }
    adj
}

/// 🕸️ Simplified global rotation averaging: a BFS spanning-tree composition from anchor node 0 gives an
/// initial guess (exact whenever the relative rotations are noiseless and the graph is connected), then a
/// fixed number of IRLS sweeps refine every non-anchor node by averaging, in its own `so(3)` tangent
/// space, the rotation implied by each neighboring edge (`Ri ≈ Rij⁻¹ Rj`). This is a documented
/// simplification of full chordal-L2 rotation averaging (which solves a single `3n x 3n` linear
/// eigenproblem): per-node tangent averaging around the current estimate converges well for the
/// small-to-moderate noise and graph sizes this crate targets, but is not the eigendecomposition-exact
/// global optimum. Node 0 is always returned as [`So3::identity`] (the gauge anchor); nodes unreachable
/// from it also default to identity.
pub fn rotation_averaging(relative_rotations: &[(usize, usize, So3)]) -> Vec<So3> {
    let n = relative_rotations.iter().map(|&(i, j, _)| i.max(j)).max().map_or(0, |m| m + 1);
    if n == 0 {
        return Vec::new();
    }
    let adj = rotation_adjacency(n, relative_rotations);
    let mut rotations = vec![So3::identity(); n];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0usize);
    while let Some(i) = queue.pop_front() {
        for &(j, rij) in &adj[i] {
            if !visited[j] {
                visited[j] = true;
                rotations[j] = rij.semio_compose_rs(&rotations[i]);
                queue.push_back(j);
            }
        }
    }
    for _ in 0..10 {
        for i in 1..n {
            if adj[i].is_empty() {
                continue;
            }
            let mut sum_tangent = [0.0; 3];
            for &(j, rij) in &adj[i] {
                let implied = rij.inverse().semio_compose_rs(&rotations[j]);
                let delta = implied.semio_compose_rs(&rotations[i].inverse()).log();
                sum_tangent = add3(sum_tangent, delta);
            }
            let avg = scale3(sum_tangent, 1.0 / adj[i].len() as f64);
            rotations[i] = So3::exp(avg).semio_compose_rs(&rotations[i]);
        }
    }
    rotations
}

/// 📐️ Simplified global translation averaging from baseline-direction constraints: for edge `(i, j,
/// local_dir)`, `local_dir` is the unit direction from camera `i`'s center to camera `j`'s center
/// *expressed in camera `i`'s local frame* (the same convention as the `t` component of the relative pose
/// [`decompose_essential`] returns). Each edge contributes the linear cross-product constraint `(Cj - Ci)
/// x world_dir_ij = 0` (rotated into the world frame via `rotations[i]`), stacked into one large
/// homogeneous system solved via [`svd_nullvector`]/gram-nullspace for the camera centers `C1..C(n-1)`
/// (`C0` fixed at the origin). Direction-only constraints carry no absolute distance information, so the
/// returned centers are correct only up to a single unknown global scale — the scale of the unit-norm
/// nullspace solution itself, not any physical unit. Callers needing metric scale should rescale via
/// [`align_to_priors`] or another absolute reference.
pub fn translation_averaging(relative_directions: &[(usize, usize, [f64; 3])], rotations: &[So3]) -> Vec<[f64; 3]> {
    let n = rotations.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 || relative_directions.is_empty() {
        return vec![[0.0; 3]; n];
    }
    let col_of = |node: usize| -> Option<usize> {
        if node == 0 {
            None
        } else {
            Some(3 * (node - 1))
        }
    };
    let mut design = MatD::zeros(3 * relative_directions.len(), 3 * (n - 1));
    for (row, &(i, j, local_dir)) in relative_directions.iter().enumerate() {
        if i >= n || j >= n {
            continue;
        }
        let world_dir = vec3d_normalize(rotations[i].inverse().act(local_dir));
        let (dx, dy, dz) = (world_dir[0], world_dir[1], world_dir[2]);
        let skew_rows = [[0.0, -dz, dy], [dz, 0.0, -dx], [-dy, dx, 0.0]];
        for (r, skew_row) in skew_rows.iter().enumerate() {
            if let Some(cj) = col_of(j) {
                for (c, &value) in skew_row.iter().enumerate() {
                    design.add_at(3 * row + r, cj + c, value);
                }
            }
            if let Some(ci) = col_of(i) {
                for (c, &value) in skew_row.iter().enumerate() {
                    design.add_at(3 * row + r, ci + c, -value);
                }
            }
        }
    }
    let mut centers = vec![[0.0; 3]; n];
    if let Some(sol) = nullvector_via_gram(&design) {
        for (node, center) in centers.iter_mut().enumerate().skip(1) {
            let base = 3 * (node - 1);
            *center = [sol.get(base), sol.get(base + 1), sol.get(base + 2)];
        }
    }
    centers
}
// #endregion 🔖️Global

// #region 🔖️LoopClosure
const LSH_TABLES: usize = 4;

/// 🔎️ LSH keyframe index over [`Descriptor256`]s: 4 hash tables, each keyed by one 256-bit descriptor's
/// low 16 bits of a distinct `u64` word (words 0..3, i.e. bits `[0,16)`, `[64,80)`, `[128,144)`,
/// `[192,208)` — one substring per word for spread across the descriptor), with no learned vocabulary.
#[derive(Clone, Debug, Default)]
pub struct KeyframeIndex {
    tables: [std::collections::HashMap<u16, Vec<usize>>; LSH_TABLES],
}

impl KeyframeIndex {
    /// 🆕️ Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Buckets `frame`'s descriptors into all 4 LSH tables (duplicate insertions of the same frame into
    /// the same bucket are harmless — `candidates` just counts them as extra votes).
    pub fn insert(&mut self, frame: usize, descriptors: &[Descriptor256]) {
        for desc in descriptors {
            for (t, table) in self.tables.iter_mut().enumerate() {
                table.entry((desc.0[t] & 0xFFFF) as u16).or_default().push(frame);
            }
        }
    }

    /// 🔍️ Frames voted for by at least `min_shared_buckets` total `(query descriptor, table)` bucket hits
    /// — i.e. the sum, over every query descriptor and every one of the 4 tables, of how many times that
    /// frame appears in the matching bucket. Ascending by frame index.
    pub fn candidates(&self, descriptors: &[Descriptor256], min_shared_buckets: usize) -> Vec<usize> {
        let mut votes: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for desc in descriptors {
            for (t, table) in self.tables.iter().enumerate() {
                if let Some(frames) = table.get(&((desc.0[t] & 0xFFFF) as u16)) {
                    for &frame in frames {
                        *votes.entry(frame).or_insert(0) += 1;
                    }
                }
            }
        }
        let mut out: Vec<usize> = votes.into_iter().filter(|&(_, count)| count >= min_shared_buckets).map(|(frame, _)| frame).collect();
        out.sort_unstable();
        out
    }
}

/// 📦️ A geometrically-verified loop-closure candidate: the revisited frame, its inlier count, and the recovered two-view model.
#[derive(Clone, Debug, PartialEq)]
pub struct LoopCandidate {
    pub frame: usize,
    pub inlier_count: usize,
    pub model: TwoViewModel,
}

/// 🔁️ Loop-closure detection: gathers LSH candidate frames from `index`, matches `current_descriptors`
/// against each candidate's descriptors ([`remodeling_feature::match_brute`], ratio `0.8`, mutual
/// cross-check), geometrically verifies via [`select_two_view_model`], and keeps candidates clearing a
/// minimum inlier count — descending by inlier count.
pub fn detect_loops(index: &KeyframeIndex, current_frame: usize, current_descriptors: &[Descriptor256], current_keypoints: &[Keypoint], all_keypoints: &[Vec<Keypoint>], all_descriptors: &[Vec<Descriptor256>]) -> Vec<LoopCandidate> {
    const MIN_SHARED_BUCKETS: usize = 3;
    const MIN_LOOP_INLIERS: usize = 12;
    let mut out = Vec::new();
    for frame in index.candidates(current_descriptors, MIN_SHARED_BUCKETS) {
        if frame == current_frame || frame >= all_keypoints.len() || frame >= all_descriptors.len() {
            continue;
        }
        let matches = match_brute(current_descriptors, &all_descriptors[frame], 0.8, true);
        if matches.len() < 8 {
            continue;
        }
        let corr: Vec<([f64; 2], [f64; 2])> = matches
            .iter()
            .filter(|m| (m.a as usize) < current_keypoints.len() && (m.b as usize) < all_keypoints[frame].len())
            .map(|m| {
                let a = current_keypoints[m.a as usize];
                let b = all_keypoints[frame][m.b as usize];
                ([f64::from(a.x), f64::from(a.y)], [f64::from(b.x), f64::from(b.y)])
            })
            .collect();
        if let Some(result) = select_two_view_model(&corr) {
            if result.inliers.len() >= MIN_LOOP_INLIERS {
                out.push(LoopCandidate { frame, inlier_count: result.inliers.len(), model: result.model });
            }
        }
    }
    out.sort_by_key(|c| std::cmp::Reverse(c.inlier_count));
    out
}

/// 🧩️ Sim3 pose-graph consistency problem, simplified to `se(3)` (rotation + translation only, dropping
/// each edge's scale dimension): parameters are one 6-dof `se(3)` log-tangent per non-anchor node (node 0
/// is fixed to `poses[0]`), and each edge residual is `log(Zij⁻¹ (Pj Pi⁻¹))` where `Zij` is the edge's
/// `Se3` part (`Sim3`'s `r`/`t`, ignoring `s`). A full Sim3-space optimization would carry the extra scale
/// tangent per node/edge; dropping it is the documented simplification here, appropriate when the input
/// poses/edges are already metrically consistent (e.g. from a calibrated monocular pipeline sharing one
/// global scale) and only rotation/translation drift needs correcting.
struct PoseGraphProblem<'a> {
    fixed_pose0: Se3,
    edges: &'a [(usize, usize, Sim3)],
    num_nodes: usize,
}

impl PoseGraphProblem<'_> {
    fn pose_at(&self, x: &VecD, node: usize) -> Se3 {
        if node == 0 {
            self.fixed_pose0
        } else {
            let base = 6 * (node - 1);
            Se3::exp(std::array::from_fn(|k| x.get(base + k)))
        }
    }
}

impl LeastSquaresProblem for PoseGraphProblem<'_> {
    fn residual_count(&self) -> usize {
        self.edges.len() * 6
    }

    fn parameter_count(&self) -> usize {
        6 * self.num_nodes.saturating_sub(1)
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        for (row, &(i, j, zij)) in self.edges.iter().enumerate() {
            let pi = self.pose_at(x, i);
            let pj = self.pose_at(x, j);
            let predicted = pj.semio_compose_rs(&pi.inverse());
            let z_se3 = Se3 { r: zij.r, t: zij.t };
            let err = z_se3.inverse().semio_compose_rs(&predicted).log();
            for (k, value) in err.into_iter().enumerate() {
                out.set(row * 6 + k, value);
            }
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for node in 1..self.num_nodes {
            let base = 6 * (node - 1);
            let cur: [f64; 6] = std::array::from_fn(|k| x.get(base + k));
            let ddelta: [f64; 6] = std::array::from_fn(|k| dx.get(base + k));
            let updated = Se3::exp(ddelta).semio_compose_rs(&Se3::exp(cur)).log();
            for (k, value) in updated.into_iter().enumerate() {
                out.set(base + k, value);
            }
        }
        out
    }
}

/// 🔁️ Sim3-edge pose-graph optimization (see [`PoseGraphProblem`] for the documented `se(3)`
/// simplification): refines `poses` via Levenberg-Marquardt to best satisfy `edges`, anchoring `poses[0]`.
pub fn pose_graph_optimize(poses: &[Se3], edges: &[(usize, usize, Sim3)]) -> Vec<Se3> {
    let n = poses.len();
    if n == 0 {
        return Vec::new();
    }
    let problem = PoseGraphProblem { fixed_pose0: poses[0], edges, num_nodes: n };
    let mut x0 = VecD::zeros(problem.parameter_count());
    for (node, pose) in poses.iter().enumerate().skip(1) {
        let base = 6 * (node - 1);
        for (k, v) in pose.log().into_iter().enumerate() {
            x0.set(base + k, v);
        }
    }
    let cfg = LmConfig { max_iters: 100, ..LmConfig::default() };
    let result = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt(&problem, x0, &cfg);
    (0..n).map(|node| problem.pose_at(&result.x, node)).collect()
}
// #endregion 🔖️LoopClosure
