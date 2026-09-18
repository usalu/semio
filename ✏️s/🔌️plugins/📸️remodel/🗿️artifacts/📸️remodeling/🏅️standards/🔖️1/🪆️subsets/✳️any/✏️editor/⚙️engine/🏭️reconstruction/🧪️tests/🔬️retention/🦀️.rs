//! 🧮️ Peak-retention probe for the staged reconstruction engine.
//!
//! The guest runs the pipeline inside a 512 MiB linear memory whose allocator never returns pages, so
//! what decides whether a run finishes is the *high-water sum of live structures*, not the total bytes
//! ever touched. This probe drives the shipped `synthetic-orbit` example (thirty-six 320x240 views) through
//! [`ReconstructionEngine::advance`] exactly as the run job does and prints the retained bytes of every
//! big structure at each stage boundary, plus a per-camera line through the dense/fusion stages where
//! the depth maps and the TSDF grid grow.
//!
//! Diagnostic, therefore `#[ignore]`:
//! `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture engine::reconstruction::retention`

use super::*;

/// 🛑️ Retained-TSDF ceiling this probe refuses to cross: past it the native host would start swapping
/// for numbers that are already conclusive, so the probe reports the growth and stops.
const TSDF_REPORT_CEILING_BYTES: usize = 320 * 1024 * 1024;

fn vec_bytes<T>(v: &[T]) -> usize {
    size_of_val(v)
}

fn nested_bytes<T>(v: &[Vec<T>]) -> usize {
    v.len() * size_of::<Vec<T>>() + v.iter().map(|inner| vec_bytes(inner.as_slice())).sum::<usize>()
}

/// 🧾️ Retained bytes per structure, in the order the pipeline fills them.
struct Retention {
    frames: usize,
    keypoints: usize,
    descriptors: usize,
    pairwise: usize,
    tracks: usize,
    sfm: usize,
    reconstruction: usize,
    observations: usize,
    recorded: usize,
    depth_maps: usize,
    dense_preparation: usize,
    tsdf: usize,
    dense_cloud: usize,
    meshing_preparation: usize,
}

impl Retention {
    fn total(&self) -> usize {
        self.frames
            + self.keypoints
            + self.descriptors
            + self.pairwise
            + self.tracks
            + self.sfm
            + self.reconstruction
            + self.observations
            + self.recorded
            + self.depth_maps
            + self.dense_preparation
            + self.tsdf
            + self.dense_cloud
            + self.meshing_preparation
    }

    fn line(&self, label: &str) -> String {
        let mib = |bytes: usize| format!("{:.2}", bytes as f64 / (1024.0 * 1024.0));
        format!(
            "[RETAIN] {label:<34} total={} MiB | frames={} kp={} desc={} pairs={} tracks={} sfm={} recon={} obs={} recorded={} depthmaps={} densePrep={} tsdf={} cloud={} meshPrep={}",
            mib(self.total()),
            mib(self.frames),
            mib(self.keypoints),
            mib(self.descriptors),
            mib(self.pairwise),
            mib(self.tracks),
            mib(self.sfm),
            mib(self.reconstruction),
            mib(self.observations),
            mib(self.recorded),
            mib(self.depth_maps),
            mib(self.dense_preparation),
            mib(self.tsdf),
            mib(self.dense_cloud),
            mib(self.meshing_preparation),
        )
    }
}

fn retention(engine: &ReconstructionEngine) -> Retention {
    Retention {
        frames: engine.frames.iter().map(|frame| frame.image.data.capacity() + size_of::<AcceptedFrame>()).sum::<usize>()
            + engine.frame_source.frames.iter().map(|frame| frame.image.data.capacity()).sum::<usize>(),
        keypoints: nested_bytes(&engine.keypoints_per_frame),
        descriptors: nested_bytes(&engine.descriptors_per_frame),
        pairwise: engine.pairwise_matches.len() * size_of::<(usize, usize, Vec<remodeling_feature::Match>)>() + engine.pairwise_matches.iter().map(|(_, _, matches)| vec_bytes(matches.as_slice())).sum::<usize>(),
        tracks: engine.tracks.as_ref().map_or(0, |tracks| nested_bytes(&tracks.tracks)),
        sfm: engine.sfm.as_ref().map_or(0, remodeling_sfm::IncrementalSfm::retained_bytes),
        reconstruction: engine.reconstruction.as_ref().map_or(0, |reconstruction| vec_bytes(reconstruction.cameras.as_slice()) + vec_bytes(reconstruction.points.as_slice()) + vec_bytes(reconstruction.point_track_ids.as_slice())),
        observations: vec_bytes(engine.observations.as_slice()),
        recorded: engine.recorded.as_ref().map_or(0, |recorded| vec_bytes(recorded.as_slice())),
        depth_maps: engine.depth_maps.iter().map(|map| vec_bytes(map.depth.as_slice()) + vec_bytes(map.normal.as_slice()) + vec_bytes(map.confidence.as_slice())).sum(),
        dense_preparation: engine.dense_preparation.as_ref().map_or(0, |preparation| {
            vec_bytes(preparation.reference_gray.data.as_slice())
                + preparation.source_grays.iter().map(|(gray, _, _)| vec_bytes(gray.data.as_slice())).sum::<usize>()
                + preparation.patch_match.as_ref().map_or(0, remodeling_dense::PatchMatchPreparation::retained_bytes)
        }),
        tsdf: engine.tsdf.as_ref().map_or(0, remodeling_dense::TsdfVolume::retained_bytes),
        dense_cloud: engine.dense_cloud.as_ref().map_or(0, |cloud| vec_bytes(cloud.positions.as_slice()) + vec_bytes(cloud.normals.as_slice()) + vec_bytes(cloud.confidence.as_slice())),
        meshing_preparation: engine.meshing_preparation.as_ref().map_or(0, |preparation| preparation.views.iter().map(|view| vec_bytes(view.image.data.as_slice())).sum::<usize>() + preparation.active_view.as_ref().map_or(0, |view| vec_bytes(view.image.data.as_slice()))),
    }
}

/// 🛰️ The `synthetic-orbit` engine, frames pushed exactly as `🧵️reconstruction-session` pushes them.
fn synthetic_orbit_engine() -> ReconstructionEngine {
    let scene = <crate::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("the synthetic-orbit document parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    eprintln!("[RETAIN] params voxel={} truncation={} dense={:?} sources={} texture={}", params.tsdf_voxel_size, params.tsdf_truncation, params.dense, params.dense_source_views, params.texture_enabled);
    let mut engine = ReconstructionEngine::new(&params);
    for (index, (_, bytes)) in crate::examples::synthetic_orbit::FRAMES.iter().enumerate() {
        let image = crate::editor::remodeling::decode_still_image("image/png", bytes).expect("the fixture frame decodes");
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    engine
}

#[test]
#[ignore = "diagnostic memory probe: minutes of dense stereo, run explicitly"]
fn synthetic_orbit_peak_retention() {
    let mut engine = synthetic_orbit_engine();
    engine.observe();
    let mut stage = engine.stage();
    let mut peak = 0usize;
    let mut peak_label = String::new();
    let mut dense_slot = usize::MAX;
    let mut drained = Vec::new();
    eprintln!("{}", retention(&engine).line("pushed"));
    loop {
        let status = engine.advance(256);
        // 🔭️ The run job drains observations every continuation; mirror that so `recorded` does not
        // stand in for the job's own trace buffer.
        engine.drain_observations(&mut drained);
        drained.clear();
        let snapshot = retention(&engine);
        if snapshot.total() > peak {
            peak = snapshot.total();
            peak_label = format!("{:?}/{}", engine.stage(), engine.stage_cursor);
        }
        if engine.stage() != stage {
            eprintln!("{}", snapshot.line(&format!("enter {:?}", engine.stage())));
            stage = engine.stage();
            dense_slot = usize::MAX;
        }
        if matches!(stage, EngineStage::DenseStereo | EngineStage::FusingVolume) && engine.stage_cursor != dense_slot {
            dense_slot = engine.stage_cursor;
            eprintln!("{}", snapshot.line(&format!("{stage:?} slot {dense_slot}")));
            if let Some(volume) = engine.tsdf.as_ref() {
                eprintln!("[RETAIN] tsdf blocks={} bytes={}", volume.block_count(), volume.retained_bytes());
            }
        }
        if snapshot.tsdf > TSDF_REPORT_CEILING_BYTES {
            eprintln!("{}", snapshot.line("TSDF CEILING — probe stops"));
            break;
        }
        match status {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => {
                eprintln!("{}", retention(&engine).line("done"));
                break;
            }
            EngineStatus::Failed(message) => {
                eprintln!("{}", retention(&engine).line(&format!("failed: {message}")));
                break;
            }
        }
    }
    eprintln!("[RETAIN] PEAK {:.2} MiB at {peak_label}", peak as f64 / (1024.0 * 1024.0));
}
