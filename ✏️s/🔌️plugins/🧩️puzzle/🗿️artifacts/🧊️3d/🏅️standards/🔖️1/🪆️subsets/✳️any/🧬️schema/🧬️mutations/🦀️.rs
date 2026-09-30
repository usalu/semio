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

use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-3d document mutation vocabulary: id-keyed object/target-volume/reference
/// create-delete plus per-field edits, vortex membership, a vortex-to-vortex attraction connect/
/// disconnect relationship, and document-level edits (domain change, kind-compatibility connect/
/// disconnect, kind-catalog replace), and the three parametric selection transforms (`drag-`,
/// `rotate-`, `scale-selection`) that record a gesture's own inputs. There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
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
    DragSelection(DragSelection),
    RotateSelection(RotateSelection),
    ScaleSelection(ScaleSelection),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle3dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧊️mutate-puzzle-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
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
    "drag-selection",
    "rotate-selection",
    "scale-selection",
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
pub use super::drag_selection::mutation::{drag_selection, DragSelection};
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
pub use super::rotate_selection::mutation::{rotate_selection, RotateSelection};
pub use super::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
pub use super::scale_object::mutation::{scale_object, ScaleObject};
pub use super::scale_selection::mutation::{scale_selection, ScaleSelection};
pub use super::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};

//#region 🔖️SelectionTransform
/// 🧭️ Shared diff of the three parametric selection leaves (`drag-`, `rotate-`, `scale-selection`).
/// `targets` is classified by document membership against `base`: an object id goes through `object`,
/// a target-volume id through `volume`, each record transformed IN PLACE about its own origin. An empty
/// or repeated target set is the Fatal `mutation.invariant` the payload schema's `minItems`/`uniqueItems`
/// forbid. Absent ids and locked records are skipped with one `mutation.partial` warning per reason (in
/// that order, ids in payload order); nothing left is `mutation.target-missing`; an `identity`
/// transform, or survivors that do not move, is `mutation.no-op`. Every moved record is patched whole
/// from the base, in document order, so the leaf replays on any base.
pub fn puzzle3d_selection_diff(
    base: &Puzzle3dSnapshot,
    targets: &[String],
    identity: bool,
    object: impl Fn(&crate::Puzzle3dObject) -> crate::Puzzle3dObject,
    volume: impl Fn(&crate::Puzzle3dTargetVolume) -> crate::Puzzle3dTargetVolume,
) -> protocol::MutationOutcome<Puzzle3dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry, Puzzle3dTargetVolumesDelta};
    if let Err(reason) = puzzle3d_targets_invariant(targets) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec());
    }
    let (mut missing, mut locked, mut survivors) = (Vec::<String>::new(), Vec::<String>::new(), std::collections::BTreeSet::<&str>::new());
    for id in targets {
        match (base.objects.iter().find(|entry| &entry.id == id).map(|entry| entry.locked), base.target_volumes.iter().find(|entry| &entry.id == id).map(|entry| entry.locked)) {
            (Some(false), _) | (None, Some(false)) => {
                survivors.insert(id.as_str());
            }
            (Some(true), _) | (None, Some(true)) => locked.push(id.clone()),
            (None, None) => missing.push(id.clone()),
        }
    }
    if survivors.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an unlocked object or target volume", targets.len()), targets.to_vec());
    }
    let partial: Vec<protocol::MutationMessage> = [(missing, "not in this scene"), (locked, "locked")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    let objects: Vec<Puzzle3dObjectPatchEntry> = if identity {
        Vec::new()
    } else {
        base.objects.iter().filter(|entry| survivors.contains(entry.id.as_str())).filter_map(|entry| Some(object(entry)).filter(|next| next != entry).map(|next| Puzzle3dObjectPatchEntry { id: entry.id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } })).collect()
    };
    let volumes: Vec<Puzzle3dTargetVolumePatchEntry> = if identity {
        Vec::new()
    } else {
        base.target_volumes.iter().filter(|entry| survivors.contains(entry.id.as_str())).filter_map(|entry| Some(volume(entry)).filter(|next| next != entry).map(|next| Puzzle3dTargetVolumePatchEntry { id: entry.id.clone(), patch: Puzzle3dTargetVolumePatch { replacement: Some(next) } })).collect()
    };
    if objects.is_empty() && volumes.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(targets.to_vec())]));
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: (!objects.is_empty()).then(|| Puzzle3dObjectsDelta { patched: objects, ..Default::default() }),
        target_volumes: (!volumes.is_empty()).then(|| Puzzle3dTargetVolumesDelta { patched: volumes, ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(partial)
}

/// ↩️ Exact base-derived inverse of a selection transform: the absolute setters restoring every pose
/// field its forward `outcome` changes — origin, orientation, scale — so an undo never accumulates the
/// float error a negated offset, angle or factor would.
pub fn puzzle3d_selection_inverse(base: &Puzzle3dSnapshot, outcome: protocol::MutationOutcome<Puzzle3dDiff>) -> Vec<Puzzle3dMutation> {
    let (diff, _) = outcome.into_parts();
    let mut steps = Vec::new();
    for entry in diff.objects.iter().flat_map(|delta| &delta.patched) {
        let (Some(before), Some(after)) = (base.objects.iter().find(|object| object.id == entry.id), entry.patch.replacement.as_ref()) else { continue };
        if before.origin != after.origin {
            steps.push(move_object(before.id.clone(), before.origin));
        }
        if before.orientation != after.orientation {
            steps.push(rotate_object(before.id.clone(), before.orientation));
        }
        if before.scale != after.scale {
            steps.push(scale_object(before.id.clone(), before.scale));
        }
    }
    for entry in diff.target_volumes.iter().flat_map(|delta| &delta.patched) {
        let (Some(before), Some(after)) = (base.target_volumes.iter().find(|volume| volume.id == entry.id), entry.patch.replacement.as_ref()) else { continue };
        if before.origin != after.origin {
            steps.push(move_target_volume(before.id.clone(), before.origin));
        }
        if before.orientation != after.orientation {
            steps.push(rotate_target_volume(before.id.clone(), before.orientation));
        }
        if before.scale != after.scale {
            steps.push(scale_target_volume(before.id.clone(), before.scale));
        }
    }
    steps
}

/// 🗃️ A selection target set names at least one id and no id twice.
pub fn puzzle3d_targets_invariant(targets: &[String]) -> Result<(), String> {
    if targets.is_empty() {
        return Err("targets must name at least one id".to_string());
    }
    match targets.iter().enumerate().find(|(at, id)| targets[..*at].contains(id)) {
        Some((_, id)) => Err(format!("targets must not repeat {id:?}")),
        None => Ok(()),
    }
}

/// 🔢️ A selection label's number as `(en, de)`: at most two decimals, trailing zeros and a negative zero
/// dropped, a decimal point in English and a decimal comma in German.
pub fn puzzle3d_selection_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 📐️ A selection label's triple as `(en, de)`: `(1, 0, -2.5)` in English, `(1; 0; -2,5)` in German.
pub fn puzzle3d_selection_triple(values: [f64; 3]) -> (String, String) {
    let parts = values.map(puzzle3d_selection_number);
    (format!("({}, {}, {})", parts[0].0, parts[1].0, parts[2].0), format!("({}; {}; {})", parts[0].1, parts[1].1, parts[2].1))
}

/// 🔠️ A selection label's counted noun, `(en, de)`: "1 item" / "1 Element", "3 items" / "3 Elemente".
pub fn puzzle3d_selection_items(count: usize) -> (String, String) {
    match count {
        1 => ("1 item".to_string(), "1 Element".to_string()),
        count => (format!("{count} items"), format!("{count} Elemente")),
    }
}

/// ✖️ The Hamilton product `a · b` of two `[x, y, z, w]` quaternions — `a` applied after `b`.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

/// 🧭️ The `[x, y, z, w]` quaternion turning `angle` radians about the axis `(ax, ay, az)`; the identity
/// for an axis shorter than `1e-8`.
pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

/// 📏️ A pose scale multiplied per axis by `factors`, always as a per-axis triple: a uniform scalar
/// broadcasts first and an absent scale reads as `[1, 1, 1]`.
pub fn puzzle3d_scaled(scale: Option<crate::Puzzle3dScale>, factors: [f64; 3]) -> crate::Puzzle3dScale {
    let current = match scale {
        Some(crate::Puzzle3dScale::Uniform(value)) => [value; 3],
        Some(crate::Puzzle3dScale::Vec3(value)) => value,
        None => [1.0; 3],
    };
    crate::Puzzle3dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
}
//#endregion 🔖️SelectionTransform

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
                    mutations.push(replace_attraction_geometry(ReplaceAttractionGeometry { id: attraction.id.clone(), new_gap: attraction.gap, new_shift: attraction.shift, new_rise: attraction.rise, new_rotation: attraction.rotation, new_turn: attraction.turn, new_tilt: attraction.tilt, new_x: attraction.x, new_y: attraction.y }));
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
        // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
        // through `dsl::DslValue`/`dsl::ToValue`/`dsl::FromValue` instead of
        // `serde_json::from_value`/`to_value` on `Puzzle3dSnapshot` directly — that type only
        // derives `Serialize`/`Deserialize` under `#[cfg(test)]` now. `Value` (this bridge's own
        // boundary type) is untouched.
        let base: Puzzle3dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(projection)).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle3dSnapshot>::apply(self, &base).map_err(|error| error.under(["document"]))?;
        Ok(Value::from(dsl::ToValue::to_value(&next)))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle3dMutation {
    type Diff = Puzzle3dDiff;

    /// 🧷️ `#[derive(dsl::Mutations)]` on the enum only generates `impl Mutation<Puzzle3dSnapshot>`
    /// (the `#[mutations(snapshot = ...)]` type); this bridge `impl Mutation<Value>` is hand-written
    /// and forwards here too, same as every other method in this impl.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle3dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMAS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle3dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> dsl::DslValue {
        Mutation::<Puzzle3dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Mutation::<Puzzle3dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        Mutation::<Puzzle3dSnapshot>::descriptor(self)
    }

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle3dDiff> {
        let base: Puzzle3dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(projection)).unwrap_or_default();
        Mutation::<Puzzle3dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        let base: Puzzle3dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(projection)).unwrap_or_default();
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
    let before_snapshot: Puzzle3dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(before)).unwrap_or_default();
    let after_snapshot: Puzzle3dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(after)).unwrap_or_default();
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
    ///
    /// 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
    /// through `dsl::DslValue`/`dsl::FromValue` instead of `serde_json::from_value` —
    /// `Puzzle3dSnapshot` only derives `Deserialize` under `#[cfg(test)]` now.
    pub fn new(value: Value) -> Self {
        let typed = dsl::FromValue::from_value(dsl::DslValue::from(&value)).unwrap_or_default();
        let projected = std::sync::OnceLock::new();
        let _ = projected.set(std::sync::Arc::new(value));
        Self { typed: std::sync::Arc::new(typed), value: projected }
    }

    /// 🧬️ Keeps mutation application typed and defers the JSON bridge until a reader needs it.
    fn from_typed(typed: Puzzle3dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ Materializes the legacy play projection at most once per immutable snapshot.
    ///
    /// 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
    /// through `dsl::ToValue`/`dsl::DslValue` instead of `serde_json::to_value` —
    /// `Puzzle3dSnapshot` only derives `Serialize` under `#[cfg(test)]` now.
    pub fn value(&self) -> &Value {
        self.value.get_or_init(|| std::sync::Arc::new(Value::from(dsl::ToValue::to_value(self.typed.as_ref())))).as_ref()
    }

    /// 🧬️ Exposes the immutable typed authority without materializing the legacy JSON projection.
    pub fn typed(&self) -> &Puzzle3dSnapshot {
        self.typed.as_ref()
    }

    /// 🧬️ Shares the immutable typed authority — what a tool request holds without copying the document.
    pub fn typed_arc(&self) -> std::sync::Arc<Puzzle3dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
}

/// 🩹️ Hand-written, not derived (ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `ArtifactEditor::Snapshot`
/// (see `✏️editor/🦀️.rs`'s `type Snapshot = Puzzle3dPlaySnapshot`) requires `ToValue + FromValue`;
/// there is no field-wise derive shape for this struct's `Arc<Puzzle3dSnapshot>`/
/// `OnceLock<Arc<Value>>` split, so this bridges through the same materialized `Value` projection
/// `value()`/`new()` already maintain.
impl dsl::ToValue for Puzzle3dPlaySnapshot {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::from(self.value())
    }
}

impl dsl::FromValue for Puzzle3dPlaySnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Ok(Self::new(Value::from(value)))
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

/// 🧒️ Composition view of the play snapshot: a puzzle document owns no child artifacts, so the
/// typed snapshot's own (empty) composition is the whole answer.
impl semio_framework_schema::ArtifactCompositionFields for Puzzle3dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema::ArtifactCompositionFields::visit_child_refs(self.typed.as_ref(), visitor)
    }
    fn child_slots() -> &'static [semio_framework_schema::ChildSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema::ArtifactCompositionFields>::child_slots()
    }
    fn link_slots() -> &'static [semio_framework_schema::LinkSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema::ArtifactCompositionFields>::link_slots()
    }
}

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle3dSnapshot`'s derived record
/// layout and pack-schema identity.
impl store::ArtifactPack for Puzzle3dPlaySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle3dSnapshot as store::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        <Puzzle3dSnapshot as store::ArtifactPack>::record_spec()
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

    /// 🧷️ Same bridge shape as `impl Mutation<Value>` above: the derive only covers
    /// `Mutation<Puzzle3dSnapshot>`, so this hand-written impl forwards its descriptor metadata
    /// there too.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle3dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMAS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle3dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> dsl::DslValue {
        Mutation::<Puzzle3dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Mutation::<Puzzle3dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        Mutation::<Puzzle3dSnapshot>::descriptor(self)
    }

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
    fn label(&self) -> protocol::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
