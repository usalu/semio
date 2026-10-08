//! 🎬️ `setActiveExample` — loads one of this subset's committed example documents as the open document. The example TEXT is
//! the source of truth; the load is the artifact's reset-document effect (frames travel as load content), never mutation
//! rows: an example switch is not an edit and leaves no history row.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::remodeling::decode_still_image;
use crate::editor::remodeling::examples::example_text;
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use crate::{durable_remodeling_asset, store_remodeling_asset, ImageAsset, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🎬️ Loads the named example as the open document. An unknown id, or an example whose committed text no longer parses, is a
/// no-op rather than a fault: the picker is a navigation affordance. A document that already equals the example (the boot
/// document IS the boot example) answers an empty emit, so the shell's boot-time `setActiveExample` loads nothing.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let Some(text) = example_text(&payload.example_id) else { return Ok(Emit::default()) };
    let Ok(mut next) = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text) else { return Ok(Emit::default()) };
    example_media_content(&payload.example_id, &mut next);
    match &next == doc.snapshot {
        true => Ok(Emit::default()),
        false => Ok(Emit { effects: vec![crate::editor::remodeling::reset_document_effect(&next)], ..Default::default() }),
    }
}

/// 🎞️ The media an example's DSL declares but cannot carry: `RemodelingSnapshot::assets` names pixel content a `.dsl.semio` document
/// only references by asset id, so a selected example whose frames are committed PNGs loads each frame as asset-handle and durable
/// content, the same content a file-picker drop mints. Only `📚️examples/🛰️synthetic-orbit` ships frames today.
fn example_media_content(example_id: &str, next: &mut RemodelingSnapshot) {
    use crate::examples::synthetic_orbit;
    if example_id != synthetic_orbit::ID {
        return;
    }
    for (asset_id, bytes) in synthetic_orbit::FRAMES {
        let (width, height) = decode_still_image(synthetic_orbit::FRAME_MIME, bytes).map_or((0, 0), |image| (image.width, image.height));
        let asset = ImageAsset { mime: synthetic_orbit::FRAME_MIME.to_string(), data: base64_codec::base64_standard_encode(bytes), width, height };
        let Some(artifact) = durable_remodeling_asset(&asset) else { continue };
        let handle = store_remodeling_asset(asset_id, &asset);
        next.durable_artifacts.insert(handle.child_id.clone(), artifact);
        next.assets.insert(asset_id.to_string(), handle);
    }
}

//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
