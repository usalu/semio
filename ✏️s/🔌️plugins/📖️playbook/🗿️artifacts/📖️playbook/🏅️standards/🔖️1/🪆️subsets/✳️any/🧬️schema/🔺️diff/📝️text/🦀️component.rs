//! 🔺️ Playbook artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::playbook::schema::diff::{
    PlaybookBlockPatch, PlaybookBlockPatchEntry, PlaybookBlocksDelta, PlaybookDiff, PlaybookStepPatch,
    PlaybookStepPatchEntry, PlaybookStepsDelta, PlaybookStringList,
};
use crate::artifacts::playbook::schema::PlaybookArtifact;
use crate::artifacts::playbook::schema::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::snapshot::PlaybookSnapshot;
use crate::playbook::PlaybookBlock;
use crate::playbook::PlaybookStep;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
pub fn apply_blocks_delta(items: &[PlaybookBlock], delta: &PlaybookBlocksDelta) -> Vec<PlaybookBlock> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|block| block.id != *id);
    }
    for block in &delta.added {
        next.push(block.clone());
    }
    for entry in &delta.patched {
        if let Some(block) = next.iter_mut().find(|block| block.id == entry.id) {
            apply_block_patch(block, &entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|block| (block.id.clone(), block)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(block) = by_id.remove(id) {
                ordered.push(block);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn apply_block_patch(block: &mut PlaybookBlock, patch: &PlaybookBlockPatch) {
    if let Some(replacement) = &patch.block {
        *block = replacement.clone();
    }
}

fn apply_step_patch(step: &mut PlaybookStep, patch: &PlaybookStepPatch) {
    if let Some(title) = &patch.title {
        step.title = title.clone();
    }
    if let Some(description) = &patch.description {
        step.description = description.clone();
    }
    if let Some(delta) = &patch.blocks {
        step.blocks = apply_blocks_delta(&step.blocks, delta);
    }
}

pub fn apply_steps_delta(items: &[PlaybookStep], delta: &PlaybookStepsDelta) -> Vec<PlaybookStep> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|step| step.id != *id);
    }
    for step in &delta.added {
        next.push(step.clone());
    }
    for entry in &delta.patched {
        if let Some(step) = next.iter_mut().find(|step| step.id == entry.id) {
            apply_step_patch(step, &entry.patch);
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

impl PlaybookDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &PlaybookArtifact) -> PlaybookArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
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

impl MutationDiff<PlaybookSnapshot> for PlaybookDiff {
    fn apply(&self, snapshot: &PlaybookSnapshot) -> PlaybookSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        take!(selected_ids);
        take!(locale);
        take!(contributions_json);
        if let Some(src) = other.steps {
            match &mut self.steps {
                Some(dst) => {
                    dst.added.extend(src.added);
                    dst.removed.extend(src.removed);
                    dst.patched.extend(src.patched);
                    if src.reordered.is_some() {
                        dst.reordered = src.reordered;
                    }
                }
                None => self.steps = Some(src),
            }
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff {
        artifact: Some(Box::new(PlaybookArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

/// 🎯️ Builds a sparse diff from a kernel `PlaybookMutation` against `base`.
pub fn playbook_diff_from_mutation(mutation: &PlaybookMutation, base: &PlaybookSnapshot) -> PlaybookDiff {
    use crate::artifacts::playbook::mutations::PlaybookMutation;
    match mutation {
        PlaybookMutation::AddStep { step, .. } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta { added: vec![step.clone()], ..Default::default() }),
            ..Default::default()
        },
        PlaybookMutation::RemoveStep { step_id } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta { removed: vec![step_id.clone()], ..Default::default() }),
            ..Default::default()
        },
        PlaybookMutation::MoveStep { step_id, index } => {
            let mut order: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
            if let Some(pos) = order.iter().position(|id| id == step_id) {
                let entry = order.remove(pos);
                let insert_at = (*index).min(order.len());
                order.insert(insert_at, entry);
            }
            PlaybookDiff {
                steps: Some(PlaybookStepsDelta { reordered: Some(order), ..Default::default() }),
                ..Default::default()
            }
        },
        PlaybookMutation::AddBlock { step_id, block, .. } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta {
                patched: vec![PlaybookStepPatchEntry {
                    id: step_id.clone(),
                    patch: PlaybookStepPatch {
                        blocks: Some(PlaybookBlocksDelta { added: vec![block.clone()], ..Default::default() }),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        PlaybookMutation::RemoveBlock { step_id, block_id } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta {
                patched: vec![PlaybookStepPatchEntry {
                    id: step_id.clone(),
                    patch: PlaybookStepPatch {
                        blocks: Some(PlaybookBlocksDelta { removed: vec![block_id.clone()], ..Default::default() }),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        PlaybookMutation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
            if from_step_id == to_step_id {
                let mut order: Vec<String> = base
                    .steps
                    .iter()
                    .find(|step| step.id == *from_step_id)
                    .map(|step| step.blocks.iter().map(|block| block.id.clone()).collect())
                    .unwrap_or_default();
                if let Some(pos) = order.iter().position(|id| id == block_id) {
                    order.remove(pos);
                }
                order.insert((*index).min(order.len()), block_id.clone());
                PlaybookDiff {
                    steps: Some(PlaybookStepsDelta {
                        patched: vec![PlaybookStepPatchEntry {
                            id: from_step_id.clone(),
                            patch: PlaybookStepPatch {
                                blocks: Some(PlaybookBlocksDelta { reordered: Some(order), ..Default::default() }),
                                ..Default::default()
                            },
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            } else {
                let next = PlaybookSnapshot::from_kernel(crate::playbook::apply_playbook_edit_mutation(&base.as_kernel(), mutation));
                PlaybookDiff {
                    artifact: Some(Box::new(PlaybookArtifact::from_snapshot(next))),
                    ..Default::default()
                }
            }
        },
        PlaybookMutation::UpdateBlock { step_id, block } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta {
                patched: vec![PlaybookStepPatchEntry {
                    id: step_id.clone(),
                    patch: PlaybookStepPatch {
                        blocks: Some(PlaybookBlocksDelta {
                            patched: vec![PlaybookBlockPatchEntry {
                                id: block.id.clone(),
                                patch: PlaybookBlockPatch { block: Some(block.clone()) },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        PlaybookMutation::UpdateStep { step } => PlaybookDiff {
            steps: Some(PlaybookStepsDelta {
                patched: vec![PlaybookStepPatchEntry {
                    id: step.id.clone(),
                    patch: PlaybookStepPatch {
                        title: Some(step.title.clone()),
                        description: Some(step.description.clone()),
                        blocks: None,
                    },
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        PlaybookMutation::UpdatePlaybook { title } => PlaybookDiff {
            title: Some(title.clone()),
            ..Default::default()
        },
    }
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::{mutations::PlaybookMutation, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};

    #[test]
    fn add_step_diff_applies_onto_the_base_snapshot() {
        let base = PlaybookSnapshot {
            schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
            id: "playbook".into(),
            version: "1".into(),
            title: None,
            steps: Vec::new(),
        };
        let step = PlaybookStep { id: "s".into(), title: "Basics".into(), description: None, blocks: Vec::new() };
        let operation = PlaybookMutation::AddStep { step, index: None };
        let diff = playbook_diff_from_mutation(&operation, &base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
