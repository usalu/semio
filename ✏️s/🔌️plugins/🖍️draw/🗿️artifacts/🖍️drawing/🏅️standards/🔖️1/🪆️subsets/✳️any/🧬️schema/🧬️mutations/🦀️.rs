//! 🧬️ Drawing artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DrawingSnapshot>`
//! and `impl protocol::SemanticMutation<DrawingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

use crate::artifacts::drawing::schema::{find_drawing_layer, hex_to_rgba, layer_base};
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = DrawingSnapshot, diff = crate::artifacts::drawing::diff::DrawingDiff, schema = "drawing.drawing")]
pub enum DrawingMutation {
    SetLayerVisible(SetLayerVisible),
    SetLayerLocked(SetLayerLocked),
    SetLayerOpacity(SetLayerOpacity),
    SetLayerBlendMode(SetLayerBlendMode),
    RenameLayer(RenameLayer),
    UpdateLayerTransform(UpdateLayerTransform),
    ReplaceLayerFill(ReplaceLayerFill),
    ReplaceLayerStroke(ReplaceLayerStroke),
    SetLayerBooleanOperation(SetLayerBooleanOperation),
    UpdateLayerTraceParams(UpdateLayerTraceParams),
    CreateLayer(CreateLayer),
    DuplicateLayer(DuplicateLayer),
    DeleteLayer(DeleteLayer),
    ReorderLayer(ReorderLayer),
}
//#endregion 🔖️Mutations

//#region 🔖️FieldPatch
/// 🎛️ Generic single-field layer editor bridge (properties panel / bulk patch commands) — maps a
/// wire `field` name + JSON `value` onto the one semantic mutation that owns that field. Returns
/// `None` for an unknown field or a field that doesn't apply to `layer`'s kind.
pub fn drawing_op_for_layer_field(doc: &DrawingSnapshot, layer_id: &str, field: &str, value: &dsl::DslValue) -> Option<DrawingMutation> {
    let layer = find_drawing_layer(doc, layer_id)?;
    let operation = match field {
        "name" => rename_layer(layer_id.into(), value.as_str().unwrap_or("").into()),
        "opacity" => set_layer_opacity(layer_id.into(), value.as_f64().unwrap_or(1.0)),
        "visible" => set_layer_visible(layer_id.into(), value.as_bool().unwrap_or(true)),
        "locked" => set_layer_locked(layer_id.into(), value.as_bool().unwrap_or(false)),
        "blendMode" => set_layer_blend_mode(layer_id.into(), value.as_str().unwrap_or("normal").into()),
        "booleanOperation" => set_layer_boolean_operation(layer_id.into(), value.as_str().unwrap_or("union").into()),
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            update_layer_transform(layer_id.into(), transform)
        }
        "fillColor" => {
            let alpha = layer_base(layer).attributes.fill.as_ref().map_or(1.0, |fill| match fill {
                FillStyle::Solid { color } => color[3],
                FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
            });
            replace_layer_fill(layer_id.into(), Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }))
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: "butt".into(), join: "miter".into(), dash: None });
            replace_layer_stroke(layer_id.into(), Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }))
        }
        "traceThreshold" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            update_layer_trace_params(layer_id.into(), params)
        }
        "traceSimplify" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            update_layer_trace_params(layer_id.into(), params)
        }
        _ => return None,
    };
    Some(operation)
}

/// 🩹 Applies one field patch directly to `doc` — used by callers that don't need the mutation
/// value itself (`drawing_op_for_layer_field` is the undoable/command-facing entry point).
pub fn patch_layer_field(doc: &DrawingSnapshot, layer_id: &str, field: &str, value: &dsl::DslValue) -> protocol::MutationApplyResult<DrawingSnapshot> {
    use protocol::{Mutation, MutationDiff};
    match drawing_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => operation.diff(doc).diff().apply(doc).map_err(|error| error.under(["layers", layer_id])),
        None => Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "layer field cannot be patched").at(["layers", layer_id, field])),
    }
}
//#endregion 🔖️FieldPatch

// 🪆️ These fourteen leaves now live under their own semantic subset (`structure`/`style`/
// `transform`/`metadata`, see `../../../🔣️.json`), not as siblings of this catalog any more — this
// catalog is the one thing every subset composes back through, so its `use`s are fully qualified.
pub use crate::artifacts::drawing::standards::v1::subsets::structure::schema::mutations::create_layer::mutation::{create_layer, CreateLayer};
pub use crate::artifacts::drawing::standards::v1::subsets::structure::schema::mutations::delete_layer::mutation::{delete_layer, DeleteLayer};
pub use crate::artifacts::drawing::standards::v1::subsets::structure::schema::mutations::duplicate_layer::mutation::{duplicate_layer, DuplicateLayer};
pub use crate::artifacts::drawing::standards::v1::subsets::metadata::schema::mutations::rename_layer::mutation::{rename_layer, RenameLayer};
pub use crate::artifacts::drawing::standards::v1::subsets::structure::schema::mutations::reorder_layer::mutation::{reorder_layer, ReorderLayer};
pub use crate::artifacts::drawing::standards::v1::subsets::style::schema::mutations::replace_layer_fill::mutation::{replace_layer_fill, ReplaceLayerFill};
pub use crate::artifacts::drawing::standards::v1::subsets::style::schema::mutations::replace_layer_stroke::mutation::{replace_layer_stroke, ReplaceLayerStroke};
pub use crate::artifacts::drawing::standards::v1::subsets::style::schema::mutations::set_layer_blend_mode::mutation::{set_layer_blend_mode, SetLayerBlendMode};
pub use crate::artifacts::drawing::standards::v1::subsets::transform::schema::mutations::set_layer_boolean_operation::mutation::{set_layer_boolean_operation, SetLayerBooleanOperation};
pub use crate::artifacts::drawing::standards::v1::subsets::metadata::schema::mutations::set_layer_locked::mutation::{set_layer_locked, SetLayerLocked};
pub use crate::artifacts::drawing::standards::v1::subsets::style::schema::mutations::set_layer_opacity::mutation::{set_layer_opacity, SetLayerOpacity};
pub use crate::artifacts::drawing::standards::v1::subsets::metadata::schema::mutations::set_layer_visible::mutation::{set_layer_visible, SetLayerVisible};
pub use crate::artifacts::drawing::standards::v1::subsets::transform::schema::mutations::update_layer_trace_params::mutation::{update_layer_trace_params, UpdateLayerTraceParams};
pub use crate::artifacts::drawing::standards::v1::subsets::transform::schema::mutations::update_layer_transform::mutation::{update_layer_transform, UpdateLayerTransform};

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `DrawingDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_drawing_mutation(snapshot: &mut DrawingSnapshot, mutation: &DrawingMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_drawing_mutation(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Vec<DrawingMutation> {
    <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::drawing::schema::{create_drawing_path_layer, create_drawing_shape_layer_rect, default_drawing_document};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_policy_matrix};
    use protocol::{Mutation, MutationDiff, SemanticMutation};

    fn base_document() -> DrawingSnapshot {
        let mut doc = default_drawing_document("mutations-test", None);
        doc.layers.push(create_drawing_shape_layer_rect("Rect"));
        doc
    }

    #[semio_framework_async_macros::async_test]
    async fn set_layer_visible_inverse_law() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let mutation = set_layer_visible(layer_id, false);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_layer_inverse_law() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let mutation = rename_layer(layer_id, "Renamed".into());
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_layer_inverse_law() {
        let base = base_document();
        let mutation = create_layer(None, None, create_drawing_path_layer("New", Vec::new()));
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_layer_inverse_law() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let mutation = delete_layer(layer_id);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_layer_inverse_law() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let mutation = duplicate_layer(layer_id);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_layer_inverse_law() {
        let mut base = base_document();
        base.layers.push(create_drawing_path_layer("Second", Vec::new()));
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let mutation = reorder_layer(layer_id, None, 1);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_layer_opacity_diff_absorb_law() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        let d1 = set_layer_opacity(layer_id.clone(), 0.5).diff(&base).diff().clone();
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = set_layer_opacity(layer_id, 0.25).diff(&mid).diff().clone();
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    //#region 🧪️OutcomeLaws
    /// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family: `assert_missing_target_is_error`/
    /// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down (delete,
    /// rename, set, create).
    #[semio_framework_async_macros::async_test]
    async fn delete_missing_layer_is_a_target_missing_error() {
        let base = base_document();
        assert_missing_target_is_error(&base, &delete_layer("does-not-exist".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_missing_layer_is_a_target_missing_error() {
        let base = base_document();
        assert_missing_target_is_error(&base, &rename_layer("does-not-exist".into(), "New Name".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_layer_opacity_missing_layer_is_a_target_missing_error() {
        let base = base_document();
        assert_missing_target_is_error(&base, &set_layer_opacity("does-not-exist".into(), 0.5));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_layer_duplicate_id_never_applies() {
        let base = base_document();
        // Re-creating the exact existing node collides on id for real (ids are content-addressed).
        let duplicate = create_layer(None, None, base.layers[0].clone());
        assert_fatal_never_applies(&duplicate.diff(&base));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_layer_outcome_obeys_the_policy_matrix() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        assert_outcome_policy_matrix(&base, &delete_layer(layer_id));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_layer_outcome_obeys_the_policy_matrix() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        assert_outcome_policy_matrix(&base, &rename_layer(layer_id, "Renamed".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_layer_opacity_outcome_obeys_the_policy_matrix() {
        let base = base_document();
        let layer_id = crate::artifacts::drawing::schema::layer_id(&base.layers[0]).to_string();
        assert_outcome_policy_matrix(&base, &set_layer_opacity(layer_id, 0.5));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_layer_outcome_obeys_the_policy_matrix() {
        let base = base_document();
        assert_outcome_policy_matrix(&base, &create_layer(None, None, create_drawing_path_layer("New", Vec::new())));
    }
    //#endregion 🧪️OutcomeLaws

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_drawing_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in DrawingMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
            assert_eq!(kind.entity, "layer");
        }
        assert_eq!(DrawingMutation::kinds().len(), 14);
    }
}
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(DrawingSnapshot, DrawingMutation), String> {
    let snapshot: DrawingSnapshot = dsl::json::from_json_str(snapshot_json).map_err(|error| format!("the committed drawing snapshot JSON does not decode: {error}"))?;
    let mutation: DrawingMutation = dsl::json::from_json_str(mutation_json).map_err(|error| format!("the committed drawing mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_step(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<(DrawingSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <DrawingMutation as Mutation<DrawingSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_render(snapshot: &DrawingSnapshot, messages: Vec<String>) -> Result<String, String> {
    let report = dsl::DslValue::object([("snapshot".to_string(), dsl::ToValue::to_value(snapshot)), ("messages".to_string(), dsl::ToValue::to_value(&messages))]);
    Ok(dsl::json::to_json_string(&report))
}

/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `dsl`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_drawing_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    bridge_render(&applied, messages)
}

/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of the evidence the `drawing-mutation-semantics` no-oracle
/// decision rests on. The inverse is computed against the PRE-mutation document, which is the only
/// state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn undo_drawing_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <DrawingMutation as Mutation<DrawingSnapshot>>::inverse(&mutation, &base) {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    bridge_render(&current, messages)
}

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_drawing_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed drawing example does not parse: {error:?}"))?;
    let printed = <DrawingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted drawing document does not parse: {error:?}"))?;
    let report = dsl::DslValue::object([("printed".to_string(), dsl::ToValue::to_value(&printed)), ("snapshot".to_string(), dsl::ToValue::to_value(&parsed)), ("reparsed".to_string(), dsl::ToValue::to_value(&reparsed))]);
    Ok(dsl::json::to_json_string(&report))
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `DrawingMutation` variant, in declaration order — the vocabulary
/// the `drawing-1-any` catalog (`../../🔣️oracle.json`) declares and the `mutate-drawing-1`
/// exhaustive case measures itself against. Ten of the fourteen address ONE layer of the recursive
/// tree by id, `create-layer`/`duplicate-layer`/`reorder-layer` address a parent plus an index, and
/// `update-layer-trace-params` exists only for the trace node kind. `kinds_match_the_enum_and_the_
/// catalog` below is what keeps this list honest against the enum, since the framework never parses
/// Rust.
pub const KINDS: &[&str] = &[
    "set-layer-visible",
    "set-layer-locked",
    "set-layer-opacity",
    "set-layer-blend-mode",
    "rename-layer",
    "update-layer-transform",
    "replace-layer-fill",
    "replace-layer-stroke",
    "set-layer-boolean-operation",
    "update-layer-trace-params",
    "create-layer",
    "duplicate-layer",
    "delete-layer",
    "reorder-layer",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog_tests {
    use super::*;
    use protocol::SemanticMutation;

    /// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
    /// catalog — the framework reads the manifest and never parses Rust, so this test is the only
    /// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = DrawingMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared DrawingMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed drawing-1-any catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
