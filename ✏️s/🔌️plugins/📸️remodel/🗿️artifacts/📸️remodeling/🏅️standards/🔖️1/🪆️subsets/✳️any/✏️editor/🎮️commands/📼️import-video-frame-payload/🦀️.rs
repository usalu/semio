//! 📥️ Remodeling play app commands — `import-video-frame-payload`: one tick of a host-decoded video import. Every import
//! is ONE streamed tool transaction (design §15) whose tool state lives in the importing window's transient
//! (`✏️editor/🫧️transient`): tick `0` starts it, the first accepted frame mints its stream and opens the edit (the
//! document shows the frames at once), `import-video-done` commits it as one edit and one history row, `import-abort` (or
//! the window closing) reverts it with zero trace, and a tick arriving after that is dropped. The transaction is minted
//! from the stream the import builds.

#[cfg(test)]
use crate::editor::remodeling::commands::import_frame_payload;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::blur_gate_admits;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::sharpness_score;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::BLUR_GATE_ROLLING_WINDOW;
#[cfg(test)]
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::payload_from_data_url;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::transient::RemodelingImport;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::transient::RemodelingWindowTransient;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::add_stream_frame;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_asset;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_stream::create_stream;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_schema::standards::v1::subsets::any::schema::mint_remodeling_id;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾️ImportTransaction
/// 🪪️ The tool every import transaction is stamped with: `<appId>#import`.
pub const REMODELING_IMPORT_TOOL_ID: &str = "s.remodel.remodeling@1/*#editor#import";

/// 🧾️ The ONE transaction an import building `stream_id` streams into, minted from that stream: every tick of the import,
/// its commit and its abort name the same ref.
pub fn import_transaction(stream_id: &str) -> protocol::TransactionRef {
    protocol::TransactionRef::mint(&protocol::ActorId(stream_id.into()), &protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, REMODELING_IMPORT_TOOL_ID)
}
//#endregion 🧾️ImportTransaction


#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-video-frame-payload")]
pub struct ImportVideoFramePayload {
    pub payload: String,
    pub name: String,
    pub index: u32,
    pub frame_index: u32,
    pub timestamp_ms: f64,
}

/// 🎞️ Host-decoded video frame tick through the importing window's tool state `window`: decodes the sampled JPEG, runs it
/// through the engine's one relative blur gate over the import's own rolling scores (kept in `window`, so a tick costs one decode) and
/// streams it into the import's open transaction; the emission and the window's next partition. Tick `0` while this
/// window's earlier import still streams is refused (`remodeling.import.open`): one window imports one stream at a time.
pub fn handle_in_window(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    let scene = doc.snapshot;
    let started = match payload.index {
        0 => {
            refuse_while_streaming(scene, window)?;
            Some(RemodelingImport::default())
        }
        _ => window.import.clone(),
    };
    let Some(mut import) = started else { return Ok((Emit::default(), window.clone())) };
    if import.stream_id.as_ref().is_some_and(|stream_id| !scene.streams.iter().any(|stream| stream.id == *stream_id)) {
        return Ok((Emit::default(), RemodelingWindowTransient::default()));
    }
    import.done = import.done.saturating_add(1);
    let accepted = payload_from_data_url(&payload.payload).and_then(|(_mime, bytes)| remodeling_image::decode_jpeg(&bytes).ok().map(|image| (bytes, image)));
    let Some((bytes, image)) = accepted else { return Ok((Emit::default(), RemodelingWindowTransient { import: Some(import) })) };
    let stream_id = import.stream_id.clone().unwrap_or_else(|| mint_remodeling_id(doc.operation_optional(), "stream"));
    if !blur_gate_admits(&mut import.rolling_scores, BLUR_GATE_ROLLING_WINDOW, sharpness_score(&image), scene.params.ingest.min_sharpness) {
        return Ok((Emit::default(), RemodelingWindowTransient { import: Some(import) }));
    }
    let asset_key = format!("{stream_id}-frame-{}", payload.frame_index);
    let asset = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(&bytes), width: image.width, height: image.height };
    let frame = FrameRef { index: payload.frame_index, timestamp_ms: payload.timestamp_ms, asset_id: asset_key.clone() };
    let mut mutations = vec![create_asset(asset_key, asset)];
    match scene.streams.iter().any(|stream| stream.id == stream_id) {
        true => mutations.push(add_stream_frame(stream_id.clone(), frame, MediaKind::Video)),
        false => mutations.push(create_stream(MediaStream { id: stream_id.clone(), name: payload.name.clone(), kind: MediaKind::Video, camera_id: None, sync_offset_ms: 0.0, fps_hint: 0.0, frames: vec![frame], source: None })),
    }
    import.stream_id = Some(stream_id.clone());
    Ok((Emit::stream_transaction(import_transaction(&stream_id), mutations), RemodelingWindowTransient { import: Some(import) }))
}

/// 🎞️ A video frame tick dispatched without a window: refused at the first tick (an import's tool state needs the
/// importing window), a later tick is dropped.
pub fn handle(payload: &ImportVideoFramePayload, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let resting = RemodelingWindowTransient::default();
    let (emit, next) = handle_in_window(payload, doc, &resting)?;
    match next == resting {
        true => Ok(emit),
        false => Err(import_window_required()),
    }
}

/// 🪟️ The refusal of a streamed import dispatched without a window: its tool state lives in the importing window.
pub fn import_window_required() -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("remodeling.import.window-required"), "A streamed import keeps its progress in the window that starts it; dispatch it from a window.")
}

/// 🚧️ Refuses a new import in a window whose earlier import still streams into an open transaction (its stream is in the
/// document): that import commits or aborts first.
pub(crate) fn refuse_while_streaming(scene: &RemodelingSnapshot, window: &RemodelingWindowTransient) -> Result<(), Fault> {
    match window.import.as_ref().and_then(|import| import.stream_id.as_ref()).is_some_and(|stream_id| scene.streams.iter().any(|stream| stream.id == *stream_id)) {
        true => Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.import.open"), "This window is still importing; finish or cancel that import first.")),
        false => Ok(()),
    }
}

//#region 🧪️Tests
//#endregion 🧪️Tests
