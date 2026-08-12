//! 🔺️ Playbook artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::playbook::schema::diff::{PlaybookBlockPatch, PlaybookBlocksDelta, PlaybookDiff, PlaybookStepPatch, PlaybookStepsDelta};
use crate::artifacts::playbook::schema::PlaybookArtifact;
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

//#endregion 🔖️Builders
