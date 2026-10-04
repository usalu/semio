//! ⚙️ Procedure path operations over a [`Path`] — the scope addressing (`PathRef`) every program edit uses before its
//! flow child leaves are derived (`crate::procedure_flow_leaves`).

use crate::{Path, PathRef, Step};

/// 🔎️ The step list a `PathRef` addresses in `path` (an absent nested slot reads as empty).
pub fn resolve_steps_in_path(path: &Path, path_ref: &PathRef) -> Vec<Step> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return path.steps.clone();
    }
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return Vec::new() };
    let Some(owner_step) = path.steps.iter().find(|step| &step.id == owner) else { return Vec::new() };
    owner_step.bodies.get(slot).map(|body| body.steps.clone()).unwrap_or_default()
}

/// 🔧 Resolves the MUTABLE step list at `path_ref` within a `Path` — the edit-side counterpart of
/// [`resolve_steps_in_path`]; a nested slot that does not exist yet is created.
pub fn resolve_path_mut<'a>(path: &'a mut Path, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹 Removes a now-empty nested body slot after a delete — mirrors the pre-migration snapshot-
/// level `prune_empty_slot` this same helper set used to own.
pub fn prune_empty_slot(path: &mut Path, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return };
    if let Some(owner_step) = path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|body| body.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}

//#region 🔖️ProgramEdits
/// 📍️ The scope `owner`/`slot` command fields address: a top-level control step's body slot when both name a real step,
/// else the root scope (an unknown reference addresses nothing else).
pub fn path_ref_in(path: &Path, owner: Option<&str>, slot: Option<&str>) -> PathRef {
    match (owner, slot) {
        (Some(owner), Some(slot)) if path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.into()), slot: Some(slot.into()) },
        _ => PathRef::default(),
    }
}

/// 🆔️ A fresh `step-N` id one past the highest suffix anywhere in the program (nested bodies included).
pub fn next_step_id(path: &Path) -> String {
    fn max_suffix(steps: &[Step]) -> u64 {
        steps.iter().fold(0, |acc, step| {
            let own = step.id.strip_prefix("step-").and_then(|rest| rest.parse::<u64>().ok()).unwrap_or(0);
            acc.max(own).max(step.bodies.values().map(|body| max_suffix(&body.steps)).max().unwrap_or(0))
        })
    }
    format!("step-{}", max_suffix(&path.steps) + 1)
}

/// ➕️ Inserts `step` into the addressed scope at `index` (clamped; absent appends); an id the scope already holds is
/// refused (no edit).
pub fn insert_step(path: &mut Path, path_ref: &PathRef, index: Option<usize>, step: Step) {
    if resolve_steps_in_path(path, path_ref).iter().any(|existing| existing.id == step.id) {
        return;
    }
    if let Some(steps) = resolve_path_mut(path, path_ref) {
        steps.insert(index.map_or(steps.len(), |index| index.min(steps.len())), step);
    }
}

/// ➖️ Removes step `id` (with its nested bodies) from the addressed scope; an emptied body slot is pruned.
pub fn remove_step(path: &mut Path, path_ref: &PathRef, id: &str) {
    if let Some(steps) = resolve_path_mut(path, path_ref) {
        steps.retain(|step| step.id != id);
    }
    prune_empty_slot(path, path_ref);
}

/// 🚚️ Moves step `id` to `to_index` (clamped) within the addressed scope.
pub fn move_step(path: &mut Path, path_ref: &PathRef, id: &str, to_index: usize) {
    if let Some(steps) = resolve_path_mut(path, path_ref) {
        if let Some(from) = steps.iter().position(|step| step.id == id) {
            let step = steps.remove(from);
            steps.insert(to_index.min(steps.len()), step);
        }
    }
    prune_empty_slot(path, path_ref);
}

/// 🎚️ Replaces the params of step `id` in the addressed scope (the displaced dictionary is retired, never dropped).
pub fn set_step_params(path: &mut Path, path_ref: &PathRef, id: &str, params: crate::Dictionary) {
    match resolve_path_mut(path, path_ref).and_then(|steps| steps.iter_mut().find(|step| step.id == id)) {
        Some(step) => neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut step.params, params)),
        None => neural_engine::ColdRetire::retire_cold(params),
    }
    prune_empty_slot(path, path_ref);
}
//#endregion 🔖️ProgramEdits

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
