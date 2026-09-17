//! 🎬️ `setActiveExample` — loads one of this subset's committed example documents into the open
//! document. Before this row existed the two example leaves (`📚️examples/🎬️demo`,
//! `✏️editor/📚️examples/🎬️demo-session`) were mounted but unreachable: no command, no registry, no
//! manifest action. The example TEXT is the source of truth; the replay is a field-granular
//! `RemodelingMutation` set (this artifact has no whole-document-replace variant on purpose — see
//! `RemodelingMutation`'s own doc), so the whole load stays inside the retained bounded-first-step
//! envelope every other route uses.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::remodeling::decode_still_image;
use crate::editor::remodeling::examples::example_text;
use crate::mutations::{
    create_asset, create_camera_calibration, create_gcp, create_stream, delete_camera_calibration, delete_gcp, delete_stream, update_dense_params, update_feature_params, update_geo_params, update_ingest_params, update_match_params,
    update_mesh_params, update_motion_params, update_sfm_params,
};
use crate::op::RemodelingMutation;
use crate::{ImageAsset, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🎬️ Replaces the document's declared state with the named example's. An unknown id, or an example
/// whose committed text no longer parses, is a no-op rather than a fault: the picker is a navigation
/// affordance, not a destructive verb.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let Some(text) = example_text(&payload.example_id) else { return Ok(Emit::default()) };
    let Ok(next) = crate::snapshot::text::parse_dsl(text) else { return Ok(Emit::default()) };
    let mut mutations = example_media_operations(&payload.example_id, doc.snapshot);
    mutations.extend(replace_document_operations(doc.snapshot, &next));
    match mutations.is_empty() {
        true => Ok(Emit::default()),
        false => Ok(Emit::mutations(mutations)),
    }
}

/// 🎞️ The media an example's DSL declares but cannot carry. `RemodelingSnapshot::assets` holds pixel
/// bytes, and a `.dsl.semio` document only names the asset ids its frame table points at, so a
/// selected example whose frames are committed PNGs has to mint one `create-asset` per frame the same
/// way a file-picker drop does. Only `📚️examples/🛰️synthetic-orbit` ships frames today; the other two
/// examples declare no media and answer with an empty set. Assets already in the document are skipped,
/// so re-selecting the same example is not a stream of rejected duplicate creates.
fn example_media_operations(example_id: &str, current: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    use crate::examples::synthetic_orbit;
    if example_id != synthetic_orbit::ID {
        return Vec::new();
    }
    synthetic_orbit::FRAMES
        .iter()
        .filter(|(asset_id, _)| !current.assets.contains_key(*asset_id))
        .map(|(asset_id, bytes)| {
            let (width, height) = decode_still_image(synthetic_orbit::FRAME_MIME, bytes).map_or((0, 0), |image| (image.width, image.height));
            create_asset((*asset_id).to_string(), ImageAsset { mime: synthetic_orbit::FRAME_MIME.to_string(), data: base64_codec::base64_standard_encode(bytes), width, height })
        })
        .collect()
}

/// 🔁️ The field-granular diff between the open document's declared state and the example's: a
/// collection entry is deleted when the example no longer carries its id or carries it changed, created
/// when it is missing or changed, and a parameter group is overwritten only when it differs. A document
/// that already equals the example (the boot document IS the boot example) answers an empty set, so the
/// shell's boot-time `setActiveExample` and a re-selection of the current example write no edit at all.
/// `results` and `assets` are deliberately NOT transplanted — results are engine-derived (a fresh run
/// rebuilds them) and assets are media bytes an example binds through `importFramePayload`, not document
/// state a DSL carries.
pub fn replace_document_operations(current: &RemodelingSnapshot, next: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let mut mutations = Vec::new();
    for gcp in current.gcps.iter().filter(|gcp| !next.gcps.contains(gcp)) {
        mutations.push(delete_gcp(gcp.id.clone()));
    }
    for stream in current.streams.iter().filter(|stream| !next.streams.contains(stream)) {
        mutations.push(delete_stream(stream.id.clone()));
    }
    for camera in current.calibration.cameras.iter().filter(|camera| !next.calibration.cameras.contains(camera)) {
        mutations.push(delete_camera_calibration(camera.id.clone()));
    }
    for camera in next.calibration.cameras.iter().filter(|camera| !current.calibration.cameras.contains(camera)) {
        mutations.push(create_camera_calibration(camera.clone()));
    }
    for stream in next.streams.iter().filter(|stream| !current.streams.contains(stream)) {
        mutations.push(create_stream(stream.clone()));
    }
    for gcp in next.gcps.iter().filter(|gcp| !current.gcps.contains(gcp)) {
        mutations.push(create_gcp(gcp.clone()));
    }
    let (current, next) = (&current.params, &next.params);
    if current.ingest != next.ingest {
        mutations.push(update_ingest_params(next.ingest.clone()));
    }
    if current.feature != next.feature {
        mutations.push(update_feature_params(next.feature.clone()));
    }
    if current.matching != next.matching {
        mutations.push(update_match_params(next.matching.clone()));
    }
    if current.sfm != next.sfm {
        mutations.push(update_sfm_params(next.sfm.clone()));
    }
    if current.dense != next.dense {
        mutations.push(update_dense_params(next.dense.clone()));
    }
    if current.mesh != next.mesh {
        mutations.push(update_mesh_params(next.mesh.clone()));
    }
    if current.motion != next.motion {
        mutations.push(update_motion_params(next.motion.clone()));
    }
    if current.geo != next.geo {
        mutations.push(update_geo_params(next.geo.clone()));
    }
    mutations
}

//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
