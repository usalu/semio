//! 🔎️ Orbit diagnostics of the staged reconstruction engine: the ignored stage trace, geometry audit and dense audit against the ground truth of the synthetic orbit. See [`super`].

use super::*;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_feature::editor::remodeling::engine::feature as remodeling_feature;
use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm as remodeling_sfm;
use semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh as remodeling_mesh;

/// 🔭️ Stage-by-stage trace of the shipped `synthetic-orbit` example through the bare engine (pair
/// table, registered cameras, fused-cloud extent, terminal mesh census). Diagnostic, therefore
/// `#[ignore]`: `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture
/// debug_probe_synthetic_orbit`; the asserting run of this fixture is the app-level
/// `reconstructs_the_synthetic_orbit_against_ground_truth`.
#[test]
#[ignore = "diagnostic stage trace: minutes of dense stereo, run explicitly"]
fn debug_probe_synthetic_orbit() {
    let scene = <semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    let mut engine = ReconstructionEngine::new(&params);
    for (index, (_, bytes)) in semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames().enumerate() {
        let image = semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image("image/png", bytes).expect("decodes");
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let mut last = engine.stage();
    let mut units = 0usize;
    loop {
        let status = engine.advance(1);
        units += 1;
        if engine.stage() != last {
            match last {
                EngineStage::ExtractingFeatures => {
                    let variant = std::env::var("PROBE_FEATURES").unwrap_or_default();
                    if !variant.is_empty() {
                        for frame in 0..engine.frames.len() {
                            let image = &engine.frames[frame].image;
                            let mut gray = remodeling_image::ImageGray { width: image.width, height: image.height, data: Vec::new() };
                            let mut cursor = 0;
                            append_luma_slice(image, &mut gray, &mut cursor, usize::MAX / 2);
                            let target = engine.params.target_feature_count;
                            let (k, d) = match variant.as_str() {
                                "orb" => { let p = remodeling_image::build_pyramid(&gray, 3); let k = remodeling_feature::detect_orb_keypoints(&p, target); let d = remodeling_feature::describe_orb(&p, &k); (k, d) }
                                "harris" => { let p = remodeling_image::build_pyramid(&gray, 1); let k = remodeling_feature::detect_harris_keypoints(&gray, target); let d = remodeling_feature::describe_orb(&p, &k); (k, d) }
                                _ => { let s = remodeling_feature::build_akaze_scale_space(&gray, 3, 3); let k = remodeling_feature::detect_akaze_keypoints(&s, target); let d = remodeling_feature::describe_akaze(&s, &k); (k, d) }
                            };
                            engine.keypoints_per_frame[frame] = k;
                            engine.descriptors_per_frame[frame] = d;
                        }
                    }
                }
                EngineStage::MatchingFeatures => {
                    eprintln!("tracks {}", engine.tracks.as_ref().map_or(0, |t| t.tracks.len()));
                    eprintln!("pairs {:?}", engine.pairwise_matches.iter().map(|(a, b, m)| (*a, *b, m.len())).collect::<Vec<_>>());
                }
                EngineStage::BundleAdjusting => {
                    let r = engine.reconstruction.as_ref().unwrap();
                    eprintln!("cameras {:?} points {}", r.cameras.iter().map(|c| c.0).collect::<Vec<_>>(), r.points.len());
                }
                EngineStage::FusingVolume => {
                    let cloud = engine.dense_positions();
                    let mut lo = [f64::INFINITY; 3]; let mut hi = [f64::NEG_INFINITY; 3];
                    for p in cloud { for a in 0..3 { lo[a] = lo[a].min(p[a]); hi[a] = hi[a].max(p[a]); } }
                }
                _ => {}
            }
            last = engine.stage();
        }
        match status {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => { eprintln!("[TRACE] done mesh {:?}", engine.mesh_data.as_ref().map(|m| (m.positions.len()/3, m.indices.len()/3))); break; }
            EngineStatus::Failed(msg) => {
                eprintln!("failed {msg} mesh so far {}/{}", engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().positions.len()), engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().triangles.len()));
                break;
            }
        }
    }
}

/// 🔭️ Match and seed-pose quality of the shipped `synthetic-orbit` example against its ground
/// truth: for every matched pair, how many matches satisfy the TRUE epipolar geometry, and for the
/// registered cameras, the rotation and translation-direction error of the recovered relative
/// poses. Diagnostic, therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture diagnose_synthetic_orbit_geometry`.
#[test]
#[ignore = "diagnostic geometry audit against the fixture's ground truth, run explicitly"]
fn diagnose_synthetic_orbit_geometry() {
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Quatd;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;
    let truth: serde_json::Value = serde_json::from_str(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let world_to_camera: Vec<Se3> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            let centre = [number(&c[0]), number(&c[1]), number(&c[2])];
            Se3 { r: rotation, t: scale3(rotation.act(centre), -1.0) }
        })
        .collect();
    let scene = <semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    let mut engine = ReconstructionEngine::new(&params);
    let mut intrinsics = None;
    for (index, (_, bytes)) in semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames().enumerate() {
        let image = semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image("image/png", bytes).expect("decodes");
        intrinsics.get_or_insert_with(|| default_intrinsics(image.width, image.height, params.assumed_focal_ratio, params.distortion));
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let intrinsics = intrinsics.expect("one frame");
    engine.observe();
    while engine.stage() != EngineStage::EstimatingPoses {
        if let EngineStatus::Failed(message) = engine.advance(1) {
            panic!("engine failed before pose estimation: {message}");
        }
    }
    let mut observations = Vec::new();
    engine.drain_observations(&mut observations);
    for observation in &observations {
        match observation {
            EngineObservation::PairMatched { frame_a, frame_b, matches } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) kept {matches} matches"),
            EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) dropped {dropped} matches"),
            _ => {}
        }
    }
    eprintln!("[GEO] {} observations, {} keypoints in frame 11, {} in frame 12", observations.len(), engine.keypoints_per_frame[11].len(), engine.keypoints_per_frame[12].len());
    let relative = |a: usize, b: usize| world_to_camera[b].semio_compose_rs(&world_to_camera[a].inverse());
    let essential = |pose: &Se3| {
        let t = pose.t;
        let skew = [[0.0, -t[2], t[1]], [t[2], 0.0, -t[0]], [-t[1], t[0], 0.0]];
        let r = mat3d_to_array_local(&pose.r.0);
        let mut e = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                e[i][j] = (0..3).map(|k| skew[i][k] * r[k][j]).sum();
            }
        }
        e
    };
    fn mat3d_to_array_local(m: &semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d) -> [[f64; 3]; 3] {
        std::array::from_fn(|r| std::array::from_fn(|c| m.cols[c][r]))
    }
    let sampson = |e: &[[f64; 3]; 3], a: [f64; 3], b: [f64; 3]| {
        let ea: [f64; 3] = std::array::from_fn(|i| (0..3).map(|k| e[i][k] * a[k]).sum());
        let etb: [f64; 3] = std::array::from_fn(|i| (0..3).map(|k| e[k][i] * b[k]).sum());
        let numerator: f64 = (0..3).map(|k| b[k] * ea[k]).sum::<f64>().powi(2);
        numerator / (ea[0] * ea[0] + ea[1] * ea[1] + etb[0] * etb[0] + etb[1] * etb[1]).max(1e-18)
    };
    for (a, b, matches) in &engine.pairwise_matches {
        let e = essential(&relative(*a, *b));
        let mut good = 0usize;
        let mut errors: Vec<f64> = Vec::new();
        for matched in matches {
            let ka = engine.keypoints_per_frame[*a][matched.a as usize];
            let kb = engine.keypoints_per_frame[*b][matched.b as usize];
            let ra = intrinsics.unproject_ray([f64::from(ka.x), f64::from(ka.y)]);
            let rb = intrinsics.unproject_ray([f64::from(kb.x), f64::from(kb.y)]);
            let error = sampson(&e, [ra[0], ra[1], 1.0], [rb[0], rb[1], 1.0]).sqrt();
            errors.push(error);
            if error < 0.005 {
                good += 1;
            }
        }
        errors.sort_by(f64::total_cmp);
        let median = errors.get(errors.len() / 2).copied().unwrap_or(f64::NAN);
        eprintln!("[GEO] pair ({a},{b}): {} matches, {good} true-epipolar inliers (<0.005 normalized), median sampson {median:.4}", matches.len());
    }
    {
        let cube_half = number(&truth["scene"]["halfExtentM"]);
        let camera_to_world = |frame: usize| world_to_camera[frame].inverse();
        let hit = |origin: [f64; 3], direction: [f64; 3]| -> Option<[f64; 3]> {
            let (mut t_min, mut t_max) = (-1.0e30f64, 1.0e30f64);
            for axis in 0..3 {
                let (d, o) = (direction[axis], origin[axis]);
                if d.abs() < 1e-12 {
                    if o.abs() > cube_half {
                        return None;
                    }
                    continue;
                }
                let (t1, t2) = ((-cube_half - o) / d, (cube_half - o) / d);
                let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
                t_min = t_min.max(lo);
                t_max = t_max.min(hi);
            }
            if t_max < t_min.max(0.0) || t_min <= 0.0 {
                return None;
            }
            Some(add3(origin, scale3(direction, t_min)))
        };
        let project = |frame: usize, point: [f64; 3]| -> Option<[f64; 2]> {
            let camera = world_to_camera[frame].act(point);
            (camera[2] > 1e-9).then(|| [intrinsics.cx + intrinsics.fx * camera[0] / camera[2], intrinsics.cy + intrinsics.fy * camera[1] / camera[2]])
        };
        let (a, b) = (0usize, 1usize);
        let pose_a = camera_to_world(a);
        let mut nearest_errors = Vec::new();
        let mut matched_errors = Vec::new();
        let mut off_cube = 0usize;
        let pair = engine.pairwise_matches.iter().find(|(x, y, _)| *x == a && *y == b).map(|(_, _, m)| m.clone()).unwrap_or_default();
        for (index, keypoint) in engine.keypoints_per_frame[a].iter().enumerate() {
            let ray = intrinsics.unproject_ray([f64::from(keypoint.x), f64::from(keypoint.y)]);
            let direction = normalize3(pose_a.r.act(ray));
            let Some(point) = hit(pose_a.t, direction) else { off_cube += 1; continue };
            let Some(expected) = project(b, point) else { continue };
            let nearest = engine.keypoints_per_frame[b].iter().map(|k| ((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt()).fold(f64::INFINITY, f64::min);
            nearest_errors.push(nearest);
            if let Some(matched) = pair.iter().find(|m| m.a as usize == index) {
                let k = engine.keypoints_per_frame[b][matched.b as usize];
                matched_errors.push(((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt());
            }
        }
        nearest_errors.sort_by(f64::total_cmp);
        matched_errors.sort_by(f64::total_cmp);
        let quantile = |v: &[f64], q: f64| v.get(((v.len() as f64 - 1.0) * q) as usize).copied().unwrap_or(f64::NAN);
        eprintln!("[GEO] frame {a}: {} keypoints, {off_cube} off the cube; nearest frame-{b} keypoint to the true transfer: median {:.2} px, 25% {:.2}, 75% {:.2}, within 2 px {}", engine.keypoints_per_frame[a].len(), quantile(&nearest_errors, 0.5), quantile(&nearest_errors, 0.25), quantile(&nearest_errors, 0.75), nearest_errors.iter().filter(|e| **e < 2.0).count());
        eprintln!("[GEO] matched keypoints vs true transfer: {} matched, median {:.2} px, within 2 px {}, within 5 px {}", matched_errors.len(), quantile(&matched_errors, 0.5), matched_errors.iter().filter(|e| **e < 2.0).count(), matched_errors.iter().filter(|e| **e < 5.0).count());
        let first: Vec<(f32, f32, u8, f32)> = engine.keypoints_per_frame[a].iter().take(8).map(|k| (k.x, k.y, k.octave, k.response)).collect();
        eprintln!("[GEO] first frame-{a} keypoints (x, y, octave, response): {first:?}");
        let mut true_distances = Vec::new();
        let mut best_other = Vec::new();
        let mut angle_deltas = Vec::new();
        let mut octaves = [0usize; 4];
        for keypoint in &engine.keypoints_per_frame[a] {
            octaves[usize::from(keypoint.octave).min(3)] += 1;
        }
        for (index, keypoint) in engine.keypoints_per_frame[a].iter().enumerate() {
            if keypoint.octave != 0 {
                continue;
            }
            let ray = intrinsics.unproject_ray([f64::from(keypoint.x), f64::from(keypoint.y)]);
            let direction = normalize3(pose_a.r.act(ray));
            let Some(point) = hit(pose_a.t, direction) else { continue };
            let Some(expected) = project(b, point) else { continue };
            let Some((partner, _)) = engine.keypoints_per_frame[b].iter().enumerate().filter(|(_, k)| k.octave == 0).map(|(j, k)| (j, ((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt())).filter(|(_, d)| *d < 1.5).min_by(|x, y| x.1.total_cmp(&y.1)) else { continue };
            let descriptor = &engine.descriptors_per_frame[a][index];
            let distance = descriptor_distance(descriptor, &engine.descriptors_per_frame[b][partner]);
            let other = engine.descriptors_per_frame[b].iter().enumerate().filter(|(j, _)| *j != partner).map(|(_, d)| descriptor_distance(descriptor, d)).min().unwrap_or(u32::MAX);
            true_distances.push(distance);
            best_other.push(other);
            let delta = (keypoint.angle - engine.keypoints_per_frame[b][partner].angle).rem_euclid(std::f32::consts::TAU);
            angle_deltas.push(delta.min(std::f32::consts::TAU - delta).to_degrees());
        }
        true_distances.sort_unstable();
        best_other.sort_unstable();
        angle_deltas.sort_by(f32::total_cmp);
        let mid = |v: &[u32]| v.get(v.len() / 2).copied().unwrap_or(0);
        eprintln!("[GEO] octaves of frame-{a} keypoints: {octaves:?}; {} true base-level pairs: median Hamming of the true pair {} (min {}), median best other {}, median |Δangle| {:.1}°", true_distances.len(), mid(&true_distances), true_distances.first().copied().unwrap_or(0), mid(&best_other), angle_deltas.get(angle_deltas.len() / 2).copied().unwrap_or(0.0));
    }
    let mut seen_cursor = usize::MAX;
    while engine.stage() == EngineStage::EstimatingPoses {
        if let EngineStatus::Failed(message) = engine.advance(1) {
            panic!("engine failed during pose estimation: {message}");
        }
        if engine.pose_cursor != seen_cursor {
            seen_cursor = engine.pose_cursor;
            if let Some(sfm) = engine.sfm.as_ref() {
                if seen_cursor < engine.frames.len() {
                    eprintln!("[GEO] registering frame {seen_cursor}: {} 2D-3D correspondences, {} points, {} cameras so far", sfm.correspondence_count(seen_cursor), sfm.point_count(), sfm.registered_count());
                    if seen_cursor <= 8 {
                        let registered: std::collections::BTreeSet<usize> = (0..seen_cursor).filter(|&f| sfm.camera_pose(f).is_some()).collect();
                        let mut by_span = std::collections::BTreeMap::<usize, (usize, usize)>::new();
                        for (track, has_point) in sfm.tracks_observing(seen_cursor) {
                            let span = track.iter().filter(|(f, _)| registered.contains(f)).count();
                            let entry = by_span.entry(span).or_insert((0, 0));
                            entry.0 += 1;
                            if has_point {
                                entry.1 += 1;
                            }
                        }
                        eprintln!("[GEO]   frame {seen_cursor} tracks by registered span (span: tracks/with point): {by_span:?}");
                    }
                }
            }
        }
    }
    let report = |label: &str, registered: &[(usize, remodeling_camera::CameraPose)]| {
        eprintln!("[GEO] {label}: registered {:?}", registered.iter().map(|(frame, _)| *frame).collect::<Vec<_>>());
        let (first, first_pose) = registered[0];
        let mut worst = 0.0f64;
        for &(frame, pose) in &registered[1..] {
            let recovered = pose.0.semio_compose_rs(&first_pose.0.inverse());
            let expected = relative(first, frame);
            let rotation_error = norm3(expected.r.inverse().semio_compose_rs(&recovered.r).log()).to_degrees();
            worst = worst.max(rotation_error);
            let direction = |t: [f64; 3]| normalize3(t);
            let cosine = { let (u, v) = (direction(recovered.t), direction(expected.t)); u[0] * v[0] + u[1] * v[1] + u[2] * v[2] };
            eprintln!("[GEO] {label}: camera {frame} relative to {first}: rotation error {rotation_error:.2}°, translation direction error {:.2}°, baseline recovered {:.3} true {:.3}", cosine.clamp(-1.0, 1.0).acos().to_degrees(), norm3(recovered.t), norm3(expected.t));
        }
        eprintln!("[GEO] {label}: worst rotation error {worst:.2}°");
    };
    let reconstruction = engine.sfm.as_ref().expect("sfm");
    let registered: Vec<(usize, remodeling_camera::CameraPose)> = (0..engine.frames.len()).filter_map(|frame| reconstruction.camera_pose(frame).map(|pose| (frame, pose))).collect();
    report("after registration", &registered);
    while engine.stage() == EngineStage::BundleAdjusting {
        if let EngineStatus::Failed(message) = engine.advance(64) {
            panic!("engine failed during bundle adjustment: {message}");
        }
    }
    let finalized = engine.reconstruction.as_ref().expect("finalized reconstruction");
    report("after bundle", &finalized.cameras);
}

/// 🔭️ Dense-stage audit of the shipped `synthetic-orbit` example against its ground truth: runs the
/// engine to the end and scores, in the truth frame (Sim(3) over the registered camera centres), the
/// sparse points, every depth map (object vs background pixels, depth error, object coverage), the
/// fused cloud and the final mesh by their distance to the cube's surface in half-sizes. Diagnostic,
/// therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture diagnose_synthetic_orbit_dense_against_truth`.
#[test]
#[ignore = "diagnostic dense audit against the fixture's ground truth, run explicitly"]
fn diagnose_synthetic_orbit_dense_against_truth() {
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Quatd;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
    use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;
    let truth: serde_json::Value = serde_json::from_str(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let half = number(&truth["scene"]["halfExtentM"]);
    let truth_world_to_camera: Vec<Se3> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            Se3 { r: rotation, t: scale3(rotation.act([number(&c[0]), number(&c[1]), number(&c[2])]), -1.0) }
        })
        .collect();
    let surface = |p: [f64; 3]| {
        let outside = [(p[0].abs() - half).max(0.0), (p[1].abs() - half).max(0.0), (p[2].abs() - half).max(0.0)];
        let outside = (outside[0] * outside[0] + outside[1] * outside[1] + outside[2] * outside[2]).sqrt();
        let inside = (half - p[0].abs()).min(half - p[1].abs()).min(half - p[2].abs()).max(0.0);
        (outside + inside) / half
    };
    let hit = |origin: [f64; 3], direction: [f64; 3]| -> Option<f64> {
        let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
        for axis in 0..3 {
            if direction[axis].abs() < 1e-12 {
                if origin[axis].abs() > half {
                    return None;
                }
                continue;
            }
            let a = (-half - origin[axis]) / direction[axis];
            let b = (half - origin[axis]) / direction[axis];
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
        (near <= far && far > 0.0).then_some(near.max(0.0))
    };
    let quantiles = |mut values: Vec<f64>| -> String {
        if values.is_empty() {
            return "n=0".into();
        }
        values.sort_by(f64::total_cmp);
        let q = |f: f64| values[((values.len() as f64 - 1.0) * f).round() as usize];
        format!("n={} p10 {:.3} median {:.3} p90 {:.3}", values.len(), q(0.1), q(0.5), q(0.9))
    };

    let scene = <semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    eprintln!("[DENSE] params dense={:?} voxel={} trunc={} dense_source_views={} max_dense_cameras={}", params.dense, params.tsdf_voxel_size, params.tsdf_truncation, params.dense_source_views, params.max_dense_cameras);
    let mut engine = ReconstructionEngine::new(&params);
    for (index, (_, bytes)) in semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames().enumerate() {
        let image = semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image("image/png", bytes).expect("decodes");
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let mut captured: Option<(remodeling_sfm::Reconstruction, Vec<usize>, Vec<remodeling_dense::DepthMap>)> = None;
    let mut cloud: Vec<[f64; 3]> = Vec::new();
    let mut last = engine.stage();
    let mut volume: Option<(remodeling_dense::TsdfVolume, [i32; 3], [i32; 3], [f64; 3], [f64; 3])> = None;
    let mut meshes: Vec<(usize, Vec<[f64; 3]>)> = Vec::new();
    loop {
        let budget = if matches!(engine.stage(), EngineStage::FusingVolume | EngineStage::ExtractingSurface | EngineStage::CleaningMesh | EngineStage::Texturing) { 1 } else { 64 };
        let status = engine.advance(budget);
        if volume.is_none() && engine.stage() == EngineStage::FusingVolume && engine.fusion_finalized && engine.stage_cursor >= engine.dense_camera_indices.len() {
            if let (Some(tsdf), Some(preparation)) = (&engine.tsdf, &engine.meshing_preparation) {
                let (lattice_min, lattice_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, engine.meshing_voxel().0);
                volume = Some((tsdf.clone(), lattice_min, lattice_max, preparation.bounds_min, preparation.bounds_max));
            }
        }
        if let Some(pipeline) = &engine.mesh_pipeline {
            let triangles = pipeline.mesh().triangles.len();
            if meshes.last().is_none_or(|(count, _)| *count != triangles) {
                meshes.push((triangles, pipeline.mesh().positions.clone()));
            }
        }
        if engine.stage() != last {
            eprintln!("[DENSE] {last:?} -> {:?}", engine.stage());
            if last == EngineStage::DenseStereo {
                captured = Some((engine.reconstruction.clone().expect("reconstruction"), engine.dense_camera_indices.clone(), engine.depth_maps.clone()));
            }
            if !engine.dense_positions().is_empty() && cloud.is_empty() {
                cloud = engine.dense_positions().to_vec();
            }
            last = engine.stage();
        }
        if cloud.is_empty() && !engine.dense_positions().is_empty() {
            cloud = engine.dense_positions().to_vec();
        }
        match status {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(message) => panic!("engine failed: {message}"),
        }
    }
    let (reconstruction, dense_indices, depth_maps) = captured.expect("the run passed the dense stage");
    if let Ok(path) = std::env::var("DENSE_DUMP") {
        std::fs::write(&path, dense_dump::encode(&reconstruction, &dense_indices, &depth_maps)).expect("write dense dump");
        eprintln!("[DENSE] dumped to {path}");
    }
    let recovered: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(_, pose)| pose.0.inverse().t).collect();
    let targets: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(frame, _)| truth_world_to_camera[*frame].inverse().t).collect();
    let sim = semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::umeyama(&recovered, &targets, true).expect("Sim(3) over camera centres");
    let centre_errors: Vec<f64> = recovered.iter().zip(&targets).map(|(r, t)| norm3(sub3(sim.act(*r), *t))).collect();
    eprintln!("[DENSE] {} cameras, sim scale {:.4}, centre error {}", recovered.len(), sim.s, quantiles(centre_errors));
    eprintln!("[DENSE] sparse surface distance {}", quantiles(reconstruction.points.iter().map(|p| surface(sim.act(*p))).collect()));

    let intrinsics = reconstruction.intrinsics;
    for (slot, (camera, map)) in dense_indices.iter().zip(&depth_maps).enumerate() {
        let (frame, pose) = reconstruction.cameras[*camera];
        let truth_pose = truth_world_to_camera[frame];
        let truth_centre = truth_pose.inverse().t;
        let camera_to_world = truth_pose.r.inverse();
        let (mut object_pixels, mut object_valid, mut background_valid) = (0usize, 0usize, 0usize);
        let (mut object_errors, mut background_errors, mut depth_errors) = (Vec::new(), Vec::new(), Vec::new());
        for y in 0..map.height {
            for x in 0..map.width {
                let ray = intrinsics.unproject_ray([f64::from(x), f64::from(y)]);
                let direction = camera_to_world.act(ray);
                let length = norm3(direction);
                let truth_depth = hit(truth_centre, scale3(direction, 1.0 / length)).map(|distance| distance / length);
                if truth_depth.is_some() {
                    object_pixels += 1;
                }
                let Some(depth) = map.get(x, y) else { continue };
                let world = sim.act(pose.0.inverse().act(scale3(ray, f64::from(depth))));
                match truth_depth {
                    Some(expected) => {
                        object_valid += 1;
                        object_errors.push(surface(world));
                        depth_errors.push((f64::from(depth) * sim.s - expected).abs() / half);
                    }
                    None => {
                        background_valid += 1;
                        background_errors.push(surface(world));
                    }
                }
            }
        }
        eprintln!(
            "[DENSE] map {slot} frame {frame}: object px {object_pixels}, valid on object {object_valid} ({:.0}%), valid on background {background_valid}; object surface dist {}; |depth err| {}; background surface dist {}",
            100.0 * object_valid as f64 / object_pixels.max(1) as f64,
            quantiles(object_errors),
            quantiles(depth_errors),
            quantiles(background_errors)
        );
    }
    let views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)> = dense_indices.iter().map(|camera| (reconstruction.cameras[*camera].1, intrinsics)).collect();
    for (tolerance, minimum) in [(0.01f32, 2usize), (0.02, 2), (0.03, 2), (0.02, 3), (0.03, 3), (0.05, 3)] {
        let fused = remodeling_dense::fuse_depth_maps(&views, &depth_maps, &remodeling_dense::FusionConfig { max_relative_depth_diff: tolerance, max_normal_angle_deg: 30.0, min_consistent_views: minimum });
        let distances: Vec<f64> = fused.positions.iter().map(|p| surface(sim.act(*p))).collect();
        let far = distances.iter().filter(|distance| **distance > 0.2).count();
        eprintln!("[DENSE] fusion tolerance {tolerance} views {minimum}: {} points ({far} > 0.2 half-sizes off), surface distance {}", fused.positions.len(), quantiles(distances));
    }
    if let Some((tsdf, lattice_min, lattice_max, extrema_min, extrema_max)) = &volume {
        eprintln!("[DENSE] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {lattice_min:?}..{lattice_max:?} blocks {}", tsdf.block_count());
        let voxel = engine.meshing_voxel().0;
        let (mut unobserved, mut agree, mut disagree) = (0usize, 0usize, 0usize);
        let mut disagreeing_depth = Vec::new();
        for z in lattice_min[2]..=lattice_max[2] {
            for y in lattice_min[1]..=lattice_max[1] {
                for x in lattice_min[0]..=lattice_max[0] {
                    let centre = sim.act([(f64::from(x) + 0.5) * voxel, (f64::from(y) + 0.5) * voxel, (f64::from(z) + 0.5) * voxel]);
                    let inside = centre.iter().all(|c| c.abs() < half);
                    match tsdf.sample(x, y, z) {
                        None => unobserved += 1,
                        Some((sdf, _)) => {
                            if (sdf < 0.0) == inside {
                                agree += 1;
                            } else {
                                disagree += 1;
                                disagreeing_depth.push(surface(centre));
                            }
                        }
                    }
                }
            }
        }
        let truth_points: Vec<[f64; 3]> = truth["pointsWorldM"].as_array().expect("points").iter().map(|p| { let p = p.as_array().expect("point"); [number(&p[0]), number(&p[1]), number(&p[2])] }).collect();
        let mut fusion = remodeling_dense::FusionPreparation::new(0);
        while !fusion.advance(&views, &depth_maps, &engine.fusion_config(), usize::MAX / 2) {}
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
        let grid: Vec<(f64, f64)> = std::env::var("DENSE_GRID").ok().map_or_else(
            || vec![(0.1, 0.33), (0.1, 0.2), (0.1, 0.15), (0.15, 0.3), (0.2, 0.2), (0.2, 0.3), (0.2, 0.4), (0.25, 0.25), (0.25, 0.5)],
            |text| text.split(';').map(|pair| { let (v, t) = pair.split_once(',').expect("voxel,truncation"); (v.parse().expect("voxel"), t.parse().expect("truncation")) }).collect(),
        );
        for (voxel_size, truncation) in grid {
            let (grid_min, grid_max) = compute_voxel_bounds_from_extrema(*extrema_min, *extrema_max, voxel_size);
            let mut volume = remodeling_dense::TsdfVolume::new(voxel_size, truncation).with_voxel_bounds(grid_min.map(|c| c - 2), grid_max.map(|c| c + 2));
            for (map, view) in masked.iter().zip(&views) {
                volume.integrate(map, view, true);
            }
            let mut pipeline = remodeling_mesh::MeshPipeline::new_bounded(volume, 0.0, grid_min, grid_max, engine.params.mesh.clone());
            let outcome = loop {
                match remodeling_mesh::mesh_pipeline_step(&mut pipeline, 1) {
                    remodeling_mesh::MeshPipelineStatus::Working { .. } => {}
                    remodeling_mesh::MeshPipelineStatus::Done => break "done".to_string(),
                    remodeling_mesh::MeshPipelineStatus::Failed(message) => break message.chars().take(90).collect(),
                }
            };
            let positions: Vec<[f64; 3]> = pipeline.result().map_or_else(|| pipeline.mesh().positions.iter().map(|p| sim.act(*p)).collect(), |mesh| mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect());
            let covered = truth_points.iter().filter(|point| positions.iter().any(|vertex| norm3(sub3(*vertex, **point)) <= 0.4)).count();
            eprintln!("[DENSE] grid voxel {voxel_size} trunc {truncation}: {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map(|p| surface(*p)).collect()), 100.0 * covered as f64 / truth_points.len() as f64);
        }
        eprintln!("[DENSE] tsdf lattice voxels: unobserved {unobserved}, sign agrees with truth {agree}, disagrees {disagree}; disagreeing voxels' surface distance {}", quantiles(disagreeing_depth));
    }
    for (triangles, positions) in &meshes {
        eprintln!("[DENSE] pipeline mesh {triangles} triangles / {} vertices: surface distance {}", positions.len(), quantiles(positions.iter().map(|p| surface(sim.act(*p))).collect()));
    }
    eprintln!("[DENSE] fused cloud surface distance {}", quantiles(cloud.iter().map(|p| surface(sim.act(*p))).collect()));
    let mesh = engine.mesh_data.as_ref().expect("mesh");
    let vertices: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect();
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for vertex in &vertices {
        for axis in 0..3 {
            lo[axis] = lo[axis].min(vertex[axis]);
            hi[axis] = hi[axis].max(vertex[axis]);
        }
    }
    eprintln!("[DENSE] mesh {} vertices / {} triangles, aligned bounds {lo:.2?}..{hi:.2?}, surface distance {}", vertices.len(), mesh.indices.len() / 3, quantiles(vertices.iter().map(|v| surface(*v)).collect()));
}
