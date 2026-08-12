//! 🔺️ Imperative artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::imperative::schema::diff::{ImperativeDiff, ImperativePathDelta, ImperativeStepsDelta, ImperativeStringList};
use crate::artifacts::imperative::schema::ImperativeArtifact;
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, PathRef, Step};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::imperative::schema::diff::*;


//#region 🔖️Apply
fn apply_steps_delta(items: &[Step], delta: &ImperativeStepsDelta) -> Vec<Step> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|step| step.id != *id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(step) = next.iter_mut().find(|step| step.id == entry.id) {
            step.params = entry.patch.clone();
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|step| (step.id.clone(), step)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(step) = by_id.remove(id) {
                ordered.push(step);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn resolve_steps_mut<'a>(snapshot: &'a mut ImperativeSnapshot, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut snapshot.path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = snapshot.path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

fn prune_empty_slot(snapshot: &mut ImperativeSnapshot, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else {
        return;
    };
    if let Some(owner_step) = snapshot.path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|path| path.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}

fn apply_path_delta(snapshot: &mut ImperativeSnapshot, delta: &ImperativePathDelta) {
    if let Some(steps) = resolve_steps_mut(snapshot, &delta.path_ref) {
        *steps = apply_steps_delta(steps, &delta.steps);
    }
    prune_empty_slot(snapshot, &delta.path_ref);
}

impl ImperativeDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &ImperativeArtifact) -> ImperativeArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.path {
            let mut snapshot = next.to_snapshot();
            apply_path_delta(&mut snapshot, delta);
            next.set_snapshot(snapshot);
        }
        if let Some(seed) = &self.seed {
            next.seed = seed.clone();
        }
        if let Some(list) = &self.selected_step_ids {
            next.selected_step_ids = list.values.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        next
    }
}

fn absorb_steps_delta(target: &mut Option<ImperativeStepsDelta>, incoming: Option<ImperativeStepsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_path_delta(target: &mut Option<ImperativePathDelta>, incoming: Option<ImperativePathDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) if dst.path_ref == src.path_ref => {
                dst.steps.added.extend(src.steps.added);
                dst.steps.removed.extend(src.steps.removed);
                dst.steps.patched.extend(src.steps.patched);
                if src.steps.reordered.is_some() {
                    dst.steps.reordered = src.steps.reordered;
                }
            }
            _ => *target = Some(src),
        }
    }
}

impl MutationDiff<ImperativeSnapshot> for ImperativeDiff {
    fn apply(&self, snapshot: &ImperativeSnapshot) -> ImperativeSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.path {
            apply_path_delta(&mut next, delta);
        }
        if let Some(seed) = &self.seed {
            next.seed = seed.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_path_delta(&mut self.path, other.path);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(seed);
        take!(selected_step_ids);
        take!(locale);
        take!(contributions_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: ImperativeSnapshot) -> ImperativeDiff {
    ImperativeDiff {
        artifact: Some(Box::new(ImperativeArtifact::from_snapshot(snapshot))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::engine::default_snapshot;
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn imperative_diff_absorb_whole_artifact_wins() {
        let mut diff = ImperativeDiff {
            path: Some(ImperativePathDelta {
                path_ref: PathRef::default(),
                steps: ImperativeStepsDelta { removed: vec!["step-1".into()], ..Default::default() },
            }),
            ..Default::default()
        };
        let replacement = ImperativeDiff {
            artifact: Some(Box::new(ImperativeArtifact::default())),
            ..Default::default()
        };
        diff.absorb(replacement);
        assert!(diff.artifact.is_some());
        assert!(diff.path.is_none());
    }

    #[test]
    fn path_delta_remove_round_trips_via_apply() {
        let base = default_snapshot();
        let diff = ImperativeDiff {
            path: Some(ImperativePathDelta {
                path_ref: PathRef::default(),
                steps: ImperativeStepsDelta { removed: vec!["step-1".into()], ..Default::default() },
            }),
            ..Default::default()
        };
        let next = diff.apply(&base);
        assert!(next.path.steps.iter().all(|step| step.id != "step-1"));
    }
}
//#endregion 🧪️Tests
