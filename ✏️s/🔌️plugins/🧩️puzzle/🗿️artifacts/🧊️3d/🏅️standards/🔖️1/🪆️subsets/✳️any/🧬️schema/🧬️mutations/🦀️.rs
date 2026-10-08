//! 🧬️ Puzzle 3d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle3dSnapshot>` and `impl protocol::SemanticMutation<Puzzle3dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The native editor owns its immutable play root; semantic mutation authority remains the
//! canonical typed snapshot and its typed diff algebra.

use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{DiffAlgebra, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-3d document mutation vocabulary: id-keyed object/target-volume/reference
/// create-delete plus per-field edits, vortex membership, a vortex-to-vortex attraction connect/
/// disconnect relationship, and document-level edits (domain change, kind-compatibility connect/
/// disconnect, kind-catalog replace), and the three parametric selection transforms (`drag-`,
/// `rotate-`, `scale-selection`) that record a gesture's own inputs. There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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
/// 🧭️ The members of a parametric selection leaf's target set the transform acts on, in document order, with the
/// `mutation.partial` warnings of the ones it skips. `targets` is classified by document membership against `base`:
/// an object id goes through `objects`, a target-volume id through `volumes`, each record transformed IN PLACE about
/// its own origin. An empty or repeated target set is the Fatal `mutation.invariant` the payload schema's
/// `minItems`/`uniqueItems` forbid. Absent ids and locked records are skipped with one warning per reason (in that
/// order, ids in payload order); nothing left is `mutation.target-missing`.
pub struct Puzzle3dSelection<'a> {
    pub objects: Vec<&'a crate::Puzzle3dObject>,
    pub volumes: Vec<&'a crate::Puzzle3dTargetVolume>,
    pub warnings: Vec<protocol::MutationMessage>,
}

/// 🗃️ Classifies a selection leaf's `targets` against `base`; `Err` is the leaf's final refusal outcome.
pub fn puzzle3d_selection<'a>(base: &'a Puzzle3dSnapshot, targets: &[String]) -> Result<Puzzle3dSelection<'a>, protocol::MutationOutcome<Puzzle3dDiff>> {
    if let Err(reason) = puzzle3d_targets_invariant(targets) {
        return Err(protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec()));
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
        return Err(protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an unlocked object or target volume", targets.len()), targets.to_vec()));
    }
    let warnings = [(missing, "not in this scene"), (locked, "locked")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    Ok(Puzzle3dSelection {
        objects: base.objects.iter().filter(|entry| survivors.contains(entry.id.as_str())).collect(),
        volumes: base.target_volumes.iter().filter(|entry| survivors.contains(entry.id.as_str())).collect(),
        warnings,
    })
}

/// 📦️ The outcome of a selection leaf's per-member patches: the sparse diff plus the classification warnings and, when
/// objects followed or attractions were re-derived, one Info-level `mutation.cascade` naming both — or the
/// `mutation.no-op` warning (after the classification ones) when nothing changes.
pub fn puzzle3d_selection_outcome(
    selection: Puzzle3dSelection<'_>,
    targets: &[String],
    solved: Puzzle3dSelectionFollow,
    base: &Puzzle3dSnapshot,
    volumes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle3dTargetVolumeModification>,
) -> protocol::MutationOutcome<Puzzle3dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionsDelta, Puzzle3dObjectPatch, Puzzle3dObjectModification, Puzzle3dObjectsDelta, Puzzle3dTargetVolumesDelta};
    use protocol::list_delta::RowPatch;
    let mut messages = selection.warnings;
    let objects: Vec<Puzzle3dObjectModification> = base
        .objects
        .iter()
        .zip(&solved.poses)
        .filter_map(|(entry, pose)| {
            let pose = pose.as_ref()?;
            let patch = Puzzle3dObjectPatch {
                origin: (pose.origin != entry.origin).then_some(pose.origin),
                orientation: (pose.orientation != entry.orientation).then_some(pose.orientation),
                scale: (pose.scale != entry.scale).then_some(pose.scale),
                ..Default::default()
            };
            (!patch.is_empty()).then(|| Puzzle3dObjectModification { id: entry.id.clone(), patch })
        })
        .collect();
    let attractions = solved.attractions;
    if objects.is_empty() && volumes.is_empty() && attractions.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.to_vec())]));
    }
    if !solved.followers.is_empty() || !attractions.is_empty() {
        let cascade = solved.followers.iter().cloned().chain(attractions.iter().map(|entry| entry.id.clone())).collect::<Vec<_>>();
        messages.push(protocol::MutationMessage::info("mutation.cascade", format!("{} attracted object(s) followed, {} attraction(s) re-derived", solved.followers.len(), attractions.len())).at(cascade));
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: (!objects.is_empty()).then(|| Puzzle3dObjectsDelta { modified: objects, ..Default::default() }),
        target_volumes: (!volumes.is_empty()).then(|| Puzzle3dTargetVolumesDelta { modified: volumes, ..Default::default() }),
        attractions: (!attractions.is_empty()).then(|| Puzzle3dAttractionsDelta { modified: attractions, ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(messages)
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

//#region 🔖️AttractionPose
/// ⭕️ The quaternion of no rotation, `[x, y, z, w]` — what a record without an orientation stands at.
pub const PUZZLE3D_IDENTITY_QUATERNION: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

/// 🪡️ Below this cross length an attracted vortex counts as (anti)parallel to its attracting one — the compose
/// kernel's own alignment tolerance.
const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

/// 🧭️ The pose fields a selection transform may change on one object — its world origin, orientation and scale.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dPose {
    pub origin: [f64; 3],
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<crate::Puzzle3dScale>,
}

/// 🧾️ What one selection leaf's attraction re-solve moves, in document order: per object its new pose (`None`
/// for an unmoved object), the ids of the objects that only FOLLOWED, and the re-derived connection parameters of
/// every attraction whose parameters change.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle3dSelectionFollow {
    pub poses: Vec<Option<Puzzle3dPose>>,
    pub followers: Vec<String>,
    pub attractions: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle3dAttractionModification>,
}

/// 🌲️ Re-solves the attraction graph of one selection move on `base`, with the document's own placement kernel
/// ([`puzzle3d_attraction_child_pose`], the one the editor's resolve runs). Each surviving object target (payload
/// order) takes its `pose` transform; with `follow`, a breadth-first walk then re-places every UNLOCKED object an
/// attraction hangs off a moved object (`attracting → attracted`, attractions in document order, first visit wins)
/// from its moved parent and the attraction's unchanged parameters, so a moved attracting object carries its whole
/// subtree exactly as resolving would. A locked object never follows, and neither does what hangs off it. Every
/// other attraction touching a moved object gets its six connection parameters re-derived from the moved poses
/// ([`derive_attraction_params`]), so resolving the document afterwards never snaps a moved object back. Without
/// `follow` (a scaling) nothing follows and no attraction changes.
pub fn puzzle3d_selection_follow(base: &Puzzle3dSnapshot, targets: &[String], selection: &Puzzle3dSelection<'_>, pose: &dyn Fn(&crate::Puzzle3dObject) -> Puzzle3dPose, follow: bool) -> Puzzle3dSelectionFollow {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionPatch, Puzzle3dAttractionModification};
    let survivors: std::collections::BTreeSet<&str> = selection.objects.iter().map(|entry| entry.id.as_str()).collect();
    let mut solved = Puzzle3dSelectionFollow { poses: vec![None; base.objects.len()], ..Default::default() };
    let mut queue = std::collections::VecDeque::new();
    for id in targets.iter().filter(|id| survivors.contains(id.as_str())) {
        let Some(at) = base.objects.iter().position(|entry| &entry.id == id) else { continue };
        if solved.poses[at].is_none() {
            solved.poses[at] = Some(pose(&base.objects[at]));
            queue.push_back(at);
        }
    }
    if !follow {
        return solved;
    }
    let ports: std::collections::HashMap<String, (usize, usize)> = base.objects.iter().enumerate().flat_map(|(at, entry)| entry.vortices.iter().enumerate().map(move |(port, vortex)| (puzzle3d_vortex_full_id(&entry.id, &vortex.id), (at, port)))).collect();
    let ends: Vec<Option<((usize, usize), (usize, usize))>> = base.attractions.iter().map(|attraction| Some((*ports.get(&attraction.attracting)?, *ports.get(&attraction.attracted)?)).filter(|(from, to)| from.0 != to.0)).collect();
    let mut placing = vec![false; base.attractions.len()];
    while let Some(parent) = queue.pop_front() {
        for (index, ends) in ends.iter().enumerate() {
            let Some((from, to)) = *ends else { continue };
            if from.0 != parent || solved.poses[to.0].is_some() || base.objects[to.0].locked {
                continue;
            }
            let (Some(attracting), attraction, attracted) = (solved.poses[parent].as_ref(), &base.attractions[index], &base.objects[to.0]) else { continue };
            let (source, target) = (&base.objects[parent].vortices[from.1], &attracted.vortices[to.1]);
            let (origin, orientation) = puzzle3d_attraction_child_pose(
                attracting.origin,
                attracting.orientation.unwrap_or(PUZZLE3D_IDENTITY_QUATERNION),
                source.position,
                source.direction.unwrap_or([0.0, 0.0, -1.0]),
                target.position,
                target.direction.unwrap_or([0.0, 0.0, -1.0]),
                attraction.gap,
                attraction.shift,
                attraction.rise,
                attraction.rotation,
                attraction.turn,
                attraction.tilt,
            );
            solved.poses[to.0] = Some(Puzzle3dPose { origin, orientation: Some(orientation), scale: attracted.scale });
            solved.followers.push(attracted.id.clone());
            placing[index] = true;
            queue.push_back(to.0);
        }
    }
    let current = |at: usize| solved.poses[at].clone().unwrap_or_else(|| Puzzle3dPose { origin: base.objects[at].origin, orientation: base.objects[at].orientation, scale: base.objects[at].scale });
    solved.attractions = base
        .attractions
        .iter()
        .zip(&ends)
        .zip(&placing)
        .filter_map(|((attraction, ends), placing)| {
            let (from, to) = (*ends)?;
            if *placing || solved.poses[from.0].is_none() && solved.poses[to.0].is_none() {
                return None;
            }
            let (attracting, attracted) = (current(from.0), current(to.0));
            let (source, target) = (&base.objects[from.0].vortices[from.1], &base.objects[to.0].vortices[to.1]);
            let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                attracting.origin,
                attracting.orientation.unwrap_or(PUZZLE3D_IDENTITY_QUATERNION),
                source.position,
                source.direction.unwrap_or([0.0, 0.0, -1.0]),
                target.position,
                target.direction.unwrap_or([0.0, 0.0, -1.0]),
                attracted.origin,
                attracted.orientation.unwrap_or(PUZZLE3D_IDENTITY_QUATERNION),
            );
            let patch = Puzzle3dAttractionPatch {
                gap: (gap != attraction.gap).then_some(gap),
                shift: (shift != attraction.shift).then_some(shift),
                rise: (rise != attraction.rise).then_some(rise),
                rotation: (rotation != attraction.rotation).then_some(rotation),
                turn: (turn != attraction.turn).then_some(turn),
                tilt: (tilt != attraction.tilt).then_some(tilt),
                ..Default::default()
            };
            (patch != Puzzle3dAttractionPatch::default()).then(|| Puzzle3dAttractionModification { id: attraction.id.clone(), patch })
        })
        .collect();
    solved
}

/// 🔗️ The full id an attraction endpoint names one vortex by: `<objectId>:<vortexId>`, or the vortex id itself
/// when it already is a full id.
pub fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

/// ➖️ `a − b`.
pub fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ➕️ `a + b`.
pub fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ✳️ `a · s`.
pub fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// ❎️ The cross product `a × b`.
pub fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// ⚫️ The dot product `a · b`.
pub fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 🦯️ The Euclidean length of `a`.
pub fn vec3_len(a: [f64; 3]) -> f64 {
    vec3_dot(a, a).sqrt()
}

/// 🎱️ `a` scaled to unit length; a vector shorter than `1e-12` stays as it is.
pub fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
    let len = vec3_len(a);
    if len < 1e-12 {
        a
    } else {
        vec3_scale(a, 1.0 / len)
    }
}

/// 🔁️ Degrees to radians.
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

/// 🔂️ Radians to degrees.
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

/// 🪞️ The conjugate of `q` — its inverse for a unit quaternion.
pub fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

/// 🧼️ `q` scaled to unit length; a degenerate quaternion reads as the identity.
pub fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-12 {
        PUZZLE3D_IDENTITY_QUATERNION
    } else {
        [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
    }
}

/// 🌐️ `vector` turned by the unit quaternion `quat` (`q · v · q⁻¹`).
pub fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

/// 🎏️ The quaternion rotating unit vector `from` onto unit vector `to`.
pub fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = vec3_dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = vec3_cross(from, to);
        [c[0], c[1], c[2], r]
    };
    quat_normalize(quat)
}

/// 🧲️ The align-quaternion special case for when the attracted vortex is already (anti)parallel to the
/// attracting vortex. Falls back to an alternate cross axis when the attracting direction is exactly
/// ±Z — a double-degenerate corner the compose kernel's own branch doesn't otherwise guard.
pub fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
    let reverse_child = vec3_scale(child_dir, -1.0);
    let cross_vec = vec3_cross(parent_dir, reverse_child);
    if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
        if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
            if vec3_len(axis) < 1e-9 {
                axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
            }
            let axis = vec3_normalize(axis);
            let half = std::f64::consts::FRAC_PI_2;
            quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
        }
    } else {
        puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
    }
}

/// 📐️ Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both
/// vortices' LOCAL position/direction, and the 6 connection-style parameters (angles in degrees),
/// returns the attracted object's world pose.
#[allow(clippy::too_many_arguments)]
pub fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
    let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
    let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
    let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
    let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

    let mut orientation_local = quat_conjugate(align_q);
    orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
    let orientation_local = quat_normalize(orientation_local);

    let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
    let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
    let q_b = quat_normalize(quat_mul(orientation_local, q_a));
    (t_b, q_b)
}

/// 🔙️ Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose,
/// derives the 6 parameters that reproduce it exactly, so moving/rotating an attracted object never
/// causes a resolve-triggered snap-back and creating an attraction never moves either endpoint.
#[allow(clippy::too_many_arguments)]
pub fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

    let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
    let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
    let gap = vec3_dot(diff, gap_dir);
    let shift = vec3_dot(diff, shift_dir);
    let rise = vec3_dot(diff, raise_dir);

    let residual = quat_mul(align_q, orientation_local);
    let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
    let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
    let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

    let clamp = |v: f64| v.clamp(-1.0, 1.0);
    let tilt_rad = -(clamp(col_y[2])).asin();
    let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
        (col_x[1].atan2(col_x[0]), 0.0)
    } else {
        let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
        ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
    };

    (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
}
//#endregion 🔖️AttractionPose



pub fn inverse_puzzle3d_mutation(projection: &Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
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
//#endregion 🧪️Tests
