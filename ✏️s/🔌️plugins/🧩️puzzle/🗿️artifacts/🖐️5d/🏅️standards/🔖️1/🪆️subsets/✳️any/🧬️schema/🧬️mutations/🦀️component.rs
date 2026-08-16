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


use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-5d document mutation vocabulary: id-keyed part create-delete plus per-2d/
/// per-3d-projection field edits, grip membership, a grip-to-grip fastener connect/disconnect
/// relationship, and document-level edits (label rename, domain/description change,
/// kind-compatibility connect/disconnect, kind-catalog replace). There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
//#endregion 🔖️Mutations

pub use super::add_part_grip::mutation::{add_part_grip, AddPartGrip};
pub use super::change_description::mutation::{change_description, ChangeDescription};
pub use super::change_domain::mutation::{change_domain, ChangeDomain};
pub use super::change_fastener_kind::mutation::{change_fastener_kind, ChangeFastenerKind};
pub use super::change_part_2d_hidden::mutation::{change_part_2d_hidden, ChangePart2dHidden};
pub use super::change_part_2d_icon::mutation::{change_part_2d_icon, ChangePart2dIcon};
pub use super::change_part_2d_locked::mutation::{change_part_2d_locked, ChangePart2dLocked};
pub use super::change_part_3d_mesh::mutation::{change_part_3d_mesh, ChangePart3dMesh};
pub use super::change_part_anchor::mutation::{change_part_anchor, ChangePartAnchor};
pub use super::change_part_kind::mutation::{change_part_kind, ChangePartKind};
pub use super::connect_grips::mutation::{connect_grips, ConnectGrips};
pub use super::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::create_part::mutation::{create_part, CreatePart};
pub use super::delete_part::mutation::{delete_part, DeletePart};
pub use super::disconnect_grips::mutation::{disconnect_grips, DisconnectGrips};
pub use super::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::edit_part_2d_text::mutation::{edit_part_2d_text, EditPart2dText};
pub use super::edit_part_3d_label::mutation::{edit_part_3d_label, EditPart3dLabel};
pub use super::move_part_2d::mutation::{move_part_2d, MovePart2d};
pub use super::move_part_3d::mutation::{move_part_3d, MovePart3d};
pub use super::remove_part_grip::mutation::{remove_part_grip, RemovePartGrip};
pub use super::rename_puzzle5d::mutation::{rename_puzzle5d, RenamePuzzle5d};
pub use super::replace_fastener_geometry::mutation::{replace_fastener_geometry, ReplaceFastenerGeometry};
pub use super::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_part_2d_geometry::mutation::{replace_part_2d_geometry, ReplacePart2dGeometry};
pub use super::replace_part_grip::mutation::{replace_part_grip, ReplacePartGrip};
pub use super::rotate_part_3d::mutation::{rotate_part_3d, RotatePart3d};
pub use super::scale_part_3d::mutation::{scale_part_3d, ScalePart3d};

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
                fastener.id.clone(), fastener.source.clone(), fastener.target.clone(), fastener.fastener_kind.clone(),
                fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y,
            )),
            Some(prior) if prior.source != fastener.source || prior.target != fastener.target => {
                mutations.push(disconnect_grips(fastener.id.clone()));
                mutations.push(connect_grips(
                    fastener.id.clone(), fastener.source.clone(), fastener.target.clone(), fastener.fastener_kind.clone(),
                    fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y,
                ));
            }
            Some(prior) => {
                if prior.gap != fastener.gap || prior.shift != fastener.shift || prior.rise != fastener.rise || prior.rotation != fastener.rotation || prior.turn != fastener.turn || prior.tilt != fastener.tilt || prior.x != fastener.x || prior.y != fastener.y {
                    mutations.push(replace_fastener_geometry(fastener.id.clone(), fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y));
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
        mutations.push(replace_kind_catalogs(crate::artifacts::puzzle5d::kind_catalogs_of(&after.kind_catalogs, &after.kind_catalogs_extra)));
    }
    mutations
}
//#endregion 🔖️SnapshotDelta

/// ▶️ Applies `mutation` via its diff.
pub fn apply_puzzle5d_mutation(projection: &mut Puzzle5dSnapshot, mutation: &Puzzle5dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
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
// Option<Value>` (the app's own untyped scratch fixture, `🎛️apps/🖐️5d/🦀️component.rs`) still carries
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
    let is_embedded = object.get("kindCatalogs").map(|catalogs| catalogs.is_object() && catalogs.get("childId").is_none()).unwrap_or(false);
    if !is_embedded {
        return value;
    }
    let Some(catalogs_value) = object.remove("kindCatalogs") else { return value };
    let catalogs: crate::artifacts::puzzle5d::Puzzle5dKindCatalogs = serde_json::from_value(catalogs_value).unwrap_or_default();
    let (handle, extra) = crate::artifacts::puzzle5d::split_and_seed_kind_catalogs(Some(catalogs));
    if let Ok(handle_value) = serde_json::to_value(&handle) { object.insert("kindCatalogs".into(), handle_value); }
    if let Ok(extra_value) = serde_json::to_value(&extra) { object.insert("kindCatalogsExtra".into(), extra_value); }
    value
}

impl MutationDiff<Value> for Puzzle5dDiff {
    fn apply(&self, projection: &Value) -> Value {
        let base: Puzzle5dSnapshot = serde_json::from_value(normalize_kind_catalogs_for_snapshot_value(projection)).unwrap_or_default();
        let next = MutationDiff::<Puzzle5dSnapshot>::apply(self, &base);
        serde_json::to_value(next).unwrap_or_else(|_| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle5dDiff> {
        let base: Puzzle5dSnapshot = serde_json::from_value(normalize_kind_catalogs_for_snapshot_value(projection)).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        let base: Puzzle5dSnapshot = serde_json::from_value(normalize_kind_catalogs_for_snapshot_value(projection)).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::inverse(self, &base)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare document JSON the play app mutates), by round-tripping through the typed
/// `Puzzle5dSnapshot` and delegating to [`puzzle5d_snapshot_mutations`].
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dMutation> {
    let before_snapshot: Puzzle5dSnapshot = serde_json::from_value(normalize_kind_catalogs_for_snapshot_value(before)).unwrap_or_default();
    let after_snapshot: Puzzle5dSnapshot = serde_json::from_value(normalize_kind_catalogs_for_snapshot_value(after)).unwrap_or_default();
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle5dPlaySnapshot(pub Value);

impl PartialEq for Puzzle5dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
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
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(&self.0).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Puzzle5dPlaySnapshot).map_err(store::PackError::Schema)
    }
}

impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlaySnapshot) -> Puzzle5dPlaySnapshot {
        Puzzle5dPlaySnapshot(MutationDiff::<Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Puzzle5dPlaySnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        Mutation::<Value>::diff(self, &projection.0)
    }

    fn inverse(&self, projection: &Puzzle5dPlaySnapshot) -> Vec<Puzzle5dMutation> {
        Mutation::<Value>::inverse(self, &projection.0)
    }
}
//#endregion 🔖️PlaySnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_delta_ops_round_trip_and_stay_granular() {
        let before = serde_json::json!({
            "schema": crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p1", "2d": { "x": 0.0, "y": 0.0 }, "3d": { "origin": [0.0,0.0,0.0] }, "grips": [] },
                { "id": "p2", "2d": { "x": 1.0, "y": 0.0 }, "3d": { "origin": [1.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let after = serde_json::json!({
            "schema": crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p2", "2d": { "x": 9.0, "y": 0.0 }, "3d": { "origin": [9.0,0.0,0.0] }, "grips": [] },
                { "id": "p3", "2d": { "x": 2.0, "y": 0.0 }, "3d": { "origin": [2.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle5dSnapshot>(value.clone()).expect("typed puzzle5d fixture")).expect("canonical puzzle5d JSON");
        let operations = puzzle5d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::MovePart2d(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))));
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Mutation::<Value>::inverse(operation, &forward));
            forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward);
        }
        assert_eq!(forward, canonical(&after));
        for inverse in inverses.iter().rev() {
            forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward);
        }
        assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
    }

    //#region 🔖️MutationLaws
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    #[test]
    fn move_part_2d_diff_absorb_law() {
        use crate::artifacts::puzzle5d::Puzzle5dPart;
        let base = empty();
        let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
        let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base);
        let d1 = move_part_2d("p1".into(), 10.0, 10.0).diff(&with_part).into_parts().0;
        let mid = MutationDiff::<Puzzle5dSnapshot>::apply(&d1, &with_part);
        let d2 = move_part_2d("p1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&with_part, d1, d2);
    }

    fn empty() -> Puzzle5dSnapshot {
        Puzzle5dSnapshot::default()
    }

    #[test]
    fn create_delete_part_inverse_law() {
        use crate::artifacts::puzzle5d::Puzzle5dPart;
        let base = empty();
        let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
        assert_mutation_inverse_law(&base, &create_part(part.clone(), None));
        let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base);
        assert_mutation_inverse_law(&with_part, &delete_part("p1".into()));
    }

    #[test]
    fn part_field_mutations_inverse_law() {
        use crate::artifacts::puzzle5d::{Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dScale};
        let base = empty();
        let part = Puzzle5dPart { id: "p1".into(), grips: vec![Puzzle5dGrip { id: "g1".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
        let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base);
        assert_mutation_inverse_law(&with_part, &move_part_2d("p1".into(), 5.0, 6.0));
        assert_mutation_inverse_law(&with_part, &replace_part_2d_geometry("p1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0)));
        assert_mutation_inverse_law(&with_part, &edit_part_2d_text("p1".into(), Some("hi".into())));
        assert_mutation_inverse_law(&with_part, &change_part_2d_icon("p1".into(), Some("star".into())));
        assert_mutation_inverse_law(&with_part, &change_part_2d_hidden("p1".into(), Some(true)));
        assert_mutation_inverse_law(&with_part, &change_part_2d_locked("p1".into(), Some(true)));
        assert_mutation_inverse_law(&with_part, &move_part_3d("p1".into(), [1.0, 2.0, 3.0]));
        assert_mutation_inverse_law(&with_part, &rotate_part_3d("p1".into(), Some([0.0, 0.0, 0.0, 1.0])));
        assert_mutation_inverse_law(&with_part, &scale_part_3d("p1".into(), Some(Puzzle5dScale::Uniform(2.0))));
        assert_mutation_inverse_law(&with_part, &change_part_3d_mesh("p1".into(), Some("mesh://a".into())));
        assert_mutation_inverse_law(&with_part, &edit_part_3d_label("p1".into(), Some("Label".into())));
        assert_mutation_inverse_law(&with_part, &change_part_kind("p1".into(), Some("core.capsule".into())));
        assert_mutation_inverse_law(&with_part, &change_part_anchor("p1".into(), Puzzle5dPartAnchor::Derived));
        assert_mutation_inverse_law(&with_part, &add_part_grip("p1".into(), Puzzle5dGrip { id: "g2".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }, None));
        assert_mutation_inverse_law(&with_part, &remove_part_grip("p1".into(), "g1".into()));
        assert_mutation_inverse_law(&with_part, &replace_part_grip("p1".into(), "g1".into(), Puzzle5dGrip { id: "g1".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() }));
    }

    #[test]
    fn connect_disconnect_grips_inverse_law_and_cascade() {
        use crate::artifacts::puzzle5d::{Puzzle5dGrip, Puzzle5dPart};
        let base = empty();
        let part_a = Puzzle5dPart { id: "a".into(), grips: vec![Puzzle5dGrip { id: "ga".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
        let part_b = Puzzle5dPart { id: "b".into(), grips: vec![Puzzle5dGrip { id: "gb".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
        let mut projection = base;
        projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_a, None).diff(&projection).diff(), &projection);
        projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_b, None).diff(&projection).diff(), &projection);
        assert_mutation_inverse_law(&projection, &connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection);
        assert_mutation_inverse_law(&connected, &disconnect_grips("f1".into()));
        assert_mutation_inverse_law(&connected, &replace_fastener_geometry("f1".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0));
        assert_mutation_inverse_law(&connected, &change_fastener_kind("f1".into(), Some("core.link".into())));
        let deleted = delete_part("a".into());
        let after_delete = MutationDiff::<Puzzle5dSnapshot>::apply(deleted.diff(&connected).diff(), &connected);
        assert!(!after_delete.fasteners.iter().any(|fastener| fastener.id == "f1"), "delete-part must sever fasteners touching its grips");
        assert_mutation_inverse_law(&connected, &deleted);
    }

    #[test]
    fn document_scalar_mutations_inverse_law() {
        use crate::artifacts::puzzle5d::{Puzzle5dCompatSpecificity, Puzzle5dKindCatalogs};
        let base = empty();
        assert_mutation_inverse_law(&base, &rename_puzzle5d(Some("Nakagin".into())));
        assert_mutation_inverse_law(&base, &change_domain("mechanical".into()));
        assert_mutation_inverse_law(&base, &change_description("a scene".into()));
        assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip));
        let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip).diff(&base).diff(), &base);
        assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into()));
        assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle5dKindCatalogs::default())));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_puzzle5d_mutation_descriptors();
        for kind in Puzzle5dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Puzzle5dMutation::kinds().len(), 28);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
    // `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    #[test]
    fn missing_target_is_error_per_verb_family() {
        let base = empty();
        assert_missing_target_is_error(&base, &delete_part("missing".into())); // delete
        assert_missing_target_is_error(&base, &remove_part_grip("missing".into(), "g0".into())); // remove
        assert_missing_target_is_error(&base, &change_part_2d_icon("missing".into(), Some("star".into()))); // change/set/update
        assert_missing_target_is_error(&base, &move_part_2d("missing".into(), 1.0, 1.0)); // move/drag/rotate/scale/resize
        assert_missing_target_is_error(&base, &edit_part_3d_label("missing".into(), Some("x".into()))); // edit/replace
        assert_missing_target_is_error(&base, &disconnect_grips("missing".into())); // disconnect/unbind
    }

    #[test]
    fn create_duplicate_id_is_fatal_and_never_applies() {
        use crate::artifacts::puzzle5d::Puzzle5dPart;
        let mut base = empty();
        let part = Puzzle5dPart { id: "p0".into(), ..Default::default() };
        base.parts.push(part.clone());
        let outcome = create_part(part, None).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
