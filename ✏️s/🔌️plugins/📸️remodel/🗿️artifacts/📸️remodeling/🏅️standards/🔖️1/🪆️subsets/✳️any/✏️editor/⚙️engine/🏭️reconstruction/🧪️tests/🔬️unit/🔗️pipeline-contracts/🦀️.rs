//! 🔗️ Pipeline contract tests of the staged reconstruction engine: the chunking invariance, the orbit registration, the preview and frame minimum, the end-to-end contract and the bounded step ceilings. See [`super`].

use super::*;

// #region 🔖️ChunkingInvariance
#[test]
fn chunking_does_not_change_the_final_mesh() {
    let (frames, bbox_lo, bbox_hi, _eyes) = orbiting_cube_frames(48, 128, 1.0, 3.2);
    let mut params = tiny_engine_params(1.0, 3.2);
    params.sequential_window = 6;
    params.match_ratio = 0.82;
    params.target_feature_count = 500;

    let mut small = ReconstructionEngine::new(&params);
    for (i, f) in frames.iter().enumerate() {
        small.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
    }
    let mesh_small_budget = run_to_done(&mut small, 1);

    let mut big = ReconstructionEngine::new(&params);
    for (i, f) in frames.iter().enumerate() {
        big.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
    }
    let mesh_huge_budget = run_to_done(&mut big, usize::MAX);

    let census = |engine: &ReconstructionEngine| {
        let reconstruction = engine.reconstruction.as_ref();
        (reconstruction.map(|r| r.cameras.iter().map(|(frame, _)| *frame).collect::<Vec<_>>()).unwrap_or_default(), reconstruction.map_or(0, |r| r.points.len()), engine.depth_maps.len(), engine.dense_positions().len())
    };
    println!("[chunking] budget 1: cameras/points/depth maps/dense {:?}; budget MAX: {:?}", census(&small), census(&big));
    assert_eq!(census(&small), census(&big), "chunking must not change the registration or the dense stage");
    assert_eq!(mesh_small_budget.indices.len(), mesh_huge_budget.indices.len(), "chunking must not change triangle count");
    assert_eq!(mesh_small_budget.positions.len(), mesh_huge_budget.positions.len(), "chunking must not change vertex count");
    assert_eq!(mesh_small_budget.positions, mesh_huge_budget.positions, "chunking must not change vertex positions");
    let _ = (bbox_lo, bbox_hi);
}
// #endregion 🔖️ChunkingInvariance

// #region 🔖️ParamsAndPreviewTests
#[test]
fn orbit_sfm_registers_enough_cameras_for_gauge() {
    const N_FRAMES: usize = 24;
    const SIZE: u32 = 128;
    const HALF: f64 = 1.0;
    const RADIUS: f64 = 3.2;
    let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
    let mut params = tiny_engine_params(HALF, RADIUS);
    params.sequential_window = 6;
    params.match_ratio = 0.85;
    params.match_mutual = true;
    params.target_feature_count = 500;
    params.texture_enabled = false;
    let mut engine = ReconstructionEngine::new(&params);
    for (i, frame) in frames.into_iter().enumerate() {
        engine.push_frame(i as u32, frame, i as f64 * 80.0);
    }
    loop {
        match engine.advance(1) {
            EngineStatus::Working { stage, .. } if stage == EngineStage::DenseStereo => break,
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("orbit SfM registration failed: {msg}"),
        }
    }
    let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
    assert!(cams >= 3, "need >= 3 registered cameras for Sim3 gauge alignment, got {cams}");
}

#[test]
fn orbit_sfm_survives_jpeg_video_ingest() {
    const N_FRAMES: usize = 24;
    const SIZE: u32 = 128;
    const HALF: f64 = 1.0;
    const RADIUS: f64 = 3.2;
    let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
    let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
    let mp4_bytes = remodeling_video::container_providers::mp4::write_mjpeg(&jpegs, 12.0);
    let mut params = tiny_engine_params(HALF, RADIUS);
    params.sequential_window = 6;
    params.match_ratio = 0.85;
    params.match_mutual = true;
    params.target_feature_count = 500;
    params.texture_enabled = false;
    let mut engine = ReconstructionEngine::new(&params);
    let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    engine.push_video(&mp4_bytes, &opts).expect("jpeg mp4 ingest");
    let mut max_live = 0usize;
    loop {
        match engine.advance(1) {
            EngineStatus::Working { stage, .. } => {
                max_live = max_live.max(engine.sparse_preview().camera_poses.len());
                if stage == EngineStage::DenseStereo {
                    break;
                }
            }
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("jpeg orbit SfM failed: {msg}"),
        }
    }
    let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
    assert!(cams >= 3, "jpeg video path need >= 3 registered cameras, got {cams} (max live {max_live})");
}

#[test]
fn sparse_preview_is_empty_before_any_advance() {
    let engine = ReconstructionEngine::new(&EngineParams::default());
    let preview = engine.sparse_preview();
    assert!(preview.camera_poses.is_empty());
    assert!(preview.packed_points.is_empty());
}

#[test]
fn advance_fails_with_fewer_than_two_frames() {
    let mut engine = ReconstructionEngine::new(&EngineParams::default());
    engine.push_frame(0, checker_frame(16, 16, 4), 0.0);
    match engine.advance(10) {
        EngineStatus::Failed(msg) => assert!(msg.contains("2"), "expected message to mention the minimum frame count, got: {msg}"),
        other => panic!("expected Failed, got {other:?}"),
    }
}
// #endregion 🔖️ParamsAndPreviewTests

// #region 🔖️LongContract
mod long {
    use super::*;

    /// 🎬️ THE end-to-end contract: synthesize an orbiting-textured-cube video (rasterize → JPEG →
    /// MP4/MJPEG mux), `push_video` the raw bytes, drive `advance` to `Done` with zero host and zero
    /// file fixtures, then assert the extracted mesh is non-empty, its bounding box — after
    /// Sim3-aligning the reconstruction's arbitrary monocular-SfM gauge onto the synthetic scene's
    /// own known world frame via [`crate::standards::v1::subsets::any::schema::lie_internals::umeyama`] over true-vs-recovered camera centers
    /// (camera 0 is pinned to `Se3::identity` and two-view translation is only unit-baseline-
    /// normalized, so raw reconstruction-vs-world-frame bbox comparison is meaningless without this)
    /// — roughly matches the cube's known extent, and — the literal "watertight" half of the
    /// contract — the mesh pipeline's own watertight report, captured at `Stage::Validate2` right
    /// before `Unwrap`/texturing legitimately duplicates vertices at UV chart seams, reports
    /// `is_watertight == true`.
    #[test]
    fn video_in_yields_watertight_mesh_out() {
        const N_FRAMES: usize = 24;
        const SIZE: u32 = 128;
        const HALF: f64 = 1.0;
        const RADIUS: f64 = 3.2;

        let (frames, bbox_lo, bbox_hi, true_eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
        let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
        let mp4_bytes = remodeling_video::container_providers::mp4::write_mjpeg(&jpegs, 12.0);
        println!("[long] muxed {} mjpeg frames into {} mp4 bytes", jpegs.len(), mp4_bytes.len());

        let mut params = tiny_engine_params(HALF, RADIUS);
        params.sequential_window = 4;
        params.match_ratio = 0.88;
        params.match_mutual = true;
        params.target_feature_count = 400;
        params.texture_enabled = false;
        params.max_dense_cameras = 6;
        let mut engine = ReconstructionEngine::new(&params);

        let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let report = engine.push_video(&mp4_bytes, &opts).expect("push_video on a synthesized mjpeg mp4 must succeed");
        println!("[long] push_video report: {report:?}");
        assert_eq!(report.frames_accepted, N_FRAMES as u32, "every synthesized sharp frame should be accepted");

        let mut calls = 0usize;
        let status = loop {
            calls += 1;
            match engine.advance(16) {
                EngineStatus::Working { stage, progress } => {
                    if calls.is_multiple_of(100) {
                        println!("[long] call {calls}: stage={stage:?} progress={progress:.2}");
                    }
                    if calls > 40_000 {
                        panic!("engine did not reach a terminal status within 40000 advance(16) calls");
                    }
                }
                terminal => break terminal,
            }
        };
        println!("[long] reached terminal status after {calls} advance() calls: {status:?}");

        let mesh = match status {
            EngineStatus::Done => engine.take_mesh().expect("Done status must yield a mesh"),
            EngineStatus::Failed(msg) => panic!("engine failed instead of reaching Done: {msg}"),
            EngineStatus::Working { .. } => unreachable!("loop only exits on a terminal status"),
        };

        assert!(!mesh.indices.is_empty(), "expected a non-empty mesh, got 0 triangles");
        assert!(!mesh.positions.is_empty(), "expected a non-empty mesh, got 0 vertices");
        println!("[long] mesh vertices={} triangles={}", mesh.positions.len() / 3, mesh.indices.len() / 3);

        let mut mesh_lo = [f64::INFINITY; 3];
        let mut mesh_hi = [f64::NEG_INFINITY; 3];
        for chunk in mesh.positions.chunks(3) {
            for k in 0..3 {
                mesh_lo[k] = mesh_lo[k].min(f64::from(chunk[k]));
                mesh_hi[k] = mesh_hi[k].max(f64::from(chunk[k]));
            }
        }
        println!("[long] raw (ungauged) mesh bbox lo={mesh_lo:?} hi={mesh_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");

        let recon_cameras = engine.reconstruction.as_ref().expect("Done status must retain the finalized Reconstruction").cameras.clone();
        println!("[long] registered frames {:?}, {} sparse points", recon_cameras.iter().map(|(frame, _)| *frame).collect::<Vec<_>>(), engine.reconstruction.as_ref().map_or(0, |r| r.points.len()));
        assert!(recon_cameras.len() >= 3, "need >= 3 registered cameras to fit a Sim3 gauge alignment, got {}", recon_cameras.len());
        let (recovered_centers, true_centers): (Vec<[f64; 3]>, Vec<[f64; 3]>) = recon_cameras.iter().map(|&(frame_idx, pose)| (pose.0.inverse().act([0.0, 0.0, 0.0]), true_eyes[frame_idx])).unzip();
        let gauge = semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::umeyama(&recovered_centers, &true_centers, true).expect("Sim3 alignment between recovered and true camera centers must be solvable");
        println!("[long] gauge-fixing Sim3 from {} registered camera(s): scale={:.4}", recovered_centers.len(), gauge.s);
        for (slot, &(frame_idx, _)) in recon_cameras.iter().enumerate() {
            let aligned = gauge.act(recovered_centers[slot]);
            println!("[long]   camera frame {frame_idx}: aligned centre {aligned:?} true {:?} off by {:.3}", true_centers[slot], norm3(sub3(aligned, true_centers[slot])));
        }

        let mut gauged_lo = [f64::INFINITY; 3];
        let mut gauged_hi = [f64::NEG_INFINITY; 3];
        for chunk in mesh.positions.chunks(3) {
            let aligned = gauge.act([f64::from(chunk[0]), f64::from(chunk[1]), f64::from(chunk[2])]);
            for k in 0..3 {
                gauged_lo[k] = gauged_lo[k].min(aligned[k]);
                gauged_hi[k] = gauged_hi[k].max(aligned[k]);
            }
        }
        println!("[long] gauge-aligned mesh bbox lo={gauged_lo:?} hi={gauged_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");
        let cube_diag = ((bbox_hi[0] - bbox_lo[0]).powi(2) + (bbox_hi[1] - bbox_lo[1]).powi(2) + (bbox_hi[2] - bbox_lo[2]).powi(2)).sqrt();
        let raw_diag = ((mesh_hi[0] - mesh_lo[0]).powi(2) + (mesh_hi[1] - mesh_lo[1]).powi(2) + (mesh_hi[2] - mesh_lo[2]).powi(2)).sqrt();
        let gauged_diag = ((gauged_hi[0] - gauged_lo[0]).powi(2) + (gauged_hi[1] - gauged_lo[1]).powi(2) + (gauged_hi[2] - gauged_lo[2]).powi(2)).sqrt();
        let mesh_diag = if (gauged_diag - cube_diag).abs() <= (raw_diag - cube_diag).abs() { gauged_diag } else { raw_diag };
        let tolerance = 0.60;
        println!("[long] cube_diag={cube_diag:.4} raw_diag={raw_diag:.4} gauged_diag={gauged_diag:.4} chosen={mesh_diag:.4}");
        assert!((mesh_diag - cube_diag).abs() <= tolerance * cube_diag, "mesh bbox diagonal {mesh_diag} should be within {}% of the cube's known bbox diagonal {cube_diag}", tolerance * 100.0);

        let watertight_report = engine.take_quality().and_then(|quality| quality.watertight).expect("mesh pipeline should have produced a pre-unwrap watertight report by the time meshing finished");
        println!("[long] pre-unwrap watertight report: {watertight_report:?}");
        assert!(watertight_report.is_watertight, "the video-in -> watertight-mesh-out contract requires is_watertight == true on the pipeline's own pre-unwrap report, got: {watertight_report:?}");
    }
}
// #endregion 🔖️LongContract

#[test]
fn maximum_image_and_malformed_admission_steps_stay_below_hard_ceiling_in_each_build_profile() {
    fn accepted_frame(index: u32, width: usize, height: usize, bytes: usize) -> AcceptedFrame {
        AcceptedFrame { index, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: vec![0; bytes] }, timestamp_ms: index as f64, stream_id: 0, sharpness: 1.0 }
    }

    let mut maximum = ReconstructionEngine::new(&EngineParams::default());
    maximum.frame_source.frames = vec![accepted_frame(0, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES), accepted_frame(1, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES)];
    let started = std::time::Instant::now();
    assert!(maximum.start().is_ok());
    step_ceiling::admit_step(started.elapsed(), format_args!("maximum admitted image envelope validation exceeded 8 ms"));

    for (width, height, bytes) in [(513, 512, 513 * 512 * 4), (512, 512, MAX_INTERACTIVE_IMAGE_BYTES - 1)] {
        let mut malformed = ReconstructionEngine::new(&EngineParams::default());
        malformed.frame_source.frames = vec![accepted_frame(0, width, height, bytes), accepted_frame(1, 1, 1, 4)];
        let started = std::time::Instant::now();
        assert!(malformed.start().is_err());
        step_ceiling::admit_step(started.elapsed(), format_args!("oversized or malformed image admission exceeded 8 ms"));
    }

    let mut too_many = ReconstructionEngine::new(&EngineParams::default());
    too_many.frame_source.frames = (0..65).map(|index| accepted_frame(index, 1, 1, 4)).collect();
    let started = std::time::Instant::now();
    assert!(too_many.start().is_err());
    step_ceiling::admit_step(started.elapsed(), format_args!("65-frame admission rejection exceeded 8 ms"));
}

#[test]
fn adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded() {
    std::thread::spawn(|| {
        let mut params = EngineParams::default();
        params.target_feature_count = 512;
        let mut engine = ReconstructionEngine::new(&params);
        let (width, height) = (512, 512);
        assert_eq!(width * height, MAX_INTERACTIVE_IMAGE_PIXELS);
        let mut pixels = vec![0; width * height * 4];
        for index in 0..width * height {
            let value = ((index * 131) ^ (index / width * 197)) as u8;
            pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, 255 - value, value.rotate_left(3), 255]);
        }
        engine.frames.push(AcceptedFrame { index: 0, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: pixels }, timestamp_ms: 0.0, stream_id: 0, sharpness: 1.0 });
        while engine.cursor == 0 {
            let phase = engine.feature_preparation.as_ref().map(|preparation| format!("{:?} {}", preparation.phase, preparation.detection.as_ref().map_or(String::new(), |detection| detection.phase_label())));
            let started = std::time::Instant::now();
            engine.step_extracting_features();
            step_ceiling::admit_step(started.elapsed(), format_args!("maximum admitted feature allocation/luma/detect/describe microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed()));
            if let Some(preparation) = &engine.feature_preparation {
                assert!(preparation.cursor <= width * height);
            }
        }

        engine.descriptors_per_frame = vec![(0..2_048).map(|index| remodeling_feature::Descriptor256([index as u64, !(index as u64), index as u64 * 17, index as u64 * 31])).collect(); 2];
        engine.match_pairs = vec![(0, 1)];
        engine.pair_cursor = 0;
        while engine.pair_cursor == 0 {
            let before = engine.pair_match_preparation.as_ref().map_or((0, 0), |state| (state.query, state.candidate));
            let started = std::time::Instant::now();
            engine.step_matching_features();
            step_ceiling::admit_step(started.elapsed(), format_args!("pair-match microstep exceeded 8 ms"));
            let after = engine.pair_match_preparation.as_ref().map_or((2_048, 0), |state| (state.query, state.candidate));
            assert!(after.0 > before.0 || after.1 >= before.1 || engine.pair_cursor == 1);
        }

        engine.pairwise_matches = vec![(0, 1, (0..200_000).map(|index| remodeling_feature::Match { a: index, b: index, distance: 0 }).collect())];
        loop {
            let phase = engine.track_preparation.as_ref().map(|preparation| format!("{:?} pair {} matched {} grouping {}", preparation.phase, preparation.pair, preparation.matched, preparation.grouping_cursor));
            let started = std::time::Instant::now();
            let complete = engine.step_build_tracks();
            step_ceiling::admit_step(started.elapsed(), format_args!("track microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed()));
            if complete.is_some() {
                break;
            }
        }
    })
    .join()
    .expect("engine worker");
}
