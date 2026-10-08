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
    parts: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle5dPartModification>,
    volumes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle5dTargetVolumeModification>,
) -> protocol::MutationOutcome<Puzzle5dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dPartsDelta, Puzzle5dTargetVolumesDelta};
    if parts.is_empty() && volumes.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages(selection.warnings.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.to_vec())]));
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: (!parts.is_empty()).then(|| Puzzle5dPartsDelta { modified: parts, ..Default::default() }),
        target_volumes: (!volumes.is_empty()).then(|| Puzzle5dTargetVolumesDelta { modified: volumes, ..Default::default() }),
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



pub fn inverse_puzzle5d_mutation(projection: &Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}





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
