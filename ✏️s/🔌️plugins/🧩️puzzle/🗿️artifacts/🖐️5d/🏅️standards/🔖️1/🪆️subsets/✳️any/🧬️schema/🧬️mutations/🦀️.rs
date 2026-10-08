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
use protocol::{DiffAlgebra, Mutation, MutationDiff};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-5d document mutation vocabulary: id-keyed part create-delete plus per-2d/
/// per-3d-projection field edits, grip membership, a grip-to-grip fastener connect/disconnect
/// relationship, and document-level edits (label rename, domain/description change,
/// kind-compatibility connect/disconnect, kind-catalog replace), and the four parametric selection
/// transforms (`drag-selection2d` on the board, `drag-`, `rotate-`, `scale-selection3d` in the world) that
/// record a gesture's own inputs. There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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
    CreateTargetVolume(CreateTargetVolume),
    DeleteTargetVolume(DeleteTargetVolume),
    MoveTargetVolume(MoveTargetVolume),
    RotateTargetVolume(RotateTargetVolume),
    ScaleTargetVolume(ScaleTargetVolume),
    ChangeTargetVolumeHidden(ChangeTargetVolumeHidden),
    ChangeTargetVolumeLocked(ChangeTargetVolumeLocked),
    DragSelection2d(DragSelection2d),
    DragSelection3d(DragSelection3d),
    RotateSelection3d(RotateSelection3d),
    ScaleSelection3d(ScaleSelection3d),
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
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "drag-selection2d",
    "drag-selection3d",
    "rotate-selection3d",
    "scale-selection3d",
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
pub use super::change_target_volume_hidden::{change_target_volume_hidden, ChangeTargetVolumeHidden};
pub use super::change_target_volume_locked::{change_target_volume_locked, ChangeTargetVolumeLocked};
pub use super::change_part_kind::{change_part_kind, ChangePartKind};
pub use super::connect_grips::{connect_grips, ConnectGrips};
pub use super::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::create_part::{create_part, CreatePart};
pub use super::create_target_volume::{create_target_volume, CreateTargetVolume};
pub use super::delete_part::{delete_part, DeletePart};
pub use super::delete_target_volume::{delete_target_volume, DeleteTargetVolume};
pub use super::disconnect_grips::{disconnect_grips, DisconnectGrips};
pub use super::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::drag_selection_2d::{drag_selection_2d, DragSelection2d};
pub use super::drag_selection_3d::{drag_selection_3d, DragSelection3d};
pub use super::edit_part_2d_text::{edit_part_2d_text, EditPart2dText};
pub use super::edit_part_3d_label::{edit_part_3d_label, EditPart3dLabel};
pub use super::move_part_2d::{move_part_2d, MovePart2d};
pub use super::move_part_3d::{move_part_3d, MovePart3d};
pub use super::move_target_volume::{move_target_volume, MoveTargetVolume};
pub use super::remove_part_grip::{remove_part_grip, RemovePartGrip};
pub use super::rename_puzzle5d::{rename_puzzle5d, RenamePuzzle5d};
pub use super::replace_fastener_geometry::{replace_fastener_geometry, ReplaceFastenerGeometry};
pub use super::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_part_2d_geometry::{replace_part_2d_geometry, ReplacePart2dGeometry};
pub use super::replace_part_grip::{replace_part_grip, ReplacePartGrip};
pub use super::rotate_part_3d::{rotate_part_3d, RotatePart3d};
pub use super::rotate_selection_3d::{rotate_selection_3d, RotateSelection3d};
pub use super::rotate_target_volume::{rotate_target_volume, RotateTargetVolume};
pub use super::scale_part_3d::{scale_part_3d, ScalePart3d};
pub use super::scale_selection_3d::{scale_selection_3d, ScaleSelection3d};
pub use super::scale_target_volume::{scale_target_volume, ScaleTargetVolume};
pub use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_items as puzzle5d_selection_items,puzzle3d_selection_number as puzzle5d_selection_number,puzzle3d_selection_triple as puzzle5d_selection_triple,puzzle3d_targets_invariant as puzzle5d_targets_invariant,quat_from_axis_angle,quat_mul};


//#region 🔖️SelectionTransform
/// 🎛️ The ONE board↔world scale this artifact places and moves paired parts with — the linear inverse of
/// `🧬️schema/💡️inferences/🎛️flat-position`'s plan projection, so a flat point and a world origin stay one
/// consistent pair whichever pane the gesture came from. A flat unit is one board pixel; 48 of them make
/// one world metre (the board's own default part box, `part_2d.width`/`height`).
pub const PUZZLE5D_FLAT_TO_WORLD: f64 = 1.0 / 48.0;

/// 🧭️ The members of a parametric selection leaf's target set the transform acts on, in document order, with the
/// `mutation.partial` warnings of the ones it skips. `targets` is classified by document membership against `base`: a
/// part id goes through `parts` (its board projection for a 2d leaf, its world projection for a 3d one), a
/// target-volume id through `volumes` (none for the board, which paints no volume), each record transformed IN PLACE.
/// An empty or repeated target set is the Fatal `mutation.invariant` the payload schema's `minItems`/`uniqueItems`
/// forbid. Absent ids, locked records (a part's `2d.locked`) and records the leaf does not reach are skipped with one
/// warning per reason (in that order, ids in payload order); nothing left is `mutation.target-missing`.
pub struct Puzzle5dSelection<'a> {
    pub parts: Vec<&'a crate::Puzzle5dPart>,
    pub volumes: Vec<&'a crate::Puzzle5dTargetVolume>,
    pub warnings: Vec<protocol::MutationMessage>,
}

/// 🗃️ Classifies a selection leaf's `targets` against `base`; `Err` is the leaf's final refusal outcome.
pub fn puzzle5d_selection<'a>(base: &'a Puzzle5dSnapshot, targets: &[String], volumes_reached: bool) -> Result<Puzzle5dSelection<'a>, protocol::MutationOutcome<Puzzle5dDiff>> {
    if let Err(reason) = puzzle5d_targets_invariant(targets) {
        return Err(protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec()));
    }
    let (mut missing, mut locked, mut unreached, mut survivors) = (Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), std::collections::BTreeSet::<&str>::new());
    for id in targets {
        match (base.parts.iter().find(|entry| &entry.id == id), base.target_volumes.iter().find(|entry| &entry.id == id)) {
            (Some(entry), _) if entry.part_2d.locked != Some(true) => {
                survivors.insert(id.as_str());
            }
            (Some(_), _) => locked.push(id.clone()),
            (None, Some(entry)) if entry.locked => locked.push(id.clone()),
            (None, Some(_)) if !volumes_reached => unreached.push(id.clone()),
            (None, Some(_)) => {
                survivors.insert(id.as_str());
            }
            (None, None) => missing.push(id.clone()),
        }
    }
    if survivors.is_empty() {
        return Err(protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an unlocked part or target volume this transform reaches", targets.len()), targets.to_vec()));
    }
    let warnings = [(missing, "not in this puzzle"), (locked, "locked"), (unreached, "target volumes live in the world, not on the board")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    Ok(Puzzle5dSelection {
        parts: base.parts.iter().filter(|entry| survivors.contains(entry.id.as_str())).collect(),
        volumes: base.target_volumes.iter().filter(|entry| volumes_reached && survivors.contains(entry.id.as_str())).collect(),
        warnings,
    })
}

/// 📦️ The outcome of a selection leaf's per-member patches: the sparse diff plus the classification warnings, or the
/// `mutation.no-op` warning (after them) when no surviving member changes.
pub fn puzzle5d_selection_outcome(
    selection: Puzzle5dSelection<'_>,
    targets: &[String],
    parts: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle5dPartPatchEntry>,
    volumes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle5dTargetVolumePatchEntry>,
) -> protocol::MutationOutcome<Puzzle5dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dPartsDelta, Puzzle5dTargetVolumesDelta};
    if parts.is_empty() && volumes.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages(selection.warnings.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.to_vec())]));
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: (!parts.is_empty()).then(|| Puzzle5dPartsDelta { patched: parts, ..Default::default() }),
        target_volumes: (!volumes.is_empty()).then(|| Puzzle5dTargetVolumesDelta { patched: volumes, ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(selection.warnings)
}

/// 📏️ A pose scale multiplied per axis by `factors`, always as a per-axis triple: a uniform scalar broadcasts
/// first and an absent scale reads as `[1, 1, 1]`.
pub fn puzzle5d_scaled(scale: Option<crate::Puzzle5dScale>, factors: [f64; 3]) -> crate::Puzzle5dScale {
    let current = match scale {
        Some(crate::Puzzle5dScale::Uniform(value)) => [value; 3],
        Some(crate::Puzzle5dScale::Vec3(value)) => value,
        None => [1.0; 3],
    };
    crate::Puzzle5dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
}
//#endregion 🔖️SelectionTransform

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
                fastener.y, None,
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
                    fastener.y, None,
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
            None => mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None)),
            Some(prior) if prior != row => {
                mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None));
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

pub fn inverse_puzzle5d_mutation(projection: &Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
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
    let catalogs: crate::Puzzle5dKindCatalogs = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&catalogs_value)).unwrap_or_default();
    let (handle, extra) = crate::split_and_seed_kind_catalogs(Some(catalogs));
    object.insert("kindCatalogs".into(), Value::from(&semio_framework_value::ToValue::to_value(&handle)));
    object.insert("kindCatalogsExtra".into(), Value::from(&semio_framework_value::ToValue::to_value(&extra)));
    value
}

impl MutationDiff<Value> for Puzzle5dDiff {
    fn apply(&self, projection: &Value, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Value> {
        // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
        // through `dsl::DslValue`/`dsl::ToValue`/`dsl::FromValue` instead of
        // `serde_json::from_value`/`to_value` on `Puzzle5dSnapshot` directly — that type only
        // derives `Serialize`/`Deserialize` under `#[cfg(test)]` now. `Value` (this bridge's own
        // boundary type) and `normalize_kind_catalogs_for_snapshot_value` are untouched — this
        // call did not route through that helper before this change either, preserved as-is.
        let base: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(projection)).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle5dSnapshot>::apply(self, &base, capability).map_err(|error| error.under(["document"]))?;
        Ok(Value::from(semio_framework_value::ToValue::to_value(&next)))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Value> for Puzzle5dDiff {
    fn inverse(&self, base: &Value) -> Self {
        let base: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(base)).unwrap_or_default();
        DiffAlgebra::<Puzzle5dSnapshot>::inverse(self, &base)
    }
    fn between(base: &Value, other: &Value) -> Self {
        let base: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(base)).unwrap_or_default();
        let other: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(other)).unwrap_or_default();
        <Self as DiffAlgebra<Puzzle5dSnapshot>>::between(&base, &other)
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle5dSnapshot>::is_empty(self)
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
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle5dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle5dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle5dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle5dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle5dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle5dDiff> {
        let base: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(projection))).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        let base: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(projection))).unwrap_or_default();
        Mutation::<Puzzle5dSnapshot>::inverse(self, &base)?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle5dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle5dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle5dSnapshot>::conflict_target(self)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare document JSON the play app mutates), by round-tripping through the typed
/// `Puzzle5dSnapshot` and delegating to [`puzzle5d_snapshot_mutations`].
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dMutation> {
    let before_snapshot: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(before))).unwrap_or_default();
    let after_snapshot: Puzzle5dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(after))).unwrap_or_default();
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    puzzle5d_snapshot_mutations(&before_snapshot, &after_snapshot)
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlaySnapshot
/// 🌱️ The play app's `Puzzle5dPlayApp` scene helpers still read the ad-hoc `serde_json::Value` fixture
/// shape, while every Store mutation is TYPED. This snapshot keeps the typed `Puzzle5dSnapshot` as the one
/// authority and materializes the legacy `Value` projection lazily, at most once per immutable root —
/// the shape `🧊️3d`'s `Puzzle3dPlaySnapshot` already has, kept identical on purpose.
///
/// 🐛️ It used to BE the `Value` (`Puzzle5dPlaySnapshot(pub Value)`), so every one-item Store preparation
/// (`Puzzle5dStorePreparation`, `✏️editor/🦀️.rs`) decoded the WHOLE document into `Puzzle5dSnapshot` for
/// `inverse`, again for `diff`, a third time inside `apply`, and re-encoded the post root back to `Value`:
/// four whole-document conversions per folded mutation. `capsule-dream`'s switch folds ~5 700 mutations
/// into a document that grows to 2 880 parts, so the publication was O(n²) — measured 2026-09-23 natively
/// (`sample`: `Puzzle5dStorePreparation` → `Puzzle5dSnapshot::from_value` → `Vec<Puzzle5dPart>::from_value`
/// hottest) and on the live pane as a per-turn guest cost climbing 66 → 227 ms over eight minutes
/// (`📓️block-puzzle.md` §11).
#[derive(Debug)]
pub struct Puzzle5dPlaySnapshot {
    typed: std::sync::Arc<Puzzle5dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<Value>>,
}

impl Puzzle5dPlaySnapshot {
    /// 🎯️ Builds the typed authority once from a legacy projection and retains that projection.
    pub fn new(value: Value) -> Self {
        let typed = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&normalize_kind_catalogs_for_snapshot_value(&value))).unwrap_or_default();
        let projected = std::sync::OnceLock::new();
        let _ = projected.set(std::sync::Arc::new(value));
        Self { typed: std::sync::Arc::new(typed), value: projected }
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub(crate) fn from_typed(typed: Puzzle5dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ The legacy play projection, materialized at most once per immutable snapshot.
    pub fn value(&self) -> &Value {
        self.value.get_or_init(|| std::sync::Arc::new(Value::from(semio_framework_value::ToValue::to_value(self.typed.as_ref())))).as_ref()
    }

    /// 🧬️ The typed authority, without materializing the legacy projection.
    pub fn typed(&self) -> &Puzzle5dSnapshot {
        self.typed.as_ref()
    }

    /// 🧷️ The typed authority shared, never copied — the base a tool transaction yields against.
    pub fn typed_arc(&self) -> std::sync::Arc<Puzzle5dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
}

impl Clone for Puzzle5dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle5dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🩹️ Hand-written: `ArtifactEditor::Snapshot` needs `ToValue + FromValue`, and this struct's typed/lazy
/// split has no field-wise derive shape, so both bridge through the `Value` projection `value()`/`new()`
/// maintain.
impl semio_framework_value::ToValue for Puzzle5dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(self.typed())
    }
}

impl semio_framework_value::FromValue for Puzzle5dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Puzzle5dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed)
    }
}



/// 🧒️ Visits the literal persisted child owned by the typed Puzzle5d parent.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle5dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ArtifactCompositionFields::visit_child_refs(self.typed(),visitor)
    }
}



impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dPlaySnapshot> {
        MutationDiff::<Puzzle5dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle5dPlaySnapshot::from_typed).map_err(|error| error.under(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn inverse(&self, base: &Puzzle5dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle5dSnapshot>::inverse(self, base.typed())
    }
    fn between(base: &Puzzle5dPlaySnapshot, other: &Puzzle5dPlaySnapshot) -> Self {
        <Self as DiffAlgebra<Puzzle5dSnapshot>>::between(base.typed(), other.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle5dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle5dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle5dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle5dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle5dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle5dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle5dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle5dPlaySnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        Mutation::<Puzzle5dSnapshot>::diff(self, projection.typed())
    }

    fn inverse(&self, projection: &Puzzle5dPlaySnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle5dSnapshot>::inverse(self, projection.typed())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle5dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle5dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle5dSnapshot>::conflict_target(self)
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
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
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
#[cfg(test)]
#[path = "🧪️tests/🧪️selection-time-travel/🦀️.rs"]
mod selection_time_travel;

#[cfg(test)]
#[path = "🧪️tests/🔬️committed-fixtures/🦀️.rs"]
mod committed_fixtures;
//#endregion 🧪️Tests
