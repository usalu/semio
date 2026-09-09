//! 🧬️ Puzzle 5d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle5dSnapshot>` and `impl protocol::SemanticMutation<Puzzle5dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's `Puzzle5dPlaySnapshot`
//! newtype (`🔖️PlaySnapshot`) live here too, same shape as `puzzle2d`'s: the bridge round-trips
//! through the typed `Puzzle5dSnapshot` instead of hand-splicing JSON per mutation kind.

use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-5d document mutation vocabulary: id-keyed part create-delete plus per-2d/
/// per-3d-projection field edits, grip membership, a grip-to-grip fastener connect/disconnect
/// relationship, and document-level edits (label rename, domain/description change,
/// kind-compatibility connect/disconnect, kind-catalog replace). There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Puzzle5dSnapshot, diff = Puzzle5dDiff, schema = "puzzle.puzzle5d")]
pub enum Puzzle5dMutation {
    CreatePart(CreatePart),
    DeletePart(DeletePart),
    MovePart2d(MovePart2d),
    ReplacePart2dGeometry(ReplacePart2dGeometry),
    EditPart2dText(EditPart2dText),
    ChangePart2dIcon(ChangePart2dIcon),
    ChangePart2dHidden(ChangePart2dHidden),
    ChangePart2dLocked(ChangePart2dLocked),
    MovePart3d(MovePart3d),
    RotatePart3d(RotatePart3d),
    ScalePart3d(ScalePart3d),
    ChangePart3dMesh(ChangePart3dMesh),
    EditPart3dLabel(EditPart3dLabel),
    ChangePartKind(ChangePartKind),
    ChangePartAnchor(ChangePartAnchor),
    AddPartGrip(AddPartGrip),
    RemovePartGrip(RemovePartGrip),
    ReplacePartGrip(ReplacePartGrip),
    ConnectGrips(ConnectGrips),
    DisconnectGrips(DisconnectGrips),
    ReplaceFastenerGeometry(ReplaceFastenerGeometry),
    ChangeFastenerKind(ChangeFastenerKind),
    RenamePuzzle5d(RenamePuzzle5d),
    ChangeDomain(ChangeDomain),
    ChangeDescription(ChangeDescription),
    ConnectKindCompatibility(ConnectKindCompatibility),
    DisconnectKindCompatibility(DisconnectKindCompatibility),
    ReplaceKindCatalogs(ReplaceKindCatalogs),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle5dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-5d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🖐️mutate-puzzle-5d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-part",
    "delete-part",
    "move-part2d",
    "replace-part2d-geometry",
    "edit-part2d-text",
    "change-part2d-icon",
    "change-part2d-hidden",
    "change-part2d-locked",
    "move-part3d",
    "rotate-part3d",
    "scale-part3d",
    "change-part3d-mesh",
    "edit-part3d-label",
    "change-part-kind",
    "change-part-anchor",
    "add-part-grip",
    "remove-part-grip",
    "replace-part-grip",
    "connect-grips",
    "disconnect-grips",
    "replace-fastener-geometry",
    "change-fastener-kind",
    "rename-puzzle5d",
    "change-domain",
    "change-description",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use super::add_part_grip::{add_part_grip, AddPartGrip};
pub use super::change_description::{change_description, ChangeDescription};
pub use super::change_domain::{change_domain, ChangeDomain};
pub use super::change_fastener_kind::{change_fastener_kind, ChangeFastenerKind};
pub use super::change_part_2d_hidden::{change_part_2d_hidden, ChangePart2dHidden};
pub use super::change_part_2d_icon::{change_part_2d_icon, ChangePart2dIcon};
pub use super::change_part_2d_locked::{change_part_2d_locked, ChangePart2dLocked};
pub use super::change_part_3d_mesh::{change_part_3d_mesh, ChangePart3dMesh};
pub use super::change_part_anchor::{change_part_anchor, ChangePartAnchor};
pub use super::change_part_kind::{change_part_kind, ChangePartKind};
pub use super::connect_grips::{connect_grips, ConnectGrips};
pub use super::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::create_part::{create_part, CreatePart};
pub use super::delete_part::{delete_part, DeletePart};
pub use super::disconnect_grips::{disconnect_grips, DisconnectGrips};
pub use super::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::edit_part_2d_text::{edit_part_2d_text, EditPart2dText};
pub use super::edit_part_3d_label::{edit_part_3d_label, EditPart3dLabel};
pub use super::move_part_2d::{move_part_2d, MovePart2d};
pub use super::move_part_3d::{move_part_3d, MovePart3d};
pub use super::remove_part_grip::{remove_part_grip, RemovePartGrip};
pub use super::rename_puzzle5d::{rename_puzzle5d, RenamePuzzle5d};
pub use super::replace_fastener_geometry::{replace_fastener_geometry, ReplaceFastenerGeometry};
pub use super::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_part_2d_geometry::{replace_part_2d_geometry, ReplacePart2dGeometry};
pub use super::replace_part_grip::{replace_part_grip, ReplacePartGrip};
pub use super::rotate_part_3d::{rotate_part_3d, RotatePart3d};
pub use super::scale_part_3d::{scale_part_3d, ScalePart3d};

//#region 🔖️SnapshotDelta
/// 🔀️ Diffs two typed snapshots into a minimal semantic mutation set — the single source of truth
/// both the VCS layer and the `serde_json::Value` scene bridge below replay through.
pub fn puzzle5d_snapshot_mutations(before: &Puzzle5dSnapshot, after: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let mut mutations = Vec::new();
    for part in &before.parts {
        if !after.parts.iter().any(|entry| entry.id == part.id) {
            mutations.push(delete_part(part.id.clone()));
        }
    }
    for part in &after.parts {
        match before.parts.iter().find(|entry| entry.id == part.id) {
            None => mutations.push(create_part(part.clone(), None)),
            Some(prior) => {
                if prior.part_2d.x != part.part_2d.x || prior.part_2d.y != part.part_2d.y {
                    mutations.push(move_part_2d(part.id.clone(), part.part_2d.x, part.part_2d.y));
                }
                if prior.part_2d.shape != part.part_2d.shape || prior.part_2d.radius != part.part_2d.radius || prior.part_2d.width != part.part_2d.width || prior.part_2d.height != part.part_2d.height {
                    mutations.push(replace_part_2d_geometry(part.id.clone(), part.part_2d.shape.clone(), part.part_2d.radius, part.part_2d.width, part.part_2d.height));
                }
                if prior.part_2d.text != part.part_2d.text {
                    mutations.push(edit_part_2d_text(part.id.clone(), part.part_2d.text.clone()));
                }
                if prior.part_2d.icon_kind != part.part_2d.icon_kind {
                    mutations.push(change_part_2d_icon(part.id.clone(), part.part_2d.icon_kind.clone()));
                }
                if prior.part_2d.hidden != part.part_2d.hidden {
                    mutations.push(change_part_2d_hidden(part.id.clone(), part.part_2d.hidden));
                }
                if prior.part_2d.locked != part.part_2d.locked {
                    mutations.push(change_part_2d_locked(part.id.clone(), part.part_2d.locked));
                }
                if prior.part_3d.origin != part.part_3d.origin {
                    mutations.push(move_part_3d(part.id.clone(), part.part_3d.origin));
                }
                if prior.part_3d.orientation != part.part_3d.orientation {
                    mutations.push(rotate_part_3d(part.id.clone(), part.part_3d.orientation));
                }
                if prior.part_3d.scale != part.part_3d.scale {
                    mutations.push(scale_part_3d(part.id.clone(), part.part_3d.scale));
                }
                if prior.part_3d.mesh_url != part.part_3d.mesh_url {
                    mutations.push(change_part_3d_mesh(part.id.clone(), part.part_3d.mesh_url.clone()));
                }
                if prior.part_3d.label != part.part_3d.label {
                    mutations.push(edit_part_3d_label(part.id.clone(), part.part_3d.label.clone()));
                }
                if prior.part_kind != part.part_kind {
                    mutations.push(change_part_kind(part.id.clone(), part.part_kind.clone()));
                }
                if prior.anchor != part.anchor {
                    mutations.push(change_part_anchor(part.id.clone(), part.anchor));
                }
                for grip in &prior.grips {
                    if !part.grips.iter().any(|entry| entry.id == grip.id) {
                        mutations.push(remove_part_grip(part.id.clone(), grip.id.clone()));
                    }
                }
                for grip in &part.grips {
                    match prior.grips.iter().find(|entry| entry.id == grip.id) {
                        None => mutations.push(add_part_grip(part.id.clone(), grip.clone(), None)),
                        Some(prior_grip) if prior_grip != grip => mutations.push(replace_part_grip(part.id.clone(), grip.id.clone(), grip.clone())),
                        Some(_) => {}
                    }
                }
            }
        }
    }
    for fastener in &before.fasteners {
        if !after.fasteners.iter().any(|entry| entry.id == fastener.id) {
            mutations.push(disconnect_grips(fastener.id.clone()));
        }
    }
    for fastener in &after.fasteners {
        match before.fasteners.iter().find(|entry| entry.id == fastener.id) {
            None => mutations.push(connect_grips(
                fastener.id.clone(),
                fastener.source.clone(),
                fastener.target.clone(),
                fastener.fastener_kind.clone(),
                fastener.gap,
                fastener.shift,
                fastener.rise,
                fastener.rotation,
                fastener.turn,
                fastener.tilt,
                fastener.x,
                fastener.y,
            )),
            Some(prior) if prior.source != fastener.source || prior.target != fastener.target => {
                mutations.push(disconnect_grips(fastener.id.clone()));
                mutations.push(connect_grips(
                    fastener.id.clone(),
                    fastener.source.clone(),
                    fastener.target.clone(),
                    fastener.fastener_kind.clone(),
                    fastener.gap,
                    fastener.shift,
                    fastener.rise,
                    fastener.rotation,
                    fastener.turn,
                    fastener.tilt,
                    fastener.x,
                    fastener.y,
                ));
            }
            Some(prior) => {
                if prior.gap != fastener.gap
                    || prior.shift != fastener.shift
                    || prior.rise != fastener.rise
                    || prior.rotation != fastener.rotation
                    || prior.turn != fastener.turn
                    || prior.tilt != fastener.tilt
                    || prior.x != fastener.x
                    || prior.y != fastener.y
                {
                    mutations.push(replace_fastener_geometry(ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: fastener.gap, new_shift: fastener.shift, new_rise: fastener.rise, new_rotation: fastener.rotation, new_turn: fastener.turn, new_tilt: fastener.tilt, new_x: fastener.x, new_y: fastener.y }));
                }
                if prior.fastener_kind != fastener.fastener_kind {
                    mutations.push(change_fastener_kind(fastener.id.clone(), fastener.fastener_kind.clone()));
                }
            }
        }
    }
    if before.label != after.label {
        mutations.push(rename_puzzle5d(after.label.clone()));
    }
    if before.domain != after.domain {
        mutations.push(change_domain(after.domain.clone()));
    }
    if before.meta.description != after.meta.description {
        mutations.push(change_description(after.meta.description.clone()));
    }
    for row in &before.kind_compatibility {
        if !after.kind_compatibility.iter().any(|entry| entry.source == row.source && entry.target == row.target) {
            mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
        }
    }
    for row in &after.kind_compatibility {
        match before.kind_compatibility.iter().find(|entry| entry.source == row.source && entry.target == row.target) {
            None => mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)),
            Some(prior) if prior != row => {
                mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity));
            }
            Some(_) => {}
        }
    }
    if before.kind_catalogs != after.kind_catalogs || before.kind_catalogs_extra != after.kind_catalogs_extra {
        mutations.push(replace_kind_catalogs(crate::kind_catalogs_of(&after.kind_catalogs, &after.kind_catalogs_extra)));
    }
    mutations
}
//#endregion 🔖️SnapshotDelta

/// ▶️ Applies `mutation` via its diff.
pub fn apply_puzzle5d_mutation(projection: &mut Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_puzzle5d_mutation(projection: &Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> Vec<Puzzle5dMutation> {
    mutation.inverse(projection)
}

//#region 🔖️ValueBridge
// 🌉️ The play app's scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture. Bridging `Puzzle5dMutation`/`Puzzle5dDiff` onto that `Value`
// boundary round-trips through the typed `Puzzle5dSnapshot` (`serde_json::from_value`/`to_value`)
// rather than hand-splicing JSON per mutation kind — mirrors `puzzle2d`'s bridge exactly.
//
// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `Puzzle5dDocument.kind_catalogs:
// Option<Value>` (the app's own untyped scratch fixture, `✏️editor/🦀️.rs`) still carries
// the LEGACY embedded-catalog shape end to end — the catalogue panel / mesh-resolution UI reads it
// directly and `kit:in` media import writes it directly, both untouched by this migration since they
// never round-trip through the typed `Puzzle5dSnapshot`. But `serde_json::to_value(a_document)` DOES
// feed straight into this bridge's `from_value::<Puzzle5dSnapshot>` calls below, and the composed
// `Puzzle5dSnapshot::kind_catalogs` field now expects the `{childId,target}` handle shape, not the
// embedded one — a raw `Puzzle5dDocument`-sourced `Value` would otherwise fail that one field's
// deserialize, and because serde fails the WHOLE struct (not just one field) on a shape mismatch,
// `unwrap_or_default()` would silently reset every other field too. `normalize_kind_catalogs_for_
// snapshot_value` is the one guard every `from_value::<Puzzle5dSnapshot>` call in this region funnels
// through to prevent that — never assume an inbound `Value` is already handle-shaped.
fn normalize_kind_catalogs_for_snapshot_value(value: &Value) -> Value {
    let mut value = value.clone();
    let Some(object) = value.as_object_mut() else { return value };
    let is_embedded = object.get("kindCatalogs").is_some_and(|catalogs| catalogs.is_object() && catalogs.get("childId").is_none());
    if !is_embedded {
        return value;
    }
    let Some(catalogs_value) = object.remove("kindCatalogs") else { return value };
    // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
    // through `dsl::DslValue`/`dsl::ToValue`/`dsl::FromValue` instead of
    // `serde_json::from_value`/`to_value` on `Puzzle5dKindCatalogs`/`Puzzle5dKindCatalogsExtra` —
    // both only derive `Serialize`/`Deserialize` under `#[cfg(test)]` now. `Value` (this bridge's
    // own boundary type) is untouched.
    let catalogs: crate::Puzzle5dKindCatalogs = dsl::FromValue::from_value(dsl::DslValue::from(&catalogs_value)).unwrap_or_default();
    let (handle, extra) = crate::split_and_seed_kind_catalogs(Some(catalogs));
    object.insert("kindCatalogs".into(), Value::from(&dsl::ToValue::to_value(&handle)));
    object.insert("kindCatalogsExtra".into(), Value::from(&dsl::ToValue::to_value(&extra)));
    value
}

impl MutationDiff<Value> for Puzzle5dDiff {
    fn apply(&self, projection: &Value) -> protocol::MutationApplyResult<Value> {
        // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
        // through `dsl::DslValue`/`dsl::ToValue`/`dsl::FromValue` instead of
        // `serde_json::from_value`/`to_value` on `Puzzle5dSnapshot` directly — that type only
        // derives `Serialize`/`Deserialize` under `#[cfg(test)]` now. `Value` (this bridge's own
        // boundary type) and `normalize_kind_catalogs_for_snapshot_value` are untouched — this
        // call did not route through that helper before this change either, preserved as-is.
        let base: Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(projection)).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle5dSnapshot>::apply(self, &base).map_err(|error| error.under(["document"]))?;
        Ok(Value::from(dsl::ToValue::to_value(&next)))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    /// 🧷️ Not hand-written: `#[derive(dsl::Mutations)]` on `Puzzle5dMutation` above already
    /// generates `impl protocol::Mutation<Puzzle5dSnapshot>` — including its real `DESCRIPTORS` and
    /// `descriptor()` — but only for that one projection type. This `Value` bridge is a SEPARATE
    /// hand-written `impl Mutation<_>` for the SAME enum (see the module doc's `🔖️ValueBridge`),
    /// which the derive cannot see or extend. The metadata is projection-independent (one leaf per
    /// variant, regardless of which snapshot type it is diffed against), so this forwards straight
    /// through to the derive's own table exactly like `may_emit_foreign_steps` already does below,
    /// rather than hand-authoring a duplicate 28-entry table here.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle5dSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle5dSnapshot>>::descriptor(self)
    }

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle5dDiff> {
        let base: Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(projection))).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        let base: Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(projection))).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::inverse(self, &base)
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle5dSnapshot>::may_emit_foreign_steps(self)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare document JSON the play app mutates), by round-tripping through the typed
/// `Puzzle5dSnapshot` and delegating to [`puzzle5d_snapshot_mutations`].
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dMutation> {
    let before_snapshot: Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(before))).unwrap_or_default();
    let after_snapshot: Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(after))).unwrap_or_default();
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    puzzle5d_snapshot_mutations(&before_snapshot, &after_snapshot)
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlaySnapshot
/// 🌱️ The play app's `Puzzle5dPlayApp` predates the typed `Puzzle5dSnapshot` above and stays on
/// this ad-hoc `serde_json::Value` fixture shape for its scene-mutation helpers. This newtype exists
/// only to satisfy `ArtifactApp::Snapshot: store::ArtifactDsl + store::ArtifactPack`;
/// `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip straight through the
/// still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack encoding respectively),
/// same local-bridge shape as `puzzle2d`'s `Puzzle2dPlaySnapshot`. `Mutation`/`MutationDiff`
/// delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug)]
pub struct Puzzle5dPlaySnapshot(pub Value);

impl PartialEq for Puzzle5dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
    }
}

/// 🩹️ Hand-written, not derived (ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `#[derive(ToValue,
/// FromValue)]` on a `Value` (`serde_json::Value`) field would require `serde_json::Value:
/// ToValue + FromValue`, which does not exist anywhere in this codebase — `ArtifactEditor::Snapshot`
/// (this type's own trait bound, see `✏️editor/🦀️.rs`'s `type Snapshot = Puzzle5dPlaySnapshot`)
/// still needs both traits, so this bridges through `dsl::DslValue`'s own `serde_json::Value`
/// conversions instead.
impl dsl::ToValue for Puzzle5dPlaySnapshot {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::from(&self.0)
    }
}

impl dsl::FromValue for Puzzle5dPlaySnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Ok(Puzzle5dPlaySnapshot(Value::from(value)))
    }
}

impl store::ArtifactDsl for Puzzle5dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle5d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle5dPlaySnapshot).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle5dPlaySnapshot {
    // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: the former
    // `dsl::to_dsl_value(&self.0)`/`dsl::from_dsl_value(value).map(Puzzle5dPlaySnapshot)` calls
    // required `Value` (`serde_json::Value`) to implement `ToValue`/`FromValue`, which it never has
    // anywhere in this codebase — routes through `dsl::DslValue`'s own `serde_json::Value` `From`
    // bridges directly instead.
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::DslValue::from(&self.0).encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        Ok(Puzzle5dPlaySnapshot(Value::from(value)))
    }
}

impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlaySnapshot) -> protocol::MutationApplyResult<Puzzle5dPlaySnapshot> {
        MutationDiff::<Value>::apply(self, &projection.0).map(Puzzle5dPlaySnapshot)
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle5dSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle5dSnapshot>>::descriptor(self)
    }

    fn diff(&self, projection: &Puzzle5dPlaySnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        Mutation::<Value>::diff(self, &projection.0)
    }

    fn inverse(&self, projection: &Puzzle5dPlaySnapshot) -> Vec<Puzzle5dMutation> {
        Mutation::<Value>::inverse(self, &projection.0)
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle5dSnapshot>::may_emit_foreign_steps(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle5dSnapshot>` impl above never actually reads `Puzzle5dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle5dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle5dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle5dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::semantics(self)
    }
    fn label(&self) -> String {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
