//! 📥️ Remodeling play app commands — `import-video-bytes-payload`: the in-process video decode (Tier-3 fallback, or a
//! single picked video), sampled and blur-gated by the import's one gate, committed as ONE import transaction.

use crate::editor::remodeling::commands::import_video_frame_payload::import_transaction;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::blur_gate_admits;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::sharpness_score;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::BLUR_GATE_ROLLING_WINDOW;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::describe_video_probe;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::video_codec_to_artifact;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::payload_from_data_url;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_asset;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_stream::create_stream;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_schema::standards::v1::subsets::any::schema::mint_remodeling_id;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::VideoSource;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎞️ One in-process sampled video frame, JPEG-encoded, with its sample index and timestamp.
pub struct SampledFrame {
    pub asset: ImageAsset,
    pub index: u32,
    pub timestamp_ms: f64,
}

/// 🎞️ Probes, demuxes, decodes and blur-gates a video container fully in-process under the scene's ingest parameters:
/// the sampled frames and the probed provenance, or the notice naming why the container could not be read.
pub fn sample_video(bytes: &[u8], scene: &RemodelingSnapshot) -> Result<(Vec<SampledFrame>, VideoSource), String> {
    let probe = semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::probe(bytes).map_err(|error| format!("Could not probe video: {error}"))?;
    let (codec, width, height, duration_ms, container) = describe_video_probe(&probe);
    let ingest = &scene.params.ingest;
    let opts = semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
    let iter = semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::extract_frames(bytes, &opts).map_err(|error| format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}"))?;
    let mut rolling = Vec::with_capacity(BLUR_GATE_ROLLING_WINDOW);
    let mut frames = Vec::new();
    for extracted in iter {
        let Ok(extracted) = extracted else { continue };
        if !blur_gate_admits(&mut rolling, BLUR_GATE_ROLLING_WINDOW, sharpness_score(&extracted.image), ingest.min_sharpness) {
            continue;
        }
        let jpeg = remodeling_image::encode_jpeg(&extracted.image, 90);
        frames.push(SampledFrame { asset: ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(&jpeg), width: extracted.image.width, height: extracted.image.height }, index: extracted.index, timestamp_ms: extracted.timestamp_ms });
    }
    Ok((frames, VideoSource { name: String::new(), container: container.into(), codec: video_codec_to_artifact(codec), duration_ms, frame_count: 0, width, height }))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-video-bytes-payload")]
pub struct ImportVideoBytesPayload {
    pub payload: String,
    pub name: String,
}

/// 🎞️ Tier-3 fallback (or a single picked video in `ImportFramePayload`): the host couldn't decode the video, so it hands
/// over the raw container bytes and [`sample_video`] extracts the frames fully in-process. The whole video lands in this
/// ONE call as one new video stream and commits as ONE import transaction (one edit, one history row, one undo step). An
/// unreadable container surfaces as a `Notify` naming it.
pub fn handle(payload: &ImportVideoBytesPayload, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let Some((_mime, bytes)) = payload_from_data_url(&payload.payload) else { return Ok(Emit::default()) };
    let (sampled, source) = match sample_video(&bytes, doc.snapshot) {
        Ok(sampled) => sampled,
        Err(message) => return Ok(Emit::effect(Effect::Notify { message })),
    };
    let stream_id = mint_remodeling_id(doc.operation_optional(), "stream");
    let mut mutations = Vec::with_capacity(sampled.len() + 1);
    let mut frames = Vec::with_capacity(sampled.len());
    for frame in sampled {
        let asset_key = format!("{stream_id}-frame-{}", frame.index);
        mutations.push(create_asset(asset_key.clone(), frame.asset));
        frames.push(FrameRef { index: frame.index, timestamp_ms: frame.timestamp_ms, asset_id: asset_key });
    }
    let transaction = import_transaction(&stream_id);
    mutations.push(create_stream(MediaStream { id: stream_id, name: payload.name.clone(), kind: MediaKind::Video, camera_id: None, sync_offset_ms: 0.0, fps_hint: 0.0, frames, source: Some(source) }));
    Ok(Emit::commit_transaction(transaction, mutations))
}
