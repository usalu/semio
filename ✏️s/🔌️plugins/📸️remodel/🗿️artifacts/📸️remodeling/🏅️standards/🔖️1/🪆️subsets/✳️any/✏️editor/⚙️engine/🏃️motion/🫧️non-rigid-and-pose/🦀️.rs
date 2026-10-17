//! 🫧️ Non-rigid capture and rigid-body pose tracking of the videogrammetry. See [`super`].

use super::*;

// #region 🔖️NonRigid
/// 🕸️ One embedded-deformation graph node: rest position plus its currently-fitted rigid transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeformationNode {
    pub position: [f64; 3],
    pub transform: Se3,
}

/// 🕷️ A sparse embedded-deformation graph (Sumner et al.): control nodes plus undirected neighbour edges.
#[derive(Clone, Debug, PartialEq)]
pub struct DeformationGraph {
    pub nodes: Vec<DeformationNode>,
    pub edges: Vec<(usize, usize)>,
}

/// 🏗️ Builds a deformation graph with one identity-transform node per `seed_points` entry, connected by an
/// undirected edge whenever two nodes are within `edge_radius` of each other.
pub fn build_deformation_graph(seed_points: &[[f64; 3]], edge_radius: f64) -> DeformationGraph {
    let nodes: Vec<DeformationNode> = seed_points.iter().map(|&p| DeformationNode { position: p, transform: Se3::identity() }).collect();
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if dist3(nodes[i].position, nodes[j].position) <= edge_radius {
                edges.push((i, j));
            }
        }
    }
    DeformationGraph { nodes, edges }
}

struct DeformationNodeFitProblem<'a> {
    offsets: &'a [[f64; 3]],
    targets: &'a [[f64; 3]],
}

impl LeastSquaresProblem for DeformationNodeFitProblem<'_> {
    fn residual_count(&self) -> usize {
        self.offsets.len() * 3
    }

    fn parameter_count(&self) -> usize {
        6
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let xi: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let se3 = Se3::exp(xi);
        for (row, (off, tgt)) in self.offsets.iter().zip(self.targets.iter()).enumerate() {
            let pred = se3.act(*off);
            out.set(row * 3, pred[0] - tgt[0]);
            out.set(row * 3 + 1, pred[1] - tgt[1]);
            out.set(row * 3 + 2, pred[2] - tgt[2]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let cur: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let d: [f64; 6] = std::array::from_fn(|k| dx.get(k));
        VecD::from_vec(Se3::exp(d).semio_compose_rs(&Se3::exp(cur)).log().to_vec())
    }
}

const NONRIGID_MIN_CORRESPONDENCES: usize = 3;

/// 🧩️ Fits each node's local rigid [`Se3`] transform independently via Levenberg-Marquardt over its nearby
/// correspondences (any `dst_correspondences` entry whose `src_points` position lies within
/// `correspondence_radius` of that node): a documented simplification of full embedded deformation, which
/// jointly solves all nodes with an ARAP-style smoothness coupling between neighbours — here each node's
/// fit uses only its own local data, ignoring the `edges` regularization term, trading joint-consistency for
/// simplicity while remaining a real, working, per-node rigid fit. Nodes with fewer than
/// [`NONRIGID_MIN_CORRESPONDENCES`] nearby correspondences keep their prior transform unchanged.
pub fn fit_deformation(graph: &DeformationGraph, src_points: &[[f64; 3]], dst_correspondences: &[(usize, [f64; 3])], correspondence_radius: f64) -> DeformationGraph {
    let mut nodes = graph.nodes.clone();
    for node in &mut nodes {
        let mut offsets = Vec::new();
        let mut targets = Vec::new();
        for &(src_idx, dst_pos) in dst_correspondences {
            let Some(&src_pt) = src_points.get(src_idx) else { continue };
            if dist3(src_pt, node.position) <= correspondence_radius {
                offsets.push(sub3(src_pt, node.position));
                targets.push(sub3(dst_pos, node.position));
            }
        }
        if offsets.len() < NONRIGID_MIN_CORRESPONDENCES {
            continue;
        }
        let problem = DeformationNodeFitProblem { offsets: &offsets, targets: &targets };
        let result = levenberg_marquardt(&problem, VecD::zeros(6), &LmConfig::default());
        let xi: [f64; 6] = std::array::from_fn(|k| result.x.get(k));
        node.transform = Se3::exp(xi);
    }
    DeformationGraph { nodes, edges: graph.edges.clone() }
}

const NONRIGID_WEIGHT_EPS: f64 = 1e-6;

/// 🔮️ Embedded-deformation query: blends every graph node within `radius` of `point` by inverse distance,
/// each contributing `node.transform.act(point - node.position) + node.position`. Falls back to `point`
/// unchanged when no node is within range. The natural complement to [`fit_deformation`] — without this,
/// a fitted [`DeformationGraph`]'s per-node transforms have no way to be evaluated at an arbitrary point.
pub fn deform_point(graph: &DeformationGraph, point: [f64; 3], radius: f64) -> [f64; 3] {
    let mut weighted = [0.0; 3];
    let mut weight_sum = 0.0;
    for node in &graph.nodes {
        let d = dist3(point, node.position);
        if d > radius {
            continue;
        }
        let w = 1.0 / (d + NONRIGID_WEIGHT_EPS);
        let pred = add3(node.transform.act(sub3(point, node.position)), node.position);
        weighted = add3(weighted, scale3(pred, w));
        weight_sum += w;
    }
    if weight_sum <= 0.0 {
        return point;
    }
    scale3(weighted, 1.0 / weight_sum)
}
// #endregion 🔖️NonRigid

// #region 🔖️Pose6d
const POSE6D_SMOOTH_WINDOW: usize = 5;

const POSE6D_MIN_VISIBLE_POINTS: usize = 6;

/// 📦️ Per-frame rigid-body 6DoF pose from partially-visible model-point observations: for each frame, gathers
/// the model points with a visible observation, solves an initial pose via `crate::editor::remodeling::engine::sfm::geometry::epnp` (skipping
/// frames with fewer than [`POSE6D_MIN_VISIBLE_POINTS`] visible points, `epnp`'s own minimum), polishes it
/// with `crate::editor::remodeling::engine::sfm::geometry::refine_pose_lm`, then temporally smooths the recovered sequence via
/// [`smooth_camera_path`]'s SE(3) log-tangent Savitzky-Golay filter.
pub fn track_rigid_body(model_points: &[[f64; 3]], per_frame_obs: &[(u32, Vec<Option<[f64; 2]>>)], intr: &Intrinsics) -> Vec<(u32, Se3)> {
    let mut raw: Vec<(u32, Se3)> = Vec::new();
    for (frame, obs) in per_frame_obs {
        let mut world_pts = Vec::new();
        let mut obs_px = Vec::new();
        for (i, o) in obs.iter().enumerate() {
            if let Some(px) = o {
                if let Some(&mp) = model_points.get(i) {
                    world_pts.push(mp);
                    obs_px.push(*px);
                }
            }
        }
        if world_pts.len() < POSE6D_MIN_VISIBLE_POINTS {
            continue;
        }
        let Some(initial) = semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::epnp(intr, &world_pts, &obs_px) else { continue };
        let refined = semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::refine_pose_lm(intr, &world_pts, &obs_px, initial);
        raw.push((*frame, refined));
    }
    if raw.len() < 3 {
        return raw;
    }
    let poses: Vec<Se3> = raw.iter().map(|&(_, p)| p).collect();
    let smoothed = smooth_camera_path(&poses, POSE6D_SMOOTH_WINDOW);
    raw.iter().zip(smoothed.iter()).map(|(&(frame, _), &p)| (frame, p)).collect()
}
// #endregion 🔖️Pose6d
