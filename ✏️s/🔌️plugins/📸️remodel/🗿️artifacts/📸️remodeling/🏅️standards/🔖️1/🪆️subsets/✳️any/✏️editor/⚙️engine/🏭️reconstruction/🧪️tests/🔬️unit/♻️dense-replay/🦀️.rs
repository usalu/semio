//! ♻️ Dense replay diagnostic of the staged reconstruction engine: the dense dump codec and the replay of a dumped dense stage through fusion and meshing. See [`super`].

use super::*;
use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm as remodeling_sfm;
use semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh as remodeling_mesh;

/// 💾️ Binary snapshot of a finished dense stage (reconstruction, dense camera indices, depth maps)
/// so fusion/TSDF/meshing experiments replay in seconds instead of re-running SfM and PatchMatch.
pub(super) mod dense_dump {
    use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
    use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
    use super::*;

    struct Writer(Vec<u8>);
    impl Writer {
        fn u64(&mut self, value: usize) {
            self.0.extend_from_slice(&(value as u64).to_le_bytes());
        }
        fn f64(&mut self, value: f64) {
            self.0.extend_from_slice(&value.to_le_bytes());
        }
        fn f32(&mut self, value: f32) {
            self.0.extend_from_slice(&value.to_le_bytes());
        }
    }
    struct Reader<'a>(&'a [u8], usize);
    impl Reader<'_> {
        fn take<const N: usize>(&mut self) -> [u8; N] {
            let bytes: [u8; N] = self.0[self.1..self.1 + N].try_into().expect("bytes");
            self.1 += N;
            bytes
        }
        fn u64(&mut self) -> usize {
            u64::from_le_bytes(self.take()) as usize
        }
        fn f64(&mut self) -> f64 {
            f64::from_le_bytes(self.take())
        }
        fn f32(&mut self) -> f32 {
            f32::from_le_bytes(self.take())
        }
    }

    pub(in super::super) fn encode(reconstruction: &remodeling_sfm::Reconstruction, dense: &[usize], maps: &[remodeling_dense::DepthMap]) -> Vec<u8> {
        let mut w = Writer(Vec::new());
        let i = reconstruction.intrinsics;
        for value in [i.fx, i.fy, i.cx, i.cy, i.skew] {
            w.f64(value);
        }
        match i.distortion {
            remodeling_camera::Distortion::BrownConrady { k1, k2, k3, p1, p2 } => {
                w.u64(1);
                for value in [k1, k2, k3, p1, p2] {
                    w.f64(value);
                }
            }
            _ => w.u64(0),
        }
        w.u64(reconstruction.cameras.len());
        for (frame, pose) in &reconstruction.cameras {
            w.u64(*frame);
            for column in pose.0.r.0.cols {
                for value in column {
                    w.f64(value);
                }
            }
            for value in pose.0.t {
                w.f64(value);
            }
        }
        w.u64(reconstruction.points.len());
        for point in &reconstruction.points {
            for value in point {
                w.f64(*value);
            }
        }
        w.u64(dense.len());
        for index in dense {
            w.u64(*index);
        }
        w.u64(maps.len());
        for map in maps {
            w.u64(map.width as usize);
            w.u64(map.height as usize);
            for index in 0..map.depth.len() {
                w.f32(map.depth[index]);
                for value in map.normal[index] {
                    w.f32(value);
                }
                w.f32(map.confidence[index]);
            }
        }
        w.0
    }

    pub(super) fn decode(bytes: &[u8]) -> (remodeling_sfm::Reconstruction, Vec<usize>, Vec<remodeling_dense::DepthMap>) {
        let mut r = Reader(bytes, 0);
        let (fx, fy, cx, cy, skew) = (r.f64(), r.f64(), r.f64(), r.f64(), r.f64());
        let distortion = if r.u64() == 1 {
            let (k1, k2, k3, p1, p2) = (r.f64(), r.f64(), r.f64(), r.f64(), r.f64());
            remodeling_camera::Distortion::BrownConrady { k1, k2, k3, p1, p2 }
        } else {
            remodeling_camera::Distortion::None
        };
        let intrinsics = remodeling_camera::Intrinsics { fx, fy, cx, cy, skew, distortion };
        let cameras = (0..r.u64())
            .map(|_| {
                let frame = r.u64();
                let cols: [[f64; 3]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| r.f64()));
                let t = [r.f64(), r.f64(), r.f64()];
                (frame, remodeling_camera::CameraPose(semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3 { r: semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3(semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d { cols }), t }))
            })
            .collect();
        let points = (0..r.u64()).map(|_| [r.f64(), r.f64(), r.f64()]).collect::<Vec<_>>();
        let point_track_ids = (0..points.len()).collect();
        let dense = (0..r.u64()).map(|_| r.u64()).collect();
        let maps = (0..r.u64())
            .map(|_| {
                let (width, height) = (r.u64() as u32, r.u64() as u32);
                let mut map = remodeling_dense::DepthMap::new(width, height);
                for index in 0..map.depth.len() {
                    map.depth[index] = r.f32();
                    map.normal[index] = [r.f32(), r.f32(), r.f32()];
                    map.confidence[index] = r.f32();
                }
                map
            })
            .collect();
        (remodeling_sfm::Reconstruction { cameras, points, point_track_ids, intrinsics }, dense, maps)
    }
}

/// 🔁️ Replays fusion → TSDF → surface pipeline over a [`dense_dump`] written by
/// [`diagnose_synthetic_orbit_dense_against_truth`] (`DENSE_DUMP=<path>`), scoring each mesh
/// against the fixture's truth cube. `DENSE_FUSION="tolerance,views"` and
/// `DENSE_GRID="voxel,truncation;…"` pick the experiments. Diagnostic, therefore `#[ignore]`.
#[test]
#[ignore = "diagnostic replay of a dumped dense stage, run explicitly"]
fn diagnose_dense_replay() {
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Quatd;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;
    let path = std::env::var("DENSE_DUMP").expect("DENSE_DUMP names the dump");
    let (reconstruction, dense_indices, depth_maps) = dense_dump::decode(&std::fs::read(path).expect("read dump"));
    let truth: serde_json::Value = serde_json::from_str(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let half = number(&truth["scene"]["halfExtentM"]);
    let truth_centres: Vec<[f64; 3]> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            Se3 { r: rotation, t: scale3(rotation.act([number(&c[0]), number(&c[1]), number(&c[2])]), -1.0) }.inverse().t
        })
        .collect();
    let truth_points: Vec<[f64; 3]> = truth["pointsWorldM"].as_array().expect("points").iter().map(|p| { let p = p.as_array().expect("point"); [number(&p[0]), number(&p[1]), number(&p[2])] }).collect();
    let surface = |p: [f64; 3]| {
        let outside = [(p[0].abs() - half).max(0.0), (p[1].abs() - half).max(0.0), (p[2].abs() - half).max(0.0)];
        let outside = (outside[0] * outside[0] + outside[1] * outside[1] + outside[2] * outside[2]).sqrt();
        let inside = (half - p[0].abs()).min(half - p[1].abs()).min(half - p[2].abs()).max(0.0);
        (outside + inside) / half
    };
    let quantiles = |mut values: Vec<f64>| -> String {
        if values.is_empty() {
            return "n=0".into();
        }
        values.sort_by(f64::total_cmp);
        let q = |f: f64| values[((values.len() as f64 - 1.0) * f).round() as usize];
        format!("n={} median {:.3} p90 {:.3}", values.len(), q(0.5), q(0.9))
    };
    let recovered: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(_, pose)| pose.0.inverse().t).collect();
    let targets: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(frame, _)| truth_centres[*frame]).collect();
    let sim = semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::umeyama(&recovered, &targets, true).expect("Sim(3) over camera centres");
    let scene = <semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    let mut extrema_min = [f64::INFINITY; 3];
    let mut extrema_max = [f64::NEG_INFINITY; 3];
    for point in &reconstruction.points {
        for axis in 0..3 {
            extrema_min[axis] = extrema_min[axis].min(point[axis]);
            extrema_max[axis] = extrema_max[axis].max(point[axis]);
        }
    }
    let views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)> = dense_indices.iter().map(|camera| (reconstruction.cameras[*camera].1, reconstruction.intrinsics)).collect();
    let fusion_config = std::env::var("DENSE_FUSION").ok().map_or_else(remodeling_dense::FusionConfig::default, |text| {
        let (tolerance, views) = text.split_once(',').expect("tolerance,views");
        remodeling_dense::FusionConfig { max_relative_depth_diff: tolerance.parse().expect("tolerance"), max_normal_angle_deg: 30.0, min_consistent_views: views.parse().expect("views") }
    });
    let mut fusion = remodeling_dense::FusionPreparation::new(0);
    while !fusion.advance(&views, &depth_maps, &fusion_config, usize::MAX / 2) {}
    let (_, masks) = fusion.finish_with_consistency().expect("fusion completes");
    let masked: Vec<remodeling_dense::DepthMap> = depth_maps
        .iter()
        .zip(&masks)
        .map(|(map, bits)| {
            let mut map = map.clone();
            for (index, depth) in map.depth.iter_mut().enumerate() {
                if bits[index / 64] >> (index % 64) & 1 == 0 {
                    *depth = 0.0;
                }
            }
            map
        })
        .collect();
    let kept: usize = masked.iter().map(|map| map.depth.iter().filter(|depth| **depth > 0.0).count()).sum();
    eprintln!("[REPLAY] fusion {fusion_config:?}: {kept} consistent pixels");
    let grid: Vec<(f64, f64)> = std::env::var("DENSE_GRID").ok().map_or_else(
        || vec![(0.1, 0.33), (0.1, 0.2), (0.15, 0.3), (0.2, 0.2), (0.25, 0.25)],
        |text| text.split(';').map(|pair| { let (v, t) = pair.split_once(',').expect("voxel,truncation"); (v.parse().expect("voxel"), t.parse().expect("truncation")) }).collect(),
    );
    for (voxel_size, truncation) in grid {
        let (grid_min, grid_max) = compute_voxel_bounds_from_extrema(extrema_min, extrema_max, voxel_size);
        let mut volume = remodeling_dense::TsdfVolume::new(voxel_size, truncation).with_voxel_bounds(grid_min.map(|c| c - 2), grid_max.map(|c| c + 2));
        if std::env::var_os("DENSE_BOUNDED").is_some() {
            for ((map, view), mask) in depth_maps.iter().zip(&views).zip(&masks) {
                let mut preparation = remodeling_dense::TsdfIntegrationPreparation::new();
                while !preparation.advance_masked(&mut volume, map, Some(mask), view, true, 256) {}
            }
        } else {
            for (map, view) in masked.iter().zip(&views) {
                volume.integrate(map, view, true);
            }
        }
        eprintln!("[REPLAY] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {grid_min:?}..{grid_max:?} volume blocks {}", volume.block_count());
        let mut pipeline = remodeling_mesh::MeshPipeline::new_bounded(volume, 0.0, grid_min, grid_max, params.mesh.clone());
        if std::env::var_os("DENSE_OBSERVED").is_some() {
            pipeline = pipeline.with_observed_only_surface();
        }
        let mut extracted = None;
        let mut trail: Vec<String> = Vec::new();
        let mut last_triangles = usize::MAX;
        let outcome = loop {
            let status = remodeling_mesh::mesh_pipeline_step(&mut pipeline, 1);
            if pipeline.mesh().triangles.len() != last_triangles {
                last_triangles = pipeline.mesh().triangles.len();
                let report = semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::validate_watertight(pipeline.mesh(), false);
                let worst = pipeline.mesh().positions.iter().map(|p| surface(sim.act(*p))).fold(0.0f64, f64::max);
                trail.push(format!("{}t/b{}/nm{}/nv{}/c{}/worst{:.2}", report.triangle_count, report.boundary_edge_count, report.non_manifold_edge_count, report.non_manifold_vertex_count, report.connected_components, worst));
            }
            if extracted.is_none() && !pipeline.mesh().triangles.is_empty() {
                let mesh = pipeline.mesh();
                let mut parent: Vec<usize> = (0..mesh.positions.len()).collect();
                fn root(parent: &mut [usize], mut index: usize) -> usize {
                    while parent[index] != index {
                        parent[index] = parent[parent[index]];
                        index = parent[index];
                    }
                    index
                }
                for triangle in &mesh.triangles {
                    let a = root(&mut parent, triangle[0] as usize);
                    for corner in &triangle[1..] {
                        let b = root(&mut parent, *corner as usize);
                        parent[b] = a;
                    }
                }
                let mut sizes = std::collections::BTreeMap::new();
                for triangle in &mesh.triangles {
                    *sizes.entry(root(&mut parent, triangle[0] as usize)).or_insert(0usize) += 1;
                }
                let mut sizes: Vec<usize> = sizes.into_values().collect();
                sizes.sort_unstable_by(|a, b| b.cmp(a));
                extracted = Some((mesh.positions.len(), mesh.triangles.len(), sizes));
            }
            match status {
                remodeling_mesh::MeshPipelineStatus::Working { .. } => {}
                remodeling_mesh::MeshPipelineStatus::Done => break "done".to_string(),
                remodeling_mesh::MeshPipelineStatus::Failed(message) => break message.chars().take(400).collect(),
            }
        };
        let positions: Vec<[f64; 3]> = pipeline.result().map_or_else(|| pipeline.mesh().positions.iter().map(|p| sim.act(*p)).collect(), |mesh| mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect());
        let covered = truth_points.iter().filter(|point| positions.iter().any(|vertex| norm3(sub3(*vertex, **point)) <= 0.4)).count();
        eprintln!("[REPLAY] voxel {voxel_size} trunc {truncation}: trail {trail:?}; extracted {extracted:?} -> {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map(|p| surface(*p)).collect()), 100.0 * covered as f64 / truth_points.len() as f64);
    }
}
