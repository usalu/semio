//! 👉️ and the selection, outcome and attraction-follow helpers they share.
//! ✋️ Puzzle3d selection mutations — the three parametric selection transforms (`drag-`, `rotate-`, `scale-selection`)

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use protocol::list_delta::RowPatch;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_s_artifact_puzzle_3d_model::PUZZLE3D_IDENTITY_QUATERNION;
use semio_s_artifact_puzzle_3d_model::Puzzle3dPose;
use semio_s_artifact_puzzle_3d_model::derive_attraction_params;
use semio_s_artifact_puzzle_3d_model::puzzle3d_attraction_child_pose;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;

/// 🧭️ The members of a parametric selection leaf's target set the transform acts on, in document order, with the
/// `mutation.partial` warnings of the ones it skips. `targets` is classified by document membership against `base`:
/// an object id goes through `objects`, a target-volume id through `volumes`, each record transformed IN PLACE about
/// its own origin. An empty or repeated target set is the Fatal `mutation.invariant` the payload schema's
/// `minItems`/`uniqueItems` forbid. Absent ids and locked records are skipped with one warning per reason (in that
/// order, ids in payload order); nothing left is `mutation.target-missing`.
pub struct Puzzle3dSelection<'a> {
    pub objects: Vec<&'a semio_s_artifact_puzzle_3d_model::Puzzle3dObject>,
    pub volumes: Vec<&'a semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume>,
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
    volumes: Vec<semio_s_artifact_puzzle_3d_diff::Puzzle3dTargetVolumeModification>,
) -> protocol::MutationOutcome<Puzzle3dDiff> {
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dAttractionsDelta;
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectPatch;
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectModification;
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectsDelta;
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dTargetVolumesDelta;
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

/// 🧾️ What one selection leaf's attraction re-solve moves, in document order: per object its new pose (`None`
/// for an unmoved object), the ids of the objects that only FOLLOWED, and the re-derived connection parameters of
/// every attraction whose parameters change.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle3dSelectionFollow {
    pub poses: Vec<Option<Puzzle3dPose>>,
    pub followers: Vec<String>,
    pub attractions: Vec<semio_s_artifact_puzzle_3d_diff::Puzzle3dAttractionModification>,
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
pub fn puzzle3d_selection_follow(base: &Puzzle3dSnapshot, targets: &[String], selection: &Puzzle3dSelection<'_>, pose: &dyn Fn(&semio_s_artifact_puzzle_3d_model::Puzzle3dObject) -> Puzzle3dPose, follow: bool) -> Puzzle3dSelectionFollow {
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dAttractionPatch;
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dAttractionModification;
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

//#region 🌳️Modules
#[path = "."]
pub mod drag_selection {
    #[path = "✋️drag-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✋️drag-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "✋️drag-selection/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rotate_selection {
    #[path = "🔄️rotate-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔄️rotate-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔄️rotate-selection/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod scale_selection {
    #[path = "🔍️scale-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔍️scale-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔍️scale-selection/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
