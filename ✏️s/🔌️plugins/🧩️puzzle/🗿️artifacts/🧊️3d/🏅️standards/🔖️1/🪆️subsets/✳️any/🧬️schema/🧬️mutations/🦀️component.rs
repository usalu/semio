//! 🧬️ Puzzle 3d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle3dSnapshot>` and `impl protocol::SemanticMutation<Puzzle3dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's `Puzzle3dPlaySnapshot`
//! newtype (`🔖️PlaySnapshot`) live here too, same shape as `puzzle2d`/`puzzle5d`'s: the bridge
//! round-trips through the typed `Puzzle3dSnapshot` instead of hand-splicing JSON per mutation kind.

use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-3d document mutation vocabulary: id-keyed object/target-volume/reference
/// create-delete plus per-field edits, vortex membership, a vortex-to-vortex attraction connect/
/// disconnect relationship, and document-level edits (domain change, kind-compatibility connect/
/// disconnect, kind-catalog replace). There is deliberately no camera mutation: camera pose is
/// session-only app runtime state (`ActionKind::View`), never a document operation. There is
/// deliberately no whole-document mutation: import/reset/example-load goes through
/// `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Puzzle3dSnapshot, diff = Puzzle3dDiff, schema = "puzzle.puzzle3d")]
pub enum Puzzle3dMutation {
    CreateObject(CreateObject),
    DeleteObject(DeleteObject),
    MoveObject(MoveObject),
    RotateObject(RotateObject),
    ScaleObject(ScaleObject),
    ChangeObjectMesh(ChangeObjectMesh),
    EditObjectLabel(EditObjectLabel),
    ChangeObjectKind(ChangeObjectKind),
    ChangeObjectAnchor(ChangeObjectAnchor),
    ChangeObjectHidden(ChangeObjectHidden),
    ChangeObjectLocked(ChangeObjectLocked),
    AddObjectVortex(AddObjectVortex),
    RemoveObjectVortex(RemoveObjectVortex),
    ReplaceObjectVortex(ReplaceObjectVortex),
    ConnectVortices(ConnectVortices),
    DisconnectVortices(DisconnectVortices),
    ReplaceAttractionGeometry(ReplaceAttractionGeometry),
    CreateTargetVolume(CreateTargetVolume),
    DeleteTargetVolume(DeleteTargetVolume),
    MoveTargetVolume(MoveTargetVolume),
    RotateTargetVolume(RotateTargetVolume),
    ScaleTargetVolume(ScaleTargetVolume),
    ChangeTargetVolumeHidden(ChangeTargetVolumeHidden),
    ChangeTargetVolumeLocked(ChangeTargetVolumeLocked),
    CreateReference(CreateReference),
    DeleteReference(DeleteReference),
    MoveReference(MoveReference),
    ResizeReference(ResizeReference),
    ReplaceReferenceSource(ReplaceReferenceSource),
    ChangeReferenceHidden(ChangeReferenceHidden),
    ChangeReferenceLocked(ChangeReferenceLocked),
    ChangeDomain(ChangeDomain),
    ConnectKindCompatibility(ConnectKindCompatibility),
    DisconnectKindCompatibility(DisconnectKindCompatibility),
    ReplaceKindCatalogs(ReplaceKindCatalogs),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle3dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-3d-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and
/// the `mutate-puzzle-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-object",
    "delete-object",
    "move-object",
    "rotate-object",
    "scale-object",
    "change-object-mesh",
    "edit-object-label",
    "change-object-kind",
    "change-object-anchor",
    "change-object-hidden",
    "change-object-locked",
    "add-object-vortex",
    "remove-object-vortex",
    "replace-object-vortex",
    "connect-vortices",
    "disconnect-vortices",
    "replace-attraction-geometry",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "create-reference",
    "delete-reference",
    "move-reference",
    "resize-reference",
    "replace-reference-source",
    "change-reference-hidden",
    "change-reference-locked",
    "change-domain",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use super::add_object_vortex::mutation::{add_object_vortex, AddObjectVortex};
pub use super::change_domain::mutation::{change_domain, ChangeDomain};
pub use super::change_object_anchor::mutation::{change_object_anchor, ChangeObjectAnchor};
pub use super::change_object_hidden::mutation::{change_object_hidden, ChangeObjectHidden};
pub use super::change_object_kind::mutation::{change_object_kind, ChangeObjectKind};
pub use super::change_object_locked::mutation::{change_object_locked, ChangeObjectLocked};
pub use super::change_object_mesh::mutation::{change_object_mesh, ChangeObjectMesh};
pub use super::change_reference_hidden::mutation::{change_reference_hidden, ChangeReferenceHidden};
pub use super::change_reference_locked::mutation::{change_reference_locked, ChangeReferenceLocked};
pub use super::change_target_volume_hidden::mutation::{change_target_volume_hidden, ChangeTargetVolumeHidden};
pub use super::change_target_volume_locked::mutation::{change_target_volume_locked, ChangeTargetVolumeLocked};
pub use super::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::connect_vortices::mutation::{connect_vortices, ConnectVortices};
pub use super::create_object::mutation::{create_object, CreateObject};
pub use super::create_reference::mutation::{create_reference, CreateReference};
pub use super::create_target_volume::mutation::{create_target_volume, CreateTargetVolume};
pub use super::delete_object::mutation::{delete_object, DeleteObject};
pub use super::delete_reference::mutation::{delete_reference, DeleteReference};
pub use super::delete_target_volume::mutation::{delete_target_volume, DeleteTargetVolume};
pub use super::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::disconnect_vortices::mutation::{disconnect_vortices, DisconnectVortices};
pub use super::edit_object_label::mutation::{edit_object_label, EditObjectLabel};
pub use super::move_object::mutation::{move_object, MoveObject};
pub use super::move_reference::mutation::{move_reference, MoveReference};
pub use super::move_target_volume::mutation::{move_target_volume, MoveTargetVolume};
pub use super::remove_object_vortex::mutation::{remove_object_vortex, RemoveObjectVortex};
pub use super::replace_attraction_geometry::mutation::{replace_attraction_geometry, ReplaceAttractionGeometry};
pub use super::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_object_vortex::mutation::{replace_object_vortex, ReplaceObjectVortex};
pub use super::replace_reference_source::mutation::{replace_reference_source, ReplaceReferenceSource};
pub use super::resize_reference::mutation::{resize_reference, ResizeReference};
pub use super::rotate_object::mutation::{rotate_object, RotateObject};
pub use super::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
pub use super::scale_object::mutation::{scale_object, ScaleObject};
pub use super::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};

//#region 🔖️SnapshotDelta
/// 🔀️ Diffs two typed snapshots into a minimal semantic mutation set — the single source of truth
/// both the VCS layer and the `serde_json::Value` scene bridge below replay through.
pub fn puzzle3d_snapshot_mutations(before: &Puzzle3dSnapshot, after: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let mut mutations = Vec::new();
    for object in &before.objects {
        if !after.objects.iter().any(|entry| entry.id == object.id) {
            mutations.push(delete_object(object.id.clone()));
        }
    }
    for object in &after.objects {
        match before.objects.iter().find(|entry| entry.id == object.id) {
            None => mutations.push(create_object(object.clone(), None)),
            Some(prior) => {
                if prior.origin != object.origin {
                    mutations.push(move_object(object.id.clone(), object.origin));
                }
                if prior.orientation != object.orientation {
                    mutations.push(rotate_object(object.id.clone(), object.orientation));
                }
                if prior.scale != object.scale {
                    mutations.push(scale_object(object.id.clone(), object.scale));
                }
                if prior.mesh_url != object.mesh_url {
                    mutations.push(change_object_mesh(object.id.clone(), object.mesh_url.clone()));
                }
                if prior.label != object.label {
                    mutations.push(edit_object_label(object.id.clone(), object.label.clone()));
                }
                if prior.object_kind != object.object_kind {
                    mutations.push(change_object_kind(object.id.clone(), object.object_kind.clone()));
                }
                if prior.anchor != object.anchor {
                    mutations.push(change_object_anchor(object.id.clone(), object.anchor));
                }
                if prior.hidden != object.hidden {
                    mutations.push(change_object_hidden(object.id.clone(), object.hidden));
                }
                if prior.locked != object.locked {
                    mutations.push(change_object_locked(object.id.clone(), object.locked));
                }
                for vortex in &prior.vortices {
                    if !object.vortices.iter().any(|entry| entry.id == vortex.id) {
                        mutations.push(remove_object_vortex(object.id.clone(), vortex.id.clone()));
                    }
                }
                for vortex in &object.vortices {
                    match prior.vortices.iter().find(|entry| entry.id == vortex.id) {
                        None => mutations.push(add_object_vortex(object.id.clone(), vortex.clone(), None)),
                        Some(prior_vortex) if prior_vortex != vortex => mutations.push(replace_object_vortex(object.id.clone(), vortex.id.clone(), vortex.clone())),
                        Some(_) => {}
                    }
                }
            }
        }
    }
    for attraction in &before.attractions {
        if !after.attractions.iter().any(|entry| entry.id == attraction.id) {
            mutations.push(disconnect_vortices(attraction.id.clone()));
        }
    }
    for attraction in &after.attractions {
        match before.attractions.iter().find(|entry| entry.id == attraction.id) {
            None => mutations.push(connect_vortices(
                attraction.id.clone(),
                attraction.attracting.clone(),
                attraction.attracted.clone(),
                attraction.gap,
                attraction.shift,
                attraction.rise,
                attraction.rotation,
                attraction.turn,
                attraction.tilt,
                attraction.x,
                attraction.y,
            )),
            Some(prior) if prior.attracting != attraction.attracting || prior.attracted != attraction.attracted => {
                mutations.push(disconnect_vortices(attraction.id.clone()));
                mutations.push(connect_vortices(
                    attraction.id.clone(),
                    attraction.attracting.clone(),
                    attraction.attracted.clone(),
                    attraction.gap,
                    attraction.shift,
                    attraction.rise,
                    attraction.rotation,
                    attraction.turn,
                    attraction.tilt,
                    attraction.x,
                    attraction.y,
                ));
            }
            Some(prior) => {
                if prior.gap != attraction.gap
                    || prior.shift != attraction.shift
                    || prior.rise != attraction.rise
                    || prior.rotation != attraction.rotation
                    || prior.turn != attraction.turn
                    || prior.tilt != attraction.tilt
                    || prior.x != attraction.x
                    || prior.y != attraction.y
                {
                    mutations.push(replace_attraction_geometry(attraction.id.clone(), attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y));
                }
            }
        }
    }
    for volume in &before.target_volumes {
        if !after.target_volumes.iter().any(|entry| entry.id == volume.id) {
            mutations.push(delete_target_volume(volume.id.clone()));
        }
    }
    for volume in &after.target_volumes {
        match before.target_volumes.iter().find(|entry| entry.id == volume.id) {
            None => mutations.push(create_target_volume(volume.clone(), None)),
            Some(prior) => {
                if prior.origin != volume.origin {
                    mutations.push(move_target_volume(volume.id.clone(), volume.origin));
                }
                if prior.orientation != volume.orientation {
                    mutations.push(rotate_target_volume(volume.id.clone(), volume.orientation));
                }
                if prior.scale != volume.scale {
                    mutations.push(scale_target_volume(volume.id.clone(), volume.scale));
                }
                if prior.hidden != volume.hidden {
                    mutations.push(change_target_volume_hidden(volume.id.clone(), volume.hidden));
                }
                if prior.locked != volume.locked {
                    mutations.push(change_target_volume_locked(volume.id.clone(), volume.locked));
                }
            }
        }
    }
    for reference in &before.references {
        if !after.references.iter().any(|entry| entry.id == reference.id) {
            mutations.push(delete_reference(reference.id.clone()));
        }
    }
    for reference in &after.references {
        match before.references.iter().find(|entry| entry.id == reference.id) {
            None => mutations.push(create_reference(reference.clone(), None)),
            Some(prior) => {
                if prior.origin != reference.origin {
                    mutations.push(move_reference(reference.id.clone(), reference.origin));
                }
                if prior.width_world != reference.width_world {
                    mutations.push(resize_reference(reference.id.clone(), reference.width_world));
                }
                if prior.source != reference.source {
                    mutations.push(replace_reference_source(reference.id.clone(), reference.source.clone()));
                }
                if prior.hidden != reference.hidden {
                    mutations.push(change_reference_hidden(reference.id.clone(), reference.hidden));
                }
                if prior.locked != reference.locked {
                    mutations.push(change_reference_locked(reference.id.clone(), reference.locked));
                }
            }
        }
    }
    if before.domain != after.domain {
        mutations.push(change_domain(after.domain.clone()));
    }
    for row in &before.meta.kind_compatibility {
        if !after.meta.kind_compatibility.iter().any(|entry| entry.source == row.source && entry.target == row.target) {
            mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
        }
    }
    for row in &after.meta.kind_compatibility {
        match before.meta.kind_compatibility.iter().find(|entry| entry.source == row.source && entry.target == row.target) {
            None => mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)),
            Some(prior) if prior != row => {
                mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity));
            }
            Some(_) => {}
        }
    }
    if before.meta.kind_catalogs != after.meta.kind_catalogs {
        mutations.push(replace_kind_catalogs(after.meta.kind_catalogs.clone()));
    }
    mutations
}
//#endregion 🔖️SnapshotDelta

/// ▶️ Applies `mutation` via its diff.
pub fn apply_puzzle3d_mutation(projection: &mut Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_puzzle3d_mutation(projection: &Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> Vec<Puzzle3dMutation> {
    mutation.inverse(projection)
}

//#region 🔖️ValueBridge
// 🌉️ The play app's scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture. Bridging `Puzzle3dMutation`/`Puzzle3dDiff` onto that `Value`
// boundary round-trips through the typed `Puzzle3dSnapshot` (`serde_json::from_value`/`to_value`)
// rather than hand-splicing JSON per mutation kind — mirrors `puzzle2d`/`puzzle5d`'s bridge exactly.
impl MutationDiff<Value> for Puzzle3dDiff {
    fn apply(&self, projection: &Value) -> protocol::MutationApplyResult<Value> {
        let base: Puzzle3dSnapshot = serde_json::from_value(projection.clone()).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle3dSnapshot>::apply(self, &base).map_err(|error| error.under(["document"]))?;
        serde_json::to_value(next).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-result", error.to_string()).at(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle3dMutation {
    type Diff = Puzzle3dDiff;

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle3dDiff> {
        let base: Puzzle3dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        Mutation::<Puzzle3dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        let base: Puzzle3dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        Mutation::<Puzzle3dSnapshot>::inverse(self, &base)
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle3dSnapshot>::may_emit_foreign_steps(self)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare document JSON the play app mutates), by round-tripping through the typed
/// `Puzzle3dSnapshot` and delegating to [`puzzle3d_snapshot_mutations`].
pub fn puzzle3d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle3dMutation> {
    let before_snapshot: Puzzle3dSnapshot = serde_json::from_value(before.clone()).unwrap_or_default();
    let after_snapshot: Puzzle3dSnapshot = serde_json::from_value(after.clone()).unwrap_or_default();
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    puzzle3d_snapshot_mutations(&before_snapshot, &after_snapshot)
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlaySnapshot
/// 🌱️ The play app's `Puzzle3dPlayApp` predates the typed `Puzzle3dSnapshot` above and stays on
/// this ad-hoc `serde_json::Value` fixture shape for its scene-mutation helpers. This newtype exists
/// only to satisfy `ArtifactApp::Snapshot: store::ArtifactDsl + store::ArtifactPack`;
/// `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip straight through the
/// still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack encoding respectively),
/// same local-bridge shape as `puzzle2d`'s `Puzzle2dPlaySnapshot`. `Mutation`/`MutationDiff`
/// delegate straight through to the `Value` impls above too.
#[derive(Debug)]
pub struct Puzzle3dPlaySnapshot {
    typed: std::sync::Arc<Puzzle3dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<Value>>,
}

impl Puzzle3dPlaySnapshot {
    /// 🎯️ Builds the typed snapshot once and retains the supplied projection for read paths.
    pub fn new(value: Value) -> Self {
        let typed = serde_json::from_value(value.clone()).unwrap_or_default();
        let projected = std::sync::OnceLock::new();
        let _ = projected.set(std::sync::Arc::new(value));
        Self { typed: std::sync::Arc::new(typed), value: projected }
    }

    /// 🧬️ Keeps mutation application typed and defers the JSON bridge until a reader needs it.
    fn from_typed(typed: Puzzle3dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ Materializes the legacy play projection at most once per immutable snapshot.
    pub fn value(&self) -> &Value {
        self.value.get_or_init(|| std::sync::Arc::new(serde_json::to_value(self.typed.as_ref()).unwrap_or(Value::Null))).as_ref()
    }

    /// 🧬️ Exposes the immutable typed authority without materializing the legacy JSON projection.
    pub fn typed(&self) -> &Puzzle3dSnapshot {
        self.typed.as_ref()
    }
}

impl Clone for Puzzle3dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl Serialize for Puzzle3dPlaySnapshot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Puzzle3dPlaySnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Value::deserialize(deserializer).map(Self::new)
    }
}

impl PartialEq for Puzzle3dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

impl store::ArtifactDsl for Puzzle3dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle3d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Self::new).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self.value()).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle3dPlaySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self.value()).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Self::new).map_err(store::PackError::Schema)
    }
}

impl MutationDiff<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn apply(&self, projection: &Puzzle3dPlaySnapshot) -> protocol::MutationApplyResult<Puzzle3dPlaySnapshot> {
        MutationDiff::<Puzzle3dSnapshot>::apply(self, projection.typed.as_ref()).map(Puzzle3dPlaySnapshot::from_typed)
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    type Diff = Puzzle3dDiff;

    fn diff(&self, projection: &Puzzle3dPlaySnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        Mutation::<Puzzle3dSnapshot>::diff(self, projection.typed.as_ref())
    }

    fn inverse(&self, projection: &Puzzle3dPlaySnapshot) -> Vec<Puzzle3dMutation> {
        Mutation::<Puzzle3dSnapshot>::inverse(self, projection.typed.as_ref())
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle3dSnapshot>::may_emit_foreign_steps(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle3dSnapshot>` impl above never actually reads `Puzzle3dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle3dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle3dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle3dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::semantics(self)
    }
    fn label(&self) -> String {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle3d_delta_ops_round_trip_and_stay_granular() {
        let before = serde_json::json!({
            "schema": crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA, "domain": "architecture",
            "meta": {},
            "objects": [
                { "id": "o1", "anchor": "fixed", "origin": [0.0,0.0,0.0], "vortices": [] },
                { "id": "o2", "anchor": "fixed", "origin": [1.0,0.0,0.0], "vortices": [] },
            ],
            "attractions": [], "targetVolumes": [], "references": [],
        });
        let after = serde_json::json!({
            "schema": crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA, "domain": "architecture",
            "meta": {},
            "objects": [
                { "id": "o2", "anchor": "fixed", "origin": [9.0,0.0,0.0], "vortices": [] },
                { "id": "o3", "anchor": "fixed", "origin": [2.0,0.0,0.0], "vortices": [] },
            ],
            "attractions": [], "targetVolumes": [], "references": [],
        });
        let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle3dSnapshot>(value.clone()).expect("typed puzzle3d fixture")).expect("canonical puzzle3d JSON");
        let operations = puzzle3d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::MoveObject(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::CreateObject(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::DeleteObject(_))));
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Mutation::<Value>::inverse(operation, &forward));
            forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward).expect("valid mutation diff");
        }
        assert_eq!(forward, canonical(&after));
        for inverse in inverses.iter().rev() {
            forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward).expect("valid mutation diff");
        }
        assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
    }

    //#region 🔖️MutationLaws
    use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    #[test]
    fn move_object_diff_absorb_law() {
        use crate::artifacts::puzzle3d::Puzzle3dObject;
        let base = empty();
        let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
        let d1 = move_object("o1".into(), [10.0, 10.0, 10.0]).diff(&with_object).into_parts().0;
        let mid = MutationDiff::<Puzzle3dSnapshot>::apply(&d1, &with_object).expect("valid mutation diff");
        let d2 = move_object("o1".into(), [20.0, 30.0, 40.0]).diff(&mid).into_parts().0;
        semio_framework::io::resolve_ready(assert_mutation_diff_absorb_law(&with_object, d1, d2));
    }

    fn empty() -> Puzzle3dSnapshot {
        Puzzle3dSnapshot::default()
    }

    #[test]
    fn create_delete_object_inverse_law() {
        use crate::artifacts::puzzle3d::Puzzle3dObject;
        let base = empty();
        let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_object(object.clone(), None)));
        let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &delete_object("o1".into())));
    }

    #[test]
    fn object_field_mutations_inverse_law() {
        use crate::artifacts::puzzle3d::{Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dScale, Puzzle3dVortex};
        let base = empty();
        let object = Puzzle3dObject {
            id: "o1".into(),
            label: None,
            object_kind: None,
            anchor: Default::default(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: vec![Puzzle3dVortex { id: "v1".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
            hidden: false,
            locked: false,
        };
        let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &move_object("o1".into(), [1.0, 2.0, 3.0])));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &rotate_object("o1".into(), Some([0.0, 0.0, 0.0, 1.0]))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &scale_object("o1".into(), Some(Puzzle3dScale::Uniform(2.0)))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_mesh("o1".into(), Some("mesh://a".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &edit_object_label("o1".into(), Some("Label".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_kind("o1".into(), Some("core.capsule".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_anchor("o1".into(), Puzzle3dObjectAnchor::Derived)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_hidden("o1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_locked("o1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(
            &with_object,
            &add_object_vortex("o1".into(), Puzzle3dVortex { id: "v2".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }, None),
        ));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &remove_object_vortex("o1".into(), "v1".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(
            &with_object,
            &replace_object_vortex("o1".into(), "v1".into(), Puzzle3dVortex { id: "v1".into(), vortex_kind: Some("k".into()), label: None, position: [1.0, 1.0, 1.0], direction: None, radius: None, hidden: false, locked: false }),
        ));
    }

    #[test]
    fn connect_disconnect_vortices_inverse_law_and_cascade() {
        use crate::artifacts::puzzle3d::{Puzzle3dObject, Puzzle3dVortex};
        let base = empty();
        let object_a = Puzzle3dObject {
            id: "a".into(),
            label: None,
            object_kind: None,
            anchor: Default::default(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: vec![Puzzle3dVortex { id: "va".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
            hidden: false,
            locked: false,
        };
        let object_b = Puzzle3dObject {
            id: "b".into(),
            label: None,
            object_kind: None,
            anchor: Default::default(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: vec![Puzzle3dVortex { id: "vb".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
            hidden: false,
            locked: false,
        };
        let mut projection = base;
        projection = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        projection = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
        let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_vortices("t1".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &replace_attraction_geometry("t1".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)));
        let deleted = delete_object("a".into());
        let after_delete = MutationDiff::<Puzzle3dSnapshot>::apply(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
        assert!(!after_delete.attractions.iter().any(|attraction| attraction.id == "t1"), "delete-object must sever attractions touching its vortices");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &deleted));
    }

    #[test]
    fn target_volume_and_reference_inverse_law() {
        use crate::artifacts::puzzle3d::{Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dTargetVolume};
        let base = empty();
        let volume = Puzzle3dTargetVolume { id: "tv1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: false, locked: false };
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_target_volume(volume.clone(), None)));
        let with_volume = MutationDiff::<Puzzle3dSnapshot>::apply(create_target_volume(volume, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &move_target_volume("tv1".into(), [1.0, 2.0, 3.0])));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &rotate_target_volume("tv1".into(), Some([0.0, 0.0, 0.0, 1.0]))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &scale_target_volume("tv1".into(), None)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &change_target_volume_hidden("tv1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &change_target_volume_locked("tv1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &delete_target_volume("tv1".into())));

        let reference = Puzzle3dReference { id: "r1".into(), source: Puzzle3dReferenceSource::default(), origin: [0.0, 0.0, 0.0], width_world: 1.0, locked: false, hidden: false };
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_reference(reference.clone(), None)));
        let with_reference = MutationDiff::<Puzzle3dSnapshot>::apply(create_reference(reference, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &move_reference("r1".into(), [1.0, 2.0, 3.0])));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &resize_reference("r1".into(), 4.0)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &replace_reference_source("r1".into(), Puzzle3dReferenceSource { url: "/x.png".into(), media_kind: Some("image".into()) })));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &change_reference_hidden("r1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &change_reference_locked("r1".into(), true)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &delete_reference("r1".into())));
    }

    #[test]
    fn document_scalar_mutations_inverse_law() {
        use crate::artifacts::puzzle3d::{Puzzle3dCompatSpecificity, Puzzle3dKindCatalogs};
        let base = empty();
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_domain("mechanical".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex)));
        let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle3dKindCatalogs::default()))));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_puzzle3d_mutation_descriptors();
        for kind in <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds().len(), 35);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
    // `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
    use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    #[test]
    fn missing_target_is_error_per_verb_family() {
        let base = empty();
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &delete_object("missing".into()))); // delete
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &remove_object_vortex("missing".into(), "v0".into()))); // remove
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &change_object_hidden("missing".into(), true))); // change/set/update
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &move_object("missing".into(), [1.0, 1.0, 1.0]))); // move/drag/rotate/scale/resize
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &edit_object_label("missing".into(), Some("x".into())))); // edit/replace
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &disconnect_vortices("missing".into())));
        // disconnect/unbind
    }

    #[test]
    fn create_duplicate_id_is_fatal_and_never_applies() {
        use crate::artifacts::puzzle3d::Puzzle3dObject;
        let mut base = empty();
        let object = Puzzle3dObject { id: "o0".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        base.objects.push(object.clone());
        let outcome = create_object(object, None).diff(&base);
        semio_framework::io::resolve_ready(assert_fatal_never_applies(&outcome));
        assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🧪️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
    /// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
    /// declared vocabulary and the measured one from drifting apart.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle3dMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog
}
//#endregion 🧪️Tests
