//! 🧬️ Raster artifact — closed semantic mutation dispatch enum (constitutional: op). Derived from
//! `RasterSnapshot`'s recursive layer-tree shape per `📓️derivation-rules.md`: the five old
//! option-bag/whole-tree variants (`AddLayer`, `RemoveLayer`, `PatchLayer`, `MoveLayer`, and the old
//! whole-document-replace variant) are gone, replaced by ten real verbs (`create-layer`, `delete-layer`,
//! `reorder-layers`, `rename-layer`, `change-layer-visible`, `change-layer-opacity`,
//! `change-layer-blend-mode`, `move-layer`, `resize-layer`, `change-layer-adjustment-kind`) plus two
//! justified additions for the `assets` id-keyed root collection (`add-layer-asset`,
//! `remove-layer-asset` — see that leaf's docstring). The old whole-document-replace variant dies
//! with NO replacement: whole-document replace goes through `store::ArtifactStore::reset`, entirely
//! outside this enum.
//!
//! All twelve triads are mounted directly as `mutations`-sibling modules in `🦀️.rs`, each with
//! its own unique emoji-prefixed directory — no inline `#[path = "."]` self-wiring.

use crate::diff::RasterDiff;
use crate::RasterSnapshot;

//#region 🔖️Leaves
use super::add_layer_asset;
use super::change_layer_adjustment_kind;
use super::change_layer_blend_mode;
use super::change_layer_opacity;
use super::change_layer_visible;
use super::create_layer;
use super::delete_layer;
use super::move_layer;
use super::remove_layer_asset;
use super::rename_layer;
use super::reorder_layers;
use super::resize_layer;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the raster document, derived per
/// `📓️derivation-rules.md` from `RasterLayerNode`'s recursive tree shape and the `assets` root
/// collection.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RasterSnapshot, diff = RasterDiff, schema = "raster.raster")]
pub enum RasterMutation {
    CreateLayer(create_layer::CreateLayer),
    DeleteLayer(delete_layer::DeleteLayer),
    ReorderLayers(reorder_layers::ReorderLayers),
    RenameLayer(rename_layer::RenameLayer),
    ChangeLayerVisible(change_layer_visible::ChangeLayerVisible),
    ChangeLayerOpacity(change_layer_opacity::ChangeLayerOpacity),
    ChangeLayerBlendMode(change_layer_blend_mode::ChangeLayerBlendMode),
    MoveLayer(move_layer::MoveLayer),
    ResizeLayer(resize_layer::ResizeLayer),
    ChangeLayerAdjustmentKind(change_layer_adjustment_kind::ChangeLayerAdjustmentKind),
    AddLayerAsset(add_layer_asset::AddLayerAsset),
    RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset),
}

/// ⚡️ Convenience wrapper kept for existing in-plugin callers (`RasterBuilderConstruction::mutate`,
/// the WASM bridge) — `diff().apply()` in one call, now delegating to the derive's real
/// `Mutation`/`MutationDiff` impls instead of a hand-written match.
pub fn apply_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> protocol::MutationApplyResult<RasterSnapshot> {
    protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, snapshot).diff(), snapshot)
}

/// ⚡️ Convenience wrapper mirroring `apply_raster_mutation` — forwards to the derive's real
/// `Mutation::inverse`.
pub fn inverse_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Vec<RasterMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

pub type RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>;
pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(RasterSnapshot, RasterMutation), String> {
    let snapshot: RasterSnapshot = dsl::os_pack::json::from_json_str(snapshot_json).map_err(|error| format!("the committed raster snapshot JSON does not decode: {error}"))?;
    let mutation: RasterMutation = dsl::os_pack::json::from_json_str(mutation_json).map_err(|error| format!("the committed raster mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_step(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Result<(RasterSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <RasterMutation as Mutation<RasterSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_render(snapshot: &RasterSnapshot, messages: Vec<String>) -> String {
    let value = dsl::os_pack::json::object([
        ("snapshot".to_string(), dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(snapshot))),
        ("messages".to_string(), dsl::os_pack::json::Value::Array(messages.into_iter().map(dsl::os_pack::json::Value::from).collect())),
    ]);
    dsl::os_pack::json::to_string(&value)
}

/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `dsl`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_raster_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    Ok(bridge_render(&applied, messages))
}

/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of what `🖨️mutate-raster-1` compares against its
/// Python second implementation. The inverse is computed against the PRE-mutation document, which is
/// the only state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn undo_raster_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <RasterMutation as Mutation<RasterSnapshot>>::inverse(&mutation, &base) {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    Ok(bridge_render(&current, messages))
}

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_raster_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <RasterSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed raster example does not parse: {error:?}"))?;
    let printed = <RasterSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RasterSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted raster document does not parse: {error:?}"))?;
    let value = dsl::os_pack::json::object([
        ("printed".to_string(), dsl::os_pack::json::Value::from(printed)),
        ("snapshot".to_string(), dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&reparsed))),
    ]);
    Ok(dsl::os_pack::json::to_string(&value))
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RasterMutation` variant, in declaration order — the vocabulary
/// the `raster-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `🖨️mutate-raster-1` exhaustive case measures itself against. Ten address the recursive layer tree;
/// the last two join that tree to the document's root `assets` pool by id.
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against the enum,
/// since the framework never parses Rust.
pub const KINDS: &[&str] = &[
    "create-layer",
    "delete-layer",
    "reorder-layers",
    "rename-layer",
    "change-layer-visible",
    "change-layer-opacity",
    "change-layer-blend-mode",
    "move-layer",
    "resize-layer",
    "change-layer-adjustment-kind",
    "add-layer-asset",
    "remove-layer-asset",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog
